use {
    anyhow::Result,
    apples2apples::{client_main::client_main, host_main::host_main},
    apples_utils::cli::{parse_args, Mode},
};

#[tokio::main]
async fn main() -> Result<()> {
    let mode = parse_args();
    match mode {
        Mode::Host { players, bots } => host_main(players, bots).await?,
        Mode::Client { ip } => client_main(ip.parse()?).await?,
    }
    Ok(())
}
