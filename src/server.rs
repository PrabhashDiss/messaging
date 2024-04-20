// Import Prost's runtime support
use prost::Message as ProstMessage;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// Include the `messaging` module, which is generated from messaging.proto.
pub mod messaging {
    include!(concat!(env!("OUT_DIR"), "/messaging.rs"));
}

// Usage example
use messaging::{User, Message};

pub fn main() {
    // Start a TCP listener
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    // Accept incoming connections
    if let Ok((mut stream, _)) = listener.accept() {
        // Read incoming message from the TCP stream
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read");
        let received_message_bytes = &buffer[..bytes_read];

        // Decode the received message
        let received_message = Message::decode(received_message_bytes).unwrap();
        println!("Received message: {:?}", received_message);

        // Send a response message
        let mut response_message = Message::default();
        response_message.id = "789".to_string();
        response_message.sender_id = "server".to_string();
        response_message.receiver_id = "client".to_string();
        response_message.content = "Response from server".to_string();

        // Encode the response message
        let mut response_bytes = Vec::new();
        response_message.encode(&mut response_bytes).unwrap();

        // Send the response message through the TCP stream
        stream.write_all(&response_bytes).expect("Failed to write");
    } else {
        eprintln!("Failed to accept incoming connection");
    }
}
