use super::connection::ConnectionMsg;
use apples_core::protocol::message::GameMessage;
use apples_protocol::reader::MessageReader;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tokio::net::tcp::OwnedReadHalf;

pub struct Reader;

#[ractor::async_trait]
impl Actor for Reader {
    actor_types!((), (), (OwnedReadHalf, ActorRef<ConnectionMsg>));
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<(), ActorProcessingErr> {
        let mut reader = MessageReader::new(args.0);
        let connection = args.1;
        tokio::spawn(async move {
            while let Some(Ok(msg)) = reader.next_message::<GameMessage>().await {
                let _ = ractor::cast!(connection, ConnectionMsg::Receive(msg));
            }
        });
        Ok(())
    }
}
