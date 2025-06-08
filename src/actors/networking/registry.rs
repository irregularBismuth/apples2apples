use super::connection::ConnectionMsg;
use ahash::AHashMap;
use apples_core::protocol::message::GameMessage;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;

#[derive(RactorMessage)]
pub enum RegistryMsg {
    AddClient(usize, ActorRef<ConnectionMsg>),
    Broadcast(GameMessage),
    Unicast(usize, GameMessage),
}

pub struct ConnectionRegistry;

pub struct RegistryState {
    clients: AHashMap<usize, ActorRef<ConnectionMsg>>,
}

impl RegistryState {
    pub fn new() -> RegistryState {
        Self {
            clients: AHashMap::new(),
        }
    }
}

#[ractor::async_trait]
impl Actor for ConnectionRegistry {
    actor_types!(RegistryMsg, RegistryState, ());

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(RegistryState::new())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            RegistryMsg::AddClient(id, conn) => {
                state.clients.insert(id, conn);
                ractor::cast!(
                    myself,
                    RegistryMsg::Unicast(
                        id,
                        GameMessage::RequestJudgeChoice(
                            Vec::new(),
                            apples_core::cards::green_card::GreenCard::new(
                                "0".to_string(),
                                "1".to_string(),
                                1
                            )
                        )
                    )
                )?;
            }
            RegistryMsg::Broadcast(msg) => {}
            RegistryMsg::Unicast(id, msg) => {
                if let Some(client) = state.clients.get(&id) {
                    ractor::cast!(client, ConnectionMsg::Send(msg))?;
                }
            }
        }
        Ok(())
    }
}
