use anyhow::Result;
use std::net::SocketAddrV4;
use tokio::net::TcpStream;

#[doc = "client.md"]
pub async fn client_main(socket: SocketAddrV4) -> Result<()> {
    let stream = TcpStream::connect(socket).await?;

    Ok(())
}
