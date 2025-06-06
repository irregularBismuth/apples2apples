use anyhow::Result;
use apples_core::cards::card::{BaseCard, Card};
use apples_core::deck::deck::Deck;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
/// Generic deck loader that loads the deck from the file_path and tries and parses the file and
/// builds the deck and returns it if it succedes otherwise return error
pub async fn load_deck<T, P>(file_path: P) -> Result<Deck<T>>
where
    T: Card + From<BaseCard>,
    P: AsRef<Path>,
{
    let mut file = match File::open(file_path.as_ref()).await {
        Ok(f) => f,
        Err(e) => return Err(anyhow::anyhow!("Failed to open file: {:?}", e)),
    };

    let mut deck = Deck::<T>::new();
    let mut bytes = Vec::new();

    if let Err(e) = file.read_to_end(&mut bytes).await {
        return Err(anyhow::anyhow!("Failed to read file: {:?}", e));
    }

    let contents = String::from_utf8_lossy(&bytes);
    let mut id = 0;
    for (line_num, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let name = extract_between(&line, '[', ']');
        let text = extract_text(&line);

        if let (Some(name), Some(text)) = (name, text) {
            if !name.is_empty() && !text.is_empty() {
                let card = T::from(BaseCard::new(name, text, id));
                deck.add_card(card);
                id += 1;
            } else {
                eprintln!("Warning: Empty name or text at line {}", line_num + 1);
            }
        } else {
            eprintln!(
                "Warning: Failed to parse at line {}: '{}'",
                line_num + 1,
                line
            );
        }
    }

    Ok(deck)
}

/// Helper function to extract text between characthers start - end
fn extract_between(text: &str, start: char, end: char) -> Option<String> {
    text.split(start)
        .nth(1)
        .and_then(|s| s.split(end).next())
        .map(|s| s.trim().to_string())
}

/// Helper function to extract after hyphon
fn extract_text(line: &str) -> Option<String> {
    line.split(" -")
        .nth(1)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
