use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{ExitCondition, WindowMode, WindowResolution},
};

use crate::client::network::login::UserLogin;

mod controls;
mod network;
mod plugins;
mod ui;
mod world;

const GAME_NAME: &str = "Absent Chroma";

pub const _DEFAULT_RENDER_LAYER: usize = 0;
pub const LAYER_UI: usize = 1;
pub const LAYER_WORLD: usize = 2;
pub const LAYER_PLAYER: usize = 3;
pub const LAYER_HUD: usize = 4;

#[derive(Clone, Debug)]
pub struct Create;

impl Plugin for Create {
    fn build(&self, app: &mut App) {
        let custom_window_plugin = WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                resolution: WindowResolution::new(2560, 1440).with_scale_factor_override(1.0),
                title: GAME_NAME.to_string(),
                name: Some(GAME_NAME.to_string()),
                resizable: false,
                ..default()
            }),
            primary_cursor_options: None,
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        };

        let log_filter_plugin = LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        };

        app.add_plugins(
            DefaultPlugins
                .set(custom_window_plugin)
                .set(log_filter_plugin),
        );

        app.init_state::<AppState>();

        app.insert_resource(PreviousAppState(None));

        app.insert_resource(UserLogin::default());

        app.add_plugins(plugins::SuperPlugin);
    }
}

/// Global application states.
///
/// Used to control transition between menus, game, and network connection screens.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
enum AppState {
    #[default]
    MainMenu,
    Load,
    InGame,
    Pause,
    ConnectToServer,
}

/// Resource tracking the last active [`AppState`].
#[derive(Resource, Debug, Clone)]
struct PreviousAppState(Option<AppState>);
