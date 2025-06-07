use {
    crate::{framed_transport::FramedTransport, protocol::Protocol},
    anyhow::Result,
    serde::{Deserialize, Serialize},
    tokio::io::duplex,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Ping(String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Pong(String);

#[tokio::test]
async fn protocol_roundtrip_over_memory() -> Result<()> {
    let max_buf_size = 1024;
    let (a, b) = duplex(max_buf_size);

    let (a_r, a_w) = tokio::io::split(a);
    let (b_r, b_w) = tokio::io::split(b);

    let a_transport = FramedTransport::new(a_r, a_w);
    let b_transport = FramedTransport::new(b_r, b_w);

    let mut client_proto = Protocol::new(a_transport);
    let mut server_proto = Protocol::new(b_transport);

    let server = tokio::spawn(async move {
        let Ping(msg): Ping = server_proto.next_message().await.unwrap().unwrap();
        assert_eq!(msg, "hello");

        server_proto
            .send_message(&Pong("world".into()))
            .await
            .unwrap();
    });

    client_proto.send_message(&Ping("hello".into())).await?;

    let Pong(reply): Pong = client_proto.next_message().await.unwrap().unwrap();

    assert_eq!(reply, "world");

    server.await.unwrap();
    Ok(())
}
