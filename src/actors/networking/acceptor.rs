use super::connection::Connection;
use super::registry::RegistryMsg;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tokio::net::TcpListener;
/// Connection Acceptor
pub struct Acceptor;

#[ractor::async_trait]
impl Actor for Acceptor {
    actor_types!((), (), (TcpListener, ActorRef<RegistryMsg>));
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<(), ActorProcessingErr> {
        let listener = args.0;
        let registry = args.1;
        tokio::spawn(async move {
            let mut id = 0;

            loop {
                let local = registry.clone();
                let (stream, _) = listener.accept().await.unwrap();
                let conn_id = id;
                id += 1;
                let (conn, _) = ractor::Actor::spawn(
                    None,
                    Connection,
                    (
                        stream,
                        super::connection::InboundTarget::Registry {
                            registry: local,
                            conn_id,
                        },
                    ),
                )
                .await
                .expect("failed to spawn connection actor");

                if let Err(err) = ractor::cast!(registry, RegistryMsg::AddClient(conn_id, conn)) {
                    eprintln!("error {}", err);
                }
            }
        });
        Ok(())
    }
}
