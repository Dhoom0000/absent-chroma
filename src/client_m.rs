//for windows to not show console window
// #![cfg_attr(windows, windows_subsystem = "windows")]
// Allow some for beginner code
// #![allow(unused_imports)]
// #![allow(unused_parens)]
// #![allow(dead_code)]

use bevy::prelude::*;

use crate::client::Start;

mod client; //import the client module
mod common; //import the common module

// Entry point for the client app

fn main() {
    App::new().add_plugins(Start).run();
}
