# Echo
Simple TCP echo server and client written in Rust.

The server is written with asynchronous code using tokio and handles graceful exiting. 
By default, the server listens on localhost port 9000 for incoming requests. 
A listening port can be defined using the -p/--port flag.
The server program can be stopped using ctrl+c.

The client is written using std.
By default, the client connects to localhost port 9000 and sends an empty message.
A connection address can be defined with the -a/--address flag.
A connection port can be defined with the -p/--port flag.
Finally, a message can be defined with the -m/--message flag. If the message contains spaces, make sure to enclose it with quotes.
