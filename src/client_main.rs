use anyhow::Result;
use std::net::SocketAddrV4;
#[doc = "client.md"]
pub async fn client_main(socket: SocketAddrV4) -> Result<()> {
    println!("{}", socket.ip());
    println!("{}", socket.port());
    Ok(())
}
