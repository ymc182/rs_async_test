use std::env;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
#[tokio::main]
async fn main() {
    let PORT = env::var("PORT").unwrap_or("8080".to_string());
    let HOST = "localhost";
    let mut stream = TcpStream::connect(format!("{}:{}", HOST, PORT))
        .await
        .unwrap();

    let (mut reader, mut writer) = stream.into_split();

    tokio::spawn(async move {
        //receive from server
        let mut line = String::new();
        let mut reader = BufReader::new(reader);
        loop {
            reader.read_line(&mut line).await.unwrap();
            println!("{}", line);
            line.clear();
        }
    });

    tokio::spawn(async move {
        //send to server
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            writer.write_all(input.as_bytes()).await.unwrap();
        }
    });

    loop {}
}
