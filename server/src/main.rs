use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::signal;

async fn handle_client(mut stream: TcpStream, addr: SocketAddr) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut message = String::new();
    buf_reader.read_line(&mut message).await.unwrap();
    message = message.trim_end().to_string();
    println!("[{}] Message received: {}", addr, message);

    let mut buf_writer = BufWriter::new(&mut stream);
    buf_writer.write(message.as_bytes()).await.unwrap();
    buf_writer.flush().await.unwrap();
    println!("[{}] Sent message back.", addr);
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Listening on port 9000...");

    loop {
        tokio::select! {
            f = listener.accept() => {
                let (stream, addr) = f.unwrap();
                tokio::spawn(handle_client(stream, addr));
            }
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    println!("Exiting...");
}
