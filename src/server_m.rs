// Allow some for beginner code
// #[allow(dead_code)]
// #[allow(unused_imports)]
// #[allow(unused_braces)]
mod common; // Import the common module
mod server; // Import the server module 

pub use bevy::prelude::*;

// Entry point for the server app
fn main() {
    server::app::start(); // Call the start function from the server module
}
