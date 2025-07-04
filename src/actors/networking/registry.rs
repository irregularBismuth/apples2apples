use super::connection::ConnectionMsg;
use crate::actors::player_manager::PlayerMsg;
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
    Incomming(usize, GameMessage),
}

pub struct ConnectionRegistry;

use crate::actors::host_fsm::HostMsg;
pub struct RegistryState {
    clients: AHashMap<usize, ActorRef<ConnectionMsg>>,
    player_manager: ActorRef<PlayerMsg>,
    host_fsm: ActorRef<HostMsg>,
}

impl RegistryState {
    pub fn new(player_manager: ActorRef<PlayerMsg>, fsm: ActorRef<HostMsg>) -> RegistryState {
        Self {
            clients: AHashMap::new(),
            player_manager,
            host_fsm: fsm,
        }
    }
}
use std::mem;

#[ractor::async_trait]
impl Actor for ConnectionRegistry {
    actor_types!(
        RegistryMsg,
        RegistryState,
        (ActorRef<PlayerMsg>, ActorRef<HostMsg>)
    );

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(RegistryState::new(args.0, args.1))
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
                ractor::cast!(myself, RegistryMsg::Unicast(id, GameMessage::AssignId(id)))?;
                ractor::cast!(
                    state.player_manager,
                    PlayerMsg::AddPlayer(apples_core::player::player::PlayerId(id))
                )?;
            }
            RegistryMsg::Broadcast(msg) => {
                for (id, _) in state.clients.iter() {
                    let msg = msg.clone();
                    ractor::cast!(myself, RegistryMsg::Unicast(*id, msg))?;
                }
            }
            RegistryMsg::Unicast(id, msg) => {
                if let Some(client) = state.clients.get(&id) {
                    ractor::cast!(client, ConnectionMsg::Send(msg))?;
                }
            }
            RegistryMsg::Incomming(id, msg) => {
                ractor::cast!(state.host_fsm, HostMsg::PlayerAction(id, msg))?;
            }
        }
        Ok(())
    }
}
