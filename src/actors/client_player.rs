use apples_core::GreenCard;
use dsl_ractor::{actor, actor_handle, actor_pre_start};
use ractor::RpcReplyPort;

pub(crate) enum PlayerMsg {
    ChooseCard,
    SendCard(GreenCard),
}

#[actor(msg=PlayerMsg,state=(),args=usize)]
pub(crate) struct ClientPlayer;

impl ClientPlayer {
    actor_pre_start!(Ok(()));

    actor_handle!({
        match msg {
            PlayerMsg::ChooseCard => {
                let card = todo!();
                ractor::cast!(myself, PlayerMsg::SendCard(card))?;
            }
            PlayerMsg::SendCard(card) => {
                todo!("add connection here ");
            }
        }

        Ok(())
    });
}
