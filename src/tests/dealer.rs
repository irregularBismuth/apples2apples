use crate::actors::dealer::{DealerActor, DealerMsg};
use crate::deck_handler::DeckHandler;
use apples_utils::{config::Config, consts::CONFIG_TOML};

#[tokio::test]
async fn deal_cards() -> anyhow::Result<()> {
    let mut handler = DeckHandler::new();
    let config = Config::parse_config(CONFIG_TOML.into())?;
    handler
        .load_decks(
            config.red_deck_path().into(),
            config.green_deck_path().into(),
        )
        .await?;

    handler.shuffle();

    let (dealer, _) = ractor::Actor::spawn(None, DealerActor, handler).await?;
    let amount_red_cards = 7;
    let red_cards = ractor::call!(dealer, DealerMsg::DealRedCards, amount_red_cards)?;

    assert_eq!(
        amount_red_cards,
        red_cards.len(),
        "The dealt red card amount does not match the expected amount of redcards requested"
    );

    let green_card = 1;
    let green_cards = ractor::call!(dealer, DealerMsg::DealGreenCards, green_card)?;
    assert_eq!(
        green_cards.len(),
        green_card,
        "The amount of green cards does not match the expected amount"
    );

    Ok(())
}
