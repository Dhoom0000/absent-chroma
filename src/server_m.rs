mod common; // Import the common module
mod server; // Import the server module 

// Entry point for the server app
fn main() {
    server::app::start(); // Call the start function from the server module
}
