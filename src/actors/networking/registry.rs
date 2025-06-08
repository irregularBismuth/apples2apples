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
    reg_type: RegistryType,
}

impl RegistryState {
    pub fn new(reg_type: RegistryType) -> RegistryState {
        Self {
            clients: AHashMap::new(),
            reg_type,
        }
    }
}
#[derive(PartialEq)]
pub enum RegistryType {
    Host,
    Client,
}

#[ractor::async_trait]
impl Actor for ConnectionRegistry {
    actor_types!(RegistryMsg, RegistryState, RegistryType);

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(RegistryState::new(args))
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
                if state.reg_type == RegistryType::Host {
                    ractor::cast!(myself, RegistryMsg::Unicast(id, GameMessage::AssignId(id)))?;
                }
            }
            RegistryMsg::Broadcast(msg) => {
                for (id, _) in state.clients.iter() {
                    let msg = msg.clone();
                    ractor::cast!(myself, RegistryMsg::Unicast(*id, msg));
                }
            }
            RegistryMsg::Unicast(id, msg) => {
                if let Some(client) = state.clients.get(&id) {
                    ractor::cast!(client, ConnectionMsg::Send(msg))?;
                }
            }
        }
        Ok(())
    }
}
