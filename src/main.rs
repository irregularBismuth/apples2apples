use anyhow::Result;
use apples2apples::{client_main::client_main, host_main::host_main};
use apples_core::cards::red_card::RedCard;
use apples_utils::cli::{parse_args, Mode};
use apples_utils::config::Config;
#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse_config("Config.toml".into())?;
    let player = config.get_required_apples(4);
    if let Some(player) = player {
        println!("amount of green apples requierd is {}", player);
    }
    let mode = parse_args();
    match mode {
        Mode::Host { players, bots } => host_main(players, bots).await?,
        Mode::Client { ip } => client_main(ip.parse()?).await?,
    }
    Ok(())
}
