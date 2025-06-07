use ahash::AHashMap;
use apples_core::protocol::message::GameMessage;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::RactorMessage;
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;
#[derive(RactorMessage)]
pub enum ConnMsg {
    Send(GameMessage),
}

#[derive(RactorMessage)]
pub enum RegistryMsg {
    AddClient(usize, ActorRef<ConnMsg>),
}

pub struct ConnectionRegistry;

pub struct RegistryState {
    clients: AHashMap<usize, ActorRef<ConnMsg>>,
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
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
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
                if state.clients.len() < 2 {
                    state.clients.insert(id, conn);
                } else {
                    for (id, conn) in &state.clients {
                        ractor::cast!(conn, ConnMsg::Send(GameMessage::AssignId(*id)))?;
                    }
                }
            }
        }
        Ok(())
    }
}
