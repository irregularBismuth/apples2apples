use crate::deck_handler::DeckHandler;
use anyhow::Result;
use apples_utils::{config::Config, consts::CONFIG_TOML, game_mode::GameMode};
#[doc = include_str!("../doc/host.md")]
pub async fn host_main(players: usize, bots: usize) -> Result<()> {
    let config = Config::parse_config(CONFIG_TOML.into())?;
    match config.game_mode() {
        GameMode::Original => {
            println!("{}", config.get_required_apples(players + bots).unwrap());
            let mut deck_handler = DeckHandler::new();

            deck_handler
                .load_decks(
                    config.red_deck_path().into(),
                    config.green_deck_path().into(),
                )
                .await?;

            deck_handler.shuffle();

            if let Some(card) = deck_handler.get_green_card() {
                println!("{}", card);
            }
        }
        _ => {
            todo!("unsupported now, original is supported")
        }
    }
    Ok(())
}
