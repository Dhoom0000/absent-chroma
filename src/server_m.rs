mod server; // Import the server module
mod common; // Import the common module

// Entry point for the server app
fn main() {
    server::app::start(); // Call the start function from the server module
}