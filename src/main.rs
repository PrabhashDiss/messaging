// Include the `messaging` module, which is generated from messaging.proto.
pub mod messaging {
    include!(concat!(env!("OUT_DIR"), "/messaging.rs"));
}

// Usage example
use messaging::{User, Message};

fn main() {
    let mut user = User::default();
    user.id = "123".to_string();
    user.username = "user123".to_string();

    println!("{:?}", user);

    let mut message = Message::default();
    message.id = "456".to_string();
    message.sender_id = "123".to_string();
    message.receiver_id = "789".to_string();
    message.content = "Hello, world!".to_string();

    println!("{:?}", message);
}
