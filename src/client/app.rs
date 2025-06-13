use bevy::prelude::*;

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
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        close_when_requested: true,
    };

    App::new()
        .add_plugins(DefaultPlugins.set(custom_window_plugin))
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("171717").unwrap())))
        .run();
}
