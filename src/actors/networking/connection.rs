use super::reader::Reader;
use super::writer::{Writer, WriterMsg};
use crate::actors::client_fsm::ClientStates;
use crate::actors::networking::registry::RegistryMsg;
use apples_core::protocol::message::GameMessage;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
use tokio::net::TcpStream;

pub enum InboundTarget {
    Registry {
        registry: ActorRef<RegistryMsg>,
        conn_id: usize,
    },
    Client {
        fsm: ActorRef<ClientStates>,
    },
}

pub struct Connection;

pub struct ConnectionState {
    writer: ActorRef<WriterMsg>,
    target: InboundTarget,
}

#[derive(RactorMessage)]
pub enum ConnectionMsg {
    Send(GameMessage),
    Receive(GameMessage),
    Stop,
}

#[ractor::async_trait]
impl Actor for Connection {
    actor_types!(ConnectionMsg, ConnectionState, (TcpStream, InboundTarget));

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let stream = args.0;
        let (reader, writer) = stream.into_split();
        let (writer, _) = ractor::Actor::spawn(None, Writer, writer).await?;
        let (_reader, _) = ractor::Actor::spawn(None, Reader, (reader, myself)).await?;
        Ok(ConnectionState {
            writer,
            target: args.1,
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
                ractor::cast!(state.writer, WriterMsg::Send(msg))?;
            }
            ConnectionMsg::Stop => {
                myself.stop(None);
            }
            ConnectionMsg::Receive(msg) => match &state.target {
                InboundTarget::Registry { registry, conn_id } => {
                    ractor::cast!(registry, RegistryMsg::Incomming(*conn_id, msg))?;
                }
                InboundTarget::Client { fsm } => {
                    ractor::cast!(fsm, ClientStates::Judge)?;
                }
            },
        }
        Ok(())
    }
}
