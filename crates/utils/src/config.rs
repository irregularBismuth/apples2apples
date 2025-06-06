use {serde::Deserialize, std::collections::HashMap, std::path::PathBuf};

#[derive(Deserialize)]
pub struct Config {
    red_deck_fp: String,
    green_deck_fp: String,
    win_condition: HashMap<String, u8>,
}

impl Config {
    pub fn parse_config(path: PathBuf) -> anyhow::Result<Config> {
        let str = std::fs::read_to_string(path)?;
        let config = toml::from_str(&str)?;
        Ok(config)
    }

    pub fn red_deck_path(&self) -> &str {
        &self.red_deck_fp
    }

    pub fn green_deck_path(&self) -> &str {
        &self.green_deck_fp
    }

    pub fn get_required_apples(&self, total_players: usize) -> Option<u8> {
        self.win_condition
            .iter()
            .filter_map(|(k, &v)| {
                if let Some(n) = k.strip_suffix("_p")?.parse::<usize>().ok() {
                    Some((n, v))
                } else {
                    None
                }
            })
            .filter(|(n, _)| *n <= total_players)
            .max_by_key(|(n, _)| *n)
            .map(|(_, v)| v)
    }
}
