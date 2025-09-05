// define the modeles in this module
mod audio;
mod input;
mod network;
mod player;
mod plugins;
mod setup;
mod ui;

// import bevy crate prelude
use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{ExitCondition, WindowMode, WindowResolution},
};

use crate::client::plugins::SuperPlugin;

// My app name
const GAME_NAME: &str = "Absent Chroma";

// define some Render Layers to use throughout the game
pub(crate) const MY_WORLD_RENDER_LAYER: [usize; 1] = [2];
pub(crate) const MY_CAMERA_RENDER_LAYER: [usize; 1] = [2];
pub(crate) const MY_UI_RENDER_LAYER: [usize; 1] = [1];

// define some App state
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
enum AppState {
    #[default]
    MainMenu,
    LoadingScreen,
    InGame,
    PauseMenu,
    ConnectingToServer,
}

#[derive(Resource, Debug, Clone)]
struct PreviousAppState(Option<AppState>);

pub(super) struct Start;

impl Plugin for Start {
    fn build(&self, app: &mut App) {
        // configure custom settings for our window
        let custom_window_plugin = WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                resolution: WindowResolution::new(2560. / 4., 1440. / 4.)
                    .with_scale_factor_override(1.0),
                title: GAME_NAME.to_string(),
                name: Some(GAME_NAME.to_string()),
                resizable: false,
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        };

        // define logging setting for the plugin
        let log_filter_plugin = LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        };

        // Default Plugins with custom window settings, log settings, and optional Imageplugin setting
        app.add_plugins(
            DefaultPlugins
                .set(custom_window_plugin)
                .set(log_filter_plugin)
                .set(ImagePlugin::default_nearest()),
        );

        // Define and configure App states
        app.init_state::<AppState>();
        app.insert_resource(PreviousAppState(None));

        // Setup the ClearColor
        app.insert_resource(ClearColor(Color::Srgba(Srgba::hex("171717").unwrap())));

        // add all the other plugins and defined systems in their build scripts
        app.add_plugins(SuperPlugin);
    }
}
