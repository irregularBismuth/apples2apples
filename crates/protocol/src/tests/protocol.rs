#[cfg(test)]
mod tests {
    use crate::{reader::MessageReader, writer::MessageWriter};
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use tokio::io::duplex;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Ping(String);

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Pong(String);

    #[tokio::test]
    async fn reader_writer_roundtrip_over_memory() -> Result<()> {
        let (a, b) = duplex(1024);
        let (a_r, a_w) = tokio::io::split(a);
        let (b_r, b_w) = tokio::io::split(b);

        let mut client_reader = MessageReader::new(a_r);
        let mut client_writer = MessageWriter::new(a_w);
        let mut server_reader = MessageReader::new(b_r);
        let mut server_writer = MessageWriter::new(b_w);

        let server_task = tokio::spawn(async move {
            let Ping(msg) = server_reader.next_message().await.unwrap().unwrap();
            assert_eq!(msg, "hello");

            server_writer
                .send_message(&Pong("world".into()))
                .await
                .unwrap();
        });

        client_writer.send_message(&Ping("hello".into())).await?;
        let Pong(reply) = client_reader.next_message().await.unwrap().unwrap();
        assert_eq!(reply, "world");

        server_task.await.unwrap();
        Ok(())
    }
}
