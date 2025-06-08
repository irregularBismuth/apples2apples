use super::reader::Reader;
use super::writer::{Writer, WriterMsg};
use apples_core::protocol::message::GameMessage;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

pub struct Connection;

pub struct ConnectionState {
    writer: ActorRef<WriterMsg>,
}
#[derive(RactorMessage)]
pub enum ConnectionMsg {
    Send(GameMessage),
    Receive(GameMessage),
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
        let (reader, writer) = stream.into_split();
        let (writer, _) = ractor::Actor::spawn(None, Writer, writer).await?;
        let (reader, _) = ractor::Actor::spawn(None, Reader, (reader, myself)).await?;
        Ok(ConnectionState { writer })
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
            ConnectionMsg::Receive(msg) => {
                println!("connection msg receive {:?}", msg);
            }
        }
        Ok(())
    }
}
