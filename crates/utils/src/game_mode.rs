use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameMode {
    Original,
    ApplesEyeView,
    BadHarvest,
    TwoForOne,
}
