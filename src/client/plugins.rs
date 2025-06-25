use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{ExitCondition, WindowMode, WindowResolution},
};
use bevy_renet::{RenetClientPlugin, netcode::NetcodeClientPlugin};

use crate::common::{encryption::KEMClientKey, user::UserLogin};

const GAME_NAME: &str = "Absent Chroma";

pub(super) fn super_plugins(app: &mut App) {
    // define window settings for the plugin
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

    // Default Plugins with custom window settings
    app.add_plugins(DefaultPlugins.set(custom_window_plugin));

    // Logging plugin
    app.add_plugins(log_filter_plugin);

    // Networking plugin
    app.add_plugins((RenetClientPlugin, NetcodeClientPlugin));

    // Setup the ClearColor
    app.insert_resource(ClearColor(Color::Srgba(Srgba::hex("171717").unwrap())));

    // Resources to store encryption keys and User Login details
    app.insert_resource((UserLogin::default()));
    app.insert_resource(KEMClientKey::default());
}

pub(super) fn startup_system_plugin(app: &mut App) {}
