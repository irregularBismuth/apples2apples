use ::ractor::{ActorProcessingErr, RpcReplyPort};
use actor_macros::{actor, actor_handle, actor_pre_start};
use ahash::AHashMap as Map;
use apples_core::player::player::PlayerId;
use core::num::NonZeroUsize;
use thiserror::Error;

pub type ScoreResult = std::result::Result<Score, Error>;

#[derive(Debug)]
pub enum ScoreManagerMsg {
    UpdateScore(PlayerId, Score),
    RetrieveScore(PlayerId, RpcReplyPort<ScoreResult>),
}

#[actor(msg=ScoreManagerMsg, state=ScoreState, pre_start=on_start)]
pub(crate) struct ScoreManager;

#[derive(Debug)]
pub struct Score(usize);

struct ScoreState {
    scores: Map<PlayerId, Score>,
}

impl ScoreState {
    pub fn new() -> Self {
        Self { scores: Map::new() }
    }
}

impl ScoreManager {
    actor_pre_start!(Ok(ScoreState::new()));

    actor_handle!({
        match msg {}
        Ok(())
    });
}
