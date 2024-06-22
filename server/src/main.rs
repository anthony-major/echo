use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::signal;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}

async fn handle_client(mut stream: TcpStream, addr: SocketAddr) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut message = String::new();
    buf_reader.read_line(&mut message).await.expect(format!("[{}] Failed to read line from stream.", addr).as_str());
    message = message.trim_end().to_string();
    println!("[{}] Message received: {}", addr, message);

    let mut buf_writer = BufWriter::new(&mut stream);
    buf_writer.write(message.as_bytes()).await.expect(format!("[{}] Failed to write to stream.", addr).as_str());
    buf_writer.flush().await.expect(format!("[{}] Failed to flush stream.", addr).as_str());
    println!("[{}] Sent message back.", addr);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let addr = format!("127.0.0.1:{}", args.port);

    let listener = TcpListener::bind(addr.as_str()).await.expect(format!("Failed to bind to {}.", addr).as_str());
    println!("Listening on port {}...", args.port);
    println!("Press ctrl+c to exit.");

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
