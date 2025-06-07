use apples_core::protocol::message::GameMessage;
use apples_protocol::framed_transport::FramedTransport;
use apples_protocol::protocol::Protocol;
use apples_protocol::tcp_transport::TcpTransport;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
pub struct Connection;

pub struct ConnectionState {
    transport: Arc<Mutex<Protocol<TcpTransport>>>,
}
#[derive(RactorMessage)]
pub enum ConnectionMsg {
    Send(GameMessage),
    Stop,
}

#[ractor::async_trait]
impl Actor for Connection {
    actor_types!(ConnectionMsg, ConnectionState, TcpStream);

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let stream = args;
        let (read, write) = stream.into_split();
        let transport = TcpTransport::new(read, write);
        let protocol = Protocol::new(transport);
        let protocol = Arc::new(Mutex::new(protocol));
        tokio::task::spawn({
            let protocol = protocol.clone();
            async move {
                loop {
                    let message_result: Option<anyhow::Result<GameMessage>> = {
                        let mut p = protocol.lock().await;
                        p.next_message().await
                    };

                    match message_result {
                        Some(Ok(message)) => {}
                        Some(Err(e)) => {}
                        None => {
                            println!("Player disconnected ");
                            let _ = ractor::cast!(myself, ConnectionMsg::Stop);
                            break;
                        }
                    }
                }
            }
        });
        Ok(ConnectionState {
            transport: protocol,
        })
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            ConnectionMsg::Send(msg) => {
                let mut protocol = state.transport.lock().await;
                if let Err(e) = protocol.send_message(&msg).await {
                    eprintln!("Failed to send to player");
                }
            }
            ConnectionMsg::Stop => {
                myself.stop(None);
            }
        }
        Ok(())
    }
}
