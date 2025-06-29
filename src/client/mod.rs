// define the modeles in this module
mod audio;
mod input;
mod network;
mod player;
mod plugins;
mod setup;
mod ui;

// import bevy crate prelude
use bevy::prelude::*;

use crate::client::plugins::SuperPlugin;

// My app name
const GAME_NAME: &str = "Absent Chroma";

// define some Render Layers to use throughout the game
pub(crate) const MY_WORLD_RENDER_LAYER: [usize; 1] = [2];
pub(crate) const MY_CAMERA_RENDER_LAYER: [usize; 1] = [2];
pub(crate) const MY_UI_RENDER_LAYER: [usize; 1] = [1];

// define some App state
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    LoadingScreen,
    ConnectingToServer,
}

pub(super) struct Start;

impl Plugin for Start {
    fn build(&self, app: &mut App) {
        app.add_plugins(SuperPlugin);
        app.init_state::<AppState>();
    }
}
