use crate::cards::{green_card::GreenCard, red_card::RedCard};

pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GameMessage {
    AssignId(usize),
    DealRedCard(RedCard),
    DealGreenCard(GreenCard),
    RequestRedCardChoice(GreenCard),
    RequestJudgeChoice(Vec<RedCard>, GreenCard),
    RedCardPlayed(usize, RedCard),
    JudgeVoted(usize),
    WinnerAnnouncement(usize, GreenCard),
    CurrentScore(usize),
    MaxScore(usize),
    GameEnd,
}
