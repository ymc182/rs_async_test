use std::env;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let PORT = env::var("PORT").unwrap_or("8080".to_string());
    let HOST = "localhost";
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT)).await?;

    let (tx, rx) = broadcast::channel::<String>(10);
    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        tokio::spawn(async move {
            handle_connection(socket, addr.clone(), tx).await;
        });
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    addr: std::net::SocketAddr,
    tx: broadcast::Sender<String>,
) {
    println!("New connection: {}", socket.peer_addr().unwrap());
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    let mut rx = tx.subscribe();
    loop {
        tokio::select! {
            result = reader.read_line(&mut line) =>{
                            if result.unwrap() == 0 {
                                 break;
                            }
              tx.send(format!("{}: {}", addr, line)).unwrap();

              line.clear();
            },

              result = rx.recv() =>{
                 if let Ok(msg) = result.clone() {
                    writer.write_all(msg.as_bytes()).await.unwrap();                  }
              }
        }

        //receive from all clients
    }
}
