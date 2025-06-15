use bevy::{prelude::*, window::{WindowMode, WindowResolution, ExitCondition, WindowTheme}};
use bevy_renet::{netcode::NetcodeClientPlugin, RenetClientPlugin};

use crate::client::network;

const GAME_NAME: &str = "Absent Chroma";

pub fn start(){
    let custom_window_plugin = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            position: WindowPosition::Centered(MonitorSelection::Current),
            resolution: WindowResolution::new(2560./4., 1440./4.).with_scale_factor_override(1.),
            title: GAME_NAME.to_string(),
            name: Some(GAME_NAME.to_string()),
            resizable: true,
            window_theme: Some(WindowTheme::Dark),
            prevent_default_event_handling: false,
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        close_when_requested: true,
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(custom_window_plugin))
        .add_plugins((RenetClientPlugin,NetcodeClientPlugin))
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("171717").unwrap())))
        .add_systems(Startup, (setup_camera_lights,network::connect_to_server));


    app.run()
        


}

fn setup_camera_lights(){
    // initialise a camera and lights setup, with custom render graphs if needed
}