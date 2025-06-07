use apples_core::protocol::message::GameMessage;
use apples_protocol::framed_transport::FramedTransport;
use apples_protocol::protocol::Protocol;
use apples_protocol::tcp_transport::TcpTransport;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
use tokio::net::TcpStream;
pub struct Connection;

pub struct ConnectionState {
    transport: Protocol<TcpTransport>,
    id: usize,
}
#[derive(RactorMessage)]
pub enum ConnectionMsg {
    Send(GameMessage),
}

#[ractor::async_trait]
impl Actor for Connection {
    actor_types!(ConnectionMsg, ConnectionState, (usize, TcpStream));

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (id, stream) = args;
        let (read, write) = stream.into_split();
        let transport = TcpTransport::new(read, write);
        let protocol = Protocol::new(transport);
        Ok(ConnectionState {
            transport: protocol,
            id,
        })
    }
}
