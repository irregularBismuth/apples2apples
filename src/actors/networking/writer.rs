use apples_core::protocol::message::GameMessage;
use apples_protocol::writer::MessageWriter;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
use tokio::net::tcp::OwnedWriteHalf;
#[derive(RactorMessage)]
pub enum WriterMsg {
    Send(GameMessage),
}

pub struct Writer;

pub struct WriterState {
    writer: MessageWriter<OwnedWriteHalf>,
}

#[ractor::async_trait]
impl Actor for Writer {
    actor_types!(WriterMsg, WriterState, OwnedWriteHalf);

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<WriterState, ActorProcessingErr> {
        let writer = MessageWriter::new(args);
        Ok(WriterState { writer })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            WriterMsg::Send(msg) => {
                state.writer.send_message(&msg).await?;
            }
        }
        Ok(())
    }
}
