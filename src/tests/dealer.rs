use crate::actors::dealer::{Dealer, DealerMsg};
use crate::deck_handler::DeckHandler;
use apples_utils::{config::Config, consts::CONFIG_TOML};
use core::num::NonZeroUsize;

#[tokio::test]
async fn deal_cards_success() -> anyhow::Result<()> {
    let mut handler = DeckHandler::new();
    let config = Config::parse_config(CONFIG_TOML.into())?;
    handler
        .load_decks(
            config.red_deck_path().into(),
            config.green_deck_path().into(),
        )
        .await?;

    handler.shuffle();

    let (dealer, _) = ractor::Actor::spawn(None, Dealer, handler).await?;

    // Test dealing red cards
    let amount_red_cards = NonZeroUsize::new(7).expect("failed to create ");
    let red_cards = ractor::call!(dealer, DealerMsg::DealRedCards, amount_red_cards)??;

    assert_eq!(
        amount_red_cards.get(),
        red_cards.len(),
        "The dealt red card amount does not match the expected amount"
    );

    let green_card_amount = NonZeroUsize::MIN;
    let green_cards_result = ractor::call!(dealer, DealerMsg::DealGreenCards, green_card_amount)?;
    let green_cards = green_cards_result.expect("Should successfully deal green cards");

    assert_eq!(
        green_cards.len(),
        green_card_amount.get(),
        "The amount of green cards does not match the expected amount"
    );

    // Test deck sizes
    let (red_size, green_size) = ractor::call!(dealer, DealerMsg::GetDeckSizes)?;
    println!("Remaining cards - Red: {}, Green: {}", red_size, green_size);

    Ok(())
}

#[tokio::test]
async fn deal_cards_insufficient() -> anyhow::Result<()> {
    let handler = DeckHandler::new();
    let (dealer, _) = ractor::Actor::spawn(None, Dealer, handler).await?;

    let card = NonZeroUsize::new(5).expect("Failed to create the amount of cards");

    let result = ractor::call!(dealer, DealerMsg::DealRedCards, card)?;

    assert!(result.is_err(), "Should fail when dealing from empty deck");

    match result {
        Err(crate::actors::dealer::DealError::OutOfCards {
            requested,
            available,
        }) => {
            assert_eq!(requested, 5);
            assert_eq!(available, 0);
        }
        _ => panic!("Wrong error type"),
    }

    Ok(())
}
