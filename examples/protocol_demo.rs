use {
    anyhow::Result,
    apples_core::{cards::card::Card, RedCard},
    apples_protocol::{Frame, FrameCodec, Kind},
    bytes::Bytes,
    futures_util::{SinkExt, StreamExt},
    serde_json::{from_slice, to_vec},
    tokio::net::{TcpListener, TcpStream},
    tokio_util::codec::Framed,
};

#[tokio::main]
async fn main() -> Result<()> {
    const ADDR: &str = "127.0.0.1:5555";

    let listener = TcpListener::bind(ADDR).await?;
    let server = tokio::spawn(async move {
        let (socket, _peer) = listener.accept().await?;
        let mut framed = Framed::new(socket, FrameCodec::with_default_limit());

        // Ask the connected player to submit their best red card.
        let prompt = Frame::new(
            Kind::Game,
            0,
            Bytes::from_static(b"Judge: play a red card!"),
        );
        framed.send(prompt).await?;

        // Await the player's response and decode the red card without copying the payload.
        if let Some(frame) = framed.next().await.transpose()? {
            let card: RedCard = from_slice(frame.payload())?;
            println!("judge received red card: {}", card);
        }

        Ok::<_, anyhow::Error>(())
    });

    let stream = TcpStream::connect(ADDR).await?;
    let mut framed = Framed::new(stream, FrameCodec::with_default_limit());

    // Wait for the judge's prompt before responding.
    if let Some(frame) = framed.next().await.transpose()? {
        println!(
            "player received prompt: {}",
            String::from_utf8_lossy(frame.payload())
        );

        let red_card = RedCard::new(
            42usize,
            "Fresh Socks",
            "The warm comfort of dryer-fresh socks",
        );
        let payload = to_vec(&red_card)?;
        let reply = Frame::new(Kind::Game, 0, Bytes::from(payload));
        framed.send(reply).await?;
    }

    server.await??;
    Ok(())
}
