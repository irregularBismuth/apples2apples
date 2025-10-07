use serde::Deserialize;
/// The diffrent game modes for the apples2apples game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameMode {
    Original,
    ApplesEyeView,
    BadHarvest,
    TwoForOne,
}
