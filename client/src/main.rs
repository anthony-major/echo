use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpStream;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    #[arg(short, long, default_value_t = 9000)]
    port: u16,

    #[arg(short, long, default_value = "")]
    message: String,
}

fn main() {
    let args = Args::parse();

    let addr = format!("{}:{}", args.address, args.port);

    println!("Connecting to {}...", addr);
    let stream = TcpStream::connect(addr.as_str()).expect(format!("Failed to connect to {}.", addr).as_str());
    println!("Connected.");

    println!("Sending message...");
    let message = args.message;
    let mut buf_writer = BufWriter::new(&stream);
    buf_writer.write(message.as_bytes()).expect("Failed to write to stream.");
    buf_writer.write(b"\n").expect("Failed to write to stream.");
    buf_writer.flush().expect("Failed to flush stream.");
    println!("Sent: {}", message);

    println!("Receiving echo...");
    let mut buf_reader = BufReader::new(&stream);
    let mut message = String::new();
    buf_reader.read_to_string(&mut message).expect("Failed to read from stream.");
    println!("Received echo: {}", message);

    println!("Shutting down...");
}
