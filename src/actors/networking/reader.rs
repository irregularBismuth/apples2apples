use apples_core::protocol::message::GameMessage;
use apples_protocol::reader::MessageReader;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
use tokio::net::tcp::OwnedReadHalf;
#[derive(RactorMessage)]
pub enum ReaderMsg {
    Incomming(GameMessage),
}

pub struct Reader;

pub struct ReaderState {}

#[ractor::async_trait]
impl Actor for Reader {
    actor_types!(ReaderMsg, (), (OwnedReadHalf));

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<(), ActorProcessingErr> {
        let mut reader = MessageReader::new(args);

        tokio::spawn(async move {
            let myself = myself.clone();
            while let Some(Ok(msg)) = reader.next_message::<GameMessage>().await {
                println!("{:?}", msg);
                let _ = ractor::cast!(myself, ReaderMsg::Incomming(msg));
            }
        });
        Ok(())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            ReaderMsg::Incomming(msg) => {
                println!("do somethinig");
            }
        }
        Ok(())
    }
}
