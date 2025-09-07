/*use crate::actors::score_manager::{ScoreManager, ScoreMessage, ScoreResult};
use apples_utils::{config::Config, consts::CONFIG_TOML};

#[tokio::test]
async fn score_handler() -> anyhow::Result<()> {
    let config = Config::parse_config(CONFIG_TOML.into())?;
    let players = 2;
    let win_condition = config
        .get_required_apples(players)
        .expect("failed to get win condition fix config file");

    let (score_handler, _) = ractor::Actor::spawn(None, ScoreManager, win_condition).await?;
    let (id, points) = (0, 1);
    let updated_result =
        ractor::call!(score_handler, ScoreMessage::AddScoreCheckWinner, id, points)?;
    assert!(
        updated_result == ScoreResult::Continue,
        "Updated score should not be enough to win the game"
    );

    let id = 0;
    let score = ractor::call!(score_handler, ScoreMessage::GetScore, id)?;
    assert!(
        score == points,
        "Score should be equal to {} to win the game",
        points
    );

    let (id, points) = (17, win_condition);
    let updated_result =
        ractor::call!(score_handler, ScoreMessage::AddScoreCheckWinner, id, points)?;
    assert!(
        updated_result == ScoreResult::Win,
        "Updated score should be enough to win the game"
    );
    Ok(())
}*/
