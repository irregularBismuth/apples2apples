use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "Apples Game",
    about = "Host or join a game of Apples to Apples"
)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["ip", "players"])
))]
pub struct Args {
    /// Number of players (host only)
    #[arg(short, long)]
    pub players: Option<usize>,

    /// Number of bots (host only)
    #[arg(short, long)]
    pub bots: Option<usize>,

    /// IP address to connect to (client only)
    #[arg(short, long)]
    pub ip: Option<String>,
}

pub enum Mode {
    Host { players: usize, bots: usize },
    Client { ip: String },
}

pub fn parse_args() -> Mode {
    let args = Args::parse();

    if let Some(ip) = args.ip {
        Mode::Client { ip }
    } else {
        let players = args.players.unwrap_or(0);
        let bots = args.bots.unwrap_or(0);
        Mode::Host { players, bots }
    }
}
