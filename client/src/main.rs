use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8080").await.unwrap();
    let (mut reader, mut writer) = stream.split();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        writer.write_all(buf.as_bytes()).await.unwrap();

        let mut buf = [0; 1024];
        let n = reader.read(&mut buf).await.unwrap();
        if n == 0 {
            return;
        }
        println!("{}: {}", "localhost", String::from_utf8_lossy(&buf[..n]));
    }
}
