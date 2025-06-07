use {
    ahash::AHashMap,
    apples_utils::actor_types,
    ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort},
    ractor_cluster::RactorMessage,
};

#[derive(RactorMessage, PartialEq, Hash)]
pub enum ScoreResult {
    Continue,
    Win,
}
pub struct ScoreManager;

pub struct ManagerState {
    scores: AHashMap<usize, usize>,
    win_condition: usize,
}

#[derive(RactorMessage)]
pub enum ScoreMessage {
    AddScoreCheckWinner(usize, usize, RpcReplyPort<ScoreResult>),
    GetScore(usize, RpcReplyPort<usize>),
}

///Alias for score tx
pub type ScoreTx = ActorRef<ScoreMessage>;

#[ractor::async_trait]
impl Actor for ScoreManager {
    actor_types!(ScoreMessage, ManagerState, usize);

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(ManagerState {
            scores: AHashMap::new(),
            win_condition: args,
        })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            Self::Msg::GetScore(id, reply) => {
                let score = state.scores.entry(id).or_insert(0);
                let score = *score;
                reply.send(score)?;
            }
            Self::Msg::AddScoreCheckWinner(id, add, reply) => {
                let entry = state.scores.entry(id).or_insert(0);
                *entry += add;
                if *entry >= state.win_condition {
                    reply.send(ScoreResult::Win)?;
                } else {
                    reply.send(ScoreResult::Continue)?;
                }
            }
        }
        Ok(())
    }
}
