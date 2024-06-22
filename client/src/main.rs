use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpStream;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Connecting to localhost:9000...");
    let stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    println!("Connected.");

    println!("Sending message...");
    let default_message = String::from("");
    let message = args.get(1).unwrap_or(&default_message);
    let mut buf_writer = BufWriter::new(&stream);
    buf_writer.write(message.as_bytes()).unwrap();
    buf_writer.write(b"\n").unwrap();
    buf_writer.flush().unwrap();
    println!("Sent: {}", message);

    println!("Receiving echo...");
    let mut buf_reader = BufReader::new(&stream);
    let mut message = String::new();
    buf_reader.read_to_string(&mut message).unwrap();
    println!("Received echo: {}", message);

    println!("Shutting down...");
}
