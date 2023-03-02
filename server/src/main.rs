use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:8080").await?;
    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(socket, addr.clone()).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, addr: std::net::SocketAddr) {
    println!("New connection: {}", socket.peer_addr().unwrap());
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }
        writer.write_all(line.as_bytes()).await.unwrap();
        println!("{}: {}", addr, line);
        line.clear();
    }
}
