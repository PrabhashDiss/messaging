# Messaging Application

This is a simple messaging application demonstrating the use of Protocol Buffers (protobuf) for defining message formats and prost_build for generating Rust code from the protobuf definitions.

## Overview

The messaging application consists of protobuf definitions for users and messages. The `User` message defines the structure of a user, while the `Message` message defines the structure of a message exchanged between users.

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

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
