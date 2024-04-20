use prost::Message as ProstMessage;
use std::net::TcpStream;
use std::io::{Read, Write};

pub mod messaging {
    include!(concat!(env!("OUT_DIR"), "/messaging.rs"));
}

use messaging::Message;

pub fn main() {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    // Send a message to the server
    let mut message = Message::default();
    message.id = "123".to_string();
    message.sender_id = "client".to_string();
    message.receiver_id = "server".to_string();
    message.content = "Hello from client".to_string();

    // Encode the message
    let mut message_bytes = Vec::new();
    message.encode(&mut message_bytes).unwrap();

    // Send the message through the TCP stream
    stream.write_all(&message_bytes).expect("Failed to write to server");

    // Receive a response from the server
    let mut response_buffer = [0; 1024];
    let bytes_read = stream.read(&mut response_buffer).expect("Failed to read response");
    let response_bytes = &response_buffer[..bytes_read];

    // Decode the response message
    let response_message = Message::decode(response_bytes).unwrap();
    println!("Received response from server: {:?}", response_message);
}
