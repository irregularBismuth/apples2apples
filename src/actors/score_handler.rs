use ::ractor::{ActorProcessingErr, RpcReplyPort};
use ahash::AHashMap as Map;
use anyhow::Result;
use apples_core::player::player::PlayerId;
use core::num::NonZeroUsize;
use dsl_ractor::{actor, actor_handle, actor_pre_start};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ScoreError {
    #[error("Player not found {player_id:?}")]
    PlayerNotFound { player_id: PlayerId },
}

pub type ScoreResult = Result<Score, ScoreError>;

#[derive(Debug)]
pub enum ScoreManagerMsg {
    UpdateScore(PlayerId, Score),
    RetrieveScore(PlayerId, RpcReplyPort<ScoreResult>),
}

#[actor(msg=ScoreManagerMsg, state=ScoreState)]
pub(crate) struct ScoreManager;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Score(pub usize);

pub(crate) struct ScoreState {
    scores: Map<PlayerId, Score>,
}

impl ScoreState {
    pub(crate) fn new() -> Self {
        Self { scores: Map::new() }
    }
}

impl ScoreManager {
    actor_pre_start!(Ok(ScoreState::new()));

    actor_handle!({
        match msg {
            ScoreManagerMsg::UpdateScore(id, new_score) => {
                let _ = state.scores.insert(id, new_score);
                tracing::info!("Updating score for {}", id);
            }
            ScoreManagerMsg::RetrieveScore(id, reply) => {
                let Some(score) = state.scores.get(&id).copied() else {
                    let _ = reply.send(Err(ScoreError::PlayerNotFound{ player_id: id}));
                    return Ok(());
                };
                let _ = reply.send(Ok(score));
            }
        }
        Ok(())
    });
}
