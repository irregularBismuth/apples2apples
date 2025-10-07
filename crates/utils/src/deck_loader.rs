use {
    anyhow::{bail, Context, Result},
    apples_core::{
        cards::card::{BaseCard, Card},
        deck::deck::Deck,
    },
    std::path::Path,
    tokio::{fs::File, io::AsyncReadExt},
};

pub async fn load_deck<T, P>(file_path: P) -> Result<Deck<T>>
where
    T: Card + From<BaseCard>,
    P: AsRef<Path>,
{
    let mut file = File::open(file_path.as_ref())
        .await
        .with_context(|| format!("Failed to open {:?}", file_path.as_ref()))?;

    let mut buf = Vec::new();

    file.read_to_end(&mut buf)
        .await
        .context("Failed to read deck file")?;

    let contents = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(err) => String::from_utf8_lossy(&err.into_bytes()).into_owned(),
    };

    let mut deck = Deck::<T>::new();
    let mut id: usize = 0;

    for (idx, raw_line) in contents.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let (name_start, name_end) = match (line.find('['), line.find(']')) {
            (Some(s), Some(e)) if e > s => (s + 1, e),
            _ => {
                bail!("Line {line_no}: missing [name] segment: '{line}'");
            }
        };
        let name = line[name_start..name_end].trim();
        if name.is_empty() {
            bail!("Line {line_no}: empty name inside [ ]");
        }

        let after_bracket = line[name_end + 1..].trim_start();
        anyhow::ensure!(
            after_bracket.starts_with('-'),
            "Line {line_no}: missing '-' separator"
        );

        let text = after_bracket[1..].trim();
        if text.is_empty() {
            bail!("Line {line_no}: empty text after ' - '");
        }

        let card = T::from(BaseCard::new(id, name.to_string(), text.to_string()));
        deck.add_card(card);
        id = id.checked_add(1).context("Card ID overflow")?;
    }

    Ok(deck)
}
