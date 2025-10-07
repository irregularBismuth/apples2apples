use {
    anyhow::Result,
    apples_core::{cards::card::Card, RedCard},
    apples_protocol::{Frame, FrameCodec, Kind},
    bytes::Bytes,
    futures_util::{stream::SplitSink, stream::SplitStream, SinkExt, StreamExt},
    ractor::{async_trait, Actor, ActorProcessingErr, ActorRef, MessagingErr},
    serde_json::{from_slice, to_vec},
    std::time::Duration,
    tokio::{net::{TcpListener, TcpStream}, task::JoinHandle, time::sleep},
    tokio_util::codec::Framed,
};

#[derive(Debug)]
enum WriterMsg {
    Frame(Frame),
    Close,
}

struct WriterState {
    sink: SplitSink<Framed<TcpStream, FrameCodec>, Frame>,
}

struct WriterActor;

#[async_trait]
impl Actor for WriterActor {
    type Msg = WriterMsg;
    type State = WriterState;
    type Arguments = WriterState;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(args)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            WriterMsg::Frame(frame) => {
                state
                    .sink
                    .send(frame)
                    .await
                    .map_err(|err| ActorProcessingErr::from(err.to_string()))?;
            }
            WriterMsg::Close => {
                state
                    .sink
                    .close()
                    .await
                    .map_err(|err| ActorProcessingErr::from(err.to_string()))?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum ClientEvent {
    Prompt(String),
    ConnectionClosed,
}

struct ReaderArgs {
    stream: SplitStream<Framed<TcpStream, FrameCodec>>,
    coordinator: ActorRef<ClientEvent>,
}

struct ReaderState {
    pump: JoinHandle<()>,
}

struct ReaderActor;

#[async_trait]
impl Actor for ReaderActor {
    type Msg = ();
    type State = ReaderState;
    type Arguments = ReaderArgs;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let ReaderArgs {
            mut stream,
            coordinator,
        } = args;

        let pump = tokio::spawn(async move {
            while let Some(frame) = stream.next().await {
                match frame {
                    Ok(frame) => match frame.header().kind() {
                        Kind::Game => {
                            let prompt = String::from_utf8_lossy(frame.payload()).to_string();
                            let _ = coordinator.cast(ClientEvent::Prompt(prompt));
                        }
                        other => eprintln!("client reader: unexpected frame kind {other:?}"),
                    },
                    Err(err) => {
                        eprintln!("client reader error: {err}");
                        break;
                    }
                }
            }
            let _ = coordinator.cast(ClientEvent::ConnectionClosed);
        });

        Ok(ReaderState { pump })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        _msg: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        Ok(())
    }

    async fn post_stop(
        &self,
        _myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        state.pump.abort();
        Ok(())
    }
}

struct ClientState {
    writer: ActorRef<WriterMsg>,
}

struct ClientCoordinator;

#[async_trait]
impl Actor for ClientCoordinator {
    type Msg = ClientEvent;
    type State = ClientState;
    type Arguments = ClientState;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(args)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            ClientEvent::Prompt(prompt) => {
                println!("client <- {prompt}");
                let card = RedCard::new(777usize, "Double Rainbow", "Seeing every color all at once");
                let payload = to_vec(&card)
                    .map_err(|err| ActorProcessingErr::from(err.to_string()))?;
                state
                    .writer
                    .cast(WriterMsg::Frame(Frame::new(Kind::Game, 0, Bytes::from(payload))))
                    .map_err(into_actor_err)?;
            }
            ClientEvent::ConnectionClosed => {
                println!("client: judge closed the connection");
                let _ = state.writer.cast(WriterMsg::Close);
                myself.stop(None);
            }
        }
        Ok(())
    }
}

fn into_actor_err(msg: MessagingErr<WriterMsg>) -> ActorProcessingErr {
    ActorProcessingErr::from(format!("failed to send writer message: {msg}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    const ADDR: &str = "127.0.0.1:6000";

    let server = tokio::spawn(async move {
        let listener = TcpListener::bind(ADDR).await?;
        let (socket, _addr) = listener.accept().await?;
        let mut framed = Framed::new(socket, FrameCodec::with_default_limit());

        let prompt = Frame::new(
            Kind::Game,
            0,
            Bytes::from_static(b"Judge: play your best red card"),
        );
        framed.send(prompt).await?;

        if let Some(frame) = framed.next().await.transpose()? {
            match frame.header().kind() {
                Kind::Game => {
                    let card: RedCard = from_slice(frame.payload())?;
                    println!(
                        "judge received => {} — {} (id: {})",
                        card.name(),
                        card.description(),
                        card.id().value()
                    );
                }
                other => println!("judge: unexpected frame kind {other:?}"),
            }
        }

        Ok::<_, anyhow::Error>(())
    });

    sleep(Duration::from_millis(50)).await;

    let stream = TcpStream::connect(ADDR).await?;
    let framed = Framed::new(stream, FrameCodec::with_default_limit());
    let (sink, stream) = framed.split();

    let (writer_ref, writer_handle) = WriterActor::spawn(None, WriterActor, WriterState { sink }).await?;
    let (coordinator_ref, coordinator_handle) = ClientCoordinator::spawn(
        None,
        ClientCoordinator,
        ClientState {
            writer: writer_ref.clone(),
        },
    )
    .await?;
    let (_reader_ref, reader_handle) = ReaderActor::spawn(
        None,
        ReaderActor,
        ReaderArgs {
            stream,
            coordinator: coordinator_ref,
        },
    )
    .await?;

    server.await??;
    let _ = reader_handle.await;
    let _ = writer_handle.await;
    let _ = coordinator_handle.await;

    Ok(())
}
