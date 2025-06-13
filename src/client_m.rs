// #![cfg_attr(windows, windows_subsystem = "windows")] //for windows to not show console window
#![allow(unused_imports)]
#![allow(unused_parens)]

mod client; //import the client module
mod common; //import the common module

// Entry point for the client app

fn main() {
    client::app::start(); //start the client
}
