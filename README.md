# Echo
Simple TCP echo server and client written in Rust.

The server is written with asynchronous code using tokio and handles graceful exiting. It listens on localhost port 9000 for incoming requests. The server accepts no arguments. You can run it using ```cargo run``` in the server project.

The client is written using std. The client accepts one argument, which is the message to send to the server. You can run it using ```cargo run -- "your_message_here"``` in the client project.
