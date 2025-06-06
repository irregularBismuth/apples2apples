use anyhow::Result;
use apples_core::cards::red_card::RedCard;
use apples_utils::config::Config;
#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse_config("Config.toml".into())?;
    println!("Hello {}", config.red_deck_path());
    let redcard = RedCard::new("abc".to_string(), "bcd".to_string(), 130);
    println!("Hello, world! {}", redcard);
    Ok(())
}
