use super::registry::RegistryMsg;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tokio::net::TcpListener;
pub struct Acceptor;

#[ractor::async_trait]
impl Actor for Acceptor {
    actor_types!((), (), (TcpListener, ActorRef<RegistryMsg>));
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<(), ActorProcessingErr> {
        let listener = args.0;
        let registry = args.1;
        tokio::spawn(async move {
            let mut id = 0;
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let conn_id = id;
                id += 1;
                println!("id {}", id);
                //let (conn,_)= ractor::Actor::spawn(None, (),()).await.expect("");
                // ractor::cast!()
            }
        });
        Ok(())
    }
}
