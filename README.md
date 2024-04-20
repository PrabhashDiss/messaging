# Messaging Application

This is a simple messaging application demonstrating the use of Protocol Buffers (protobuf) for defining message formats and prost_build for generating Rust code from the protobuf definitions.

## Overview

The messaging application consists of protobuf definitions for users and messages. The `User` message defines the structure of a user, while the `Message` message defines the structure of a message exchanged between users. Additionally, encoding and decoding functionality for User and Message structs has been added.

## Usage

To use this application, make sure you have Rust and Cargo installed on your system.

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/messaging.git
    ```

2. Navigate into the project directory:

    ```bash
    cd messaging
    ```

3. Build the project:

    ```bash
    cargo build
    ```

4. Run the example:

    ```bash
    cargo run
    ```

## Code Generation with prost_build

The Rust code for working with protobuf messages is automatically generated from the protobuf definitions using prost_build. Here's how it works:

1. Define protobuf messages in `src/messaging.proto`. For example:

    ```proto
    syntax = "proto3";

    package messaging;

    message User {
        string id = 1;
        string username = 2;
    }

    message Message {
        string id = 1;
        string sender_id = 2;
        string receiver_id = 3;
        string content = 4;
    }
    ```

2. Define a build script in `build.rs` to generate Rust code from the protobuf file:

    ```rust
    use std::io::Result;

    fn main() -> Result<()> {
        prost_build::compile_protos(&["src/messaging.proto"], &["src/"])?;
        Ok(())
    }
    ```

3. The generated Rust code will be placed in the `src/` directory and can be included in your Rust code as needed.

## Importing Prost's Runtime Support

In your Rust code, import Prost's runtime support as follows:

```rust
// Import Prost's runtime support
use prost::Message as ProstMessage;
```

## TCP Implementation

For TCP communication between clients and servers, the application has been extended with TCP server and client implementations. The server listens for incoming connections, processes messages, and sends responses, while the client connects to the server, sends messages, and receives responses.

Here's how to use the TCP implementation:

1. Start the server:

    ```bash
    cargo run --bin server
    ```

2. Start the client:

    ```bash
    cargo run --bin client
    ```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
