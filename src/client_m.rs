// #![cfg_attr(windows, windows_subsystem = "windows")]

use bevy::prelude::*;

mod client;
mod common;

/// Main entry point for the client application

fn main() {
    App::new().add_plugins(client::Create).run();
}
