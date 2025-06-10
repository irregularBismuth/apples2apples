use super::super::client_fsm::ClientStates;
use super::reader::Reader;
use super::writer::{Writer, WriterMsg};
use apples_core::protocol::message::GameMessage;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
pub struct ClientConnectionState {
    writer: ActorRef<WriterMsg>,
    fsm: ActorRef<ClientStates>,
}

#[derive(RactorMessage)]
pub enum ClientConnectionMsg {
    Send(GameMessage),
    Receive(GameMessage),
    Stop,
}
use tokio::net::TcpStream;
pub struct ClientConnection;
#[ractor::async_trait]
impl Actor for ClientConnection {
    actor_types!(
        ClientConnectionMsg,
        ClientConnectionState,
        (TcpStream, ActorRef<ClientStates>)
    );

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        (stream, fsm): Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (r, w) = stream.into_split();
        let (writer, _) = Actor::spawn(None, Writer, w).await?;
        let (_reader, _) = Actor::spawn(None, Reader, (r, myself)).await?;
        Ok(ClientConnectionState { writer, fsm })
    }

    async fn handle(
        &self,
        _me: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            ClientConnectionMsg::Send(m) => cast!(state.writer, WriterMsg::Send(m))?,
            ClientConnectionMsg::Receive(m) => cast!(state.fsm, ClientFsmMsg::Incoming(m))?,
            ClientConnectionMsg::Stop => _me.stop(None),
        }
        Ok(())
    }
}
