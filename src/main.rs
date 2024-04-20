// Import the server and client modules
mod server;
mod client;

fn main() {
    // Run both server and client
    std::thread::spawn(|| {
        server::main();
    });
    client::main();
}
