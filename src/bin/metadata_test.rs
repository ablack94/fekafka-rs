use abkaf::messages::{
    self,
    metadata::{
        self,
        v0::{to_bytes, Topic, TopicMetadata},
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let req = metadata::v0::MetadataRequest {
        topics: vec![Topic {
            name: "coinbase-user".into(),
        }],
    };

    let mut stream = TcpStream::connect("core1:9094").await.unwrap();

    let mut buf = Vec::new();
    to_bytes(req, 99, &mut buf);
    println!("Sending {:?}", buf);

    let buf_len = buf.len();
    let buf_len: i32 = buf_len.try_into().unwrap();
    println!("Buflen {}", buf_len);

    stream.write_all(&buf_len.to_be_bytes()).await.unwrap();
    stream.write_all(&buf).await.unwrap();
    println!("Sent");

    loop {
        println!("Looking for response");
        let mut size_buf: [u8; 4] = [0, 0, 0, 0];
        stream.read_exact(&mut size_buf).await.unwrap();
        let n = i32::from_be_bytes(size_buf);
        println!("{:?}", size_buf);
        println!("Got a response of {n} bytes!");
        let mut buf = Vec::<u8>::new();
        buf.resize(n.try_into().unwrap(), 0);
        stream.read_exact(&mut buf).await.unwrap();
        println!("Payload: {:?}", buf);
    }
}
