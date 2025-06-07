use {serde::Deserialize, std::collections::HashMap, std::path::PathBuf};

#[derive(Deserialize)]
pub struct Config {
    red_deck_fp: String,
    green_deck_fp: String,
    win_condition: HashMap<String, usize>,
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

    pub fn get_required_apples(&self, total_players: usize) -> Option<usize> {
        self.win_condition
            .iter()
            .filter_map(|(k, &v)| {
                k.strip_suffix("_p")
                    .and_then(|s| s.parse::<usize>().ok())
                    .map(|n| (n, v))
            })
            .fold(None, |acc, (n, v)| match acc {
                Some((best_n, _)) if n <= total_players && n > best_n => Some((n, v)),
                Some(_) => acc,
                None => Some((n, v)),
            })
            .map(|(_, v)| v)
    }
}
