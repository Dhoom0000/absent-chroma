use std::{default, f32::consts::PI};

use bevy::{
    log::LogPlugin,
    math::VectorSpace,
    prelude::*,
    render::{
        camera::{RenderTarget, Viewport},
        view::RenderLayers,
    },
    scene::SceneInstance,
    ui::UiPlugin,
    window::{
        ExitCondition, SystemCursorIcon, VideoMode, WindowMode, WindowRef, WindowResolution,
        WindowTheme,
    },
};
use bevy_renet::{
    RenetClientPlugin, client_connected, netcode::NetcodeClientPlugin, renet::RenetClient,
};
use bincode::de;

use crate::{
    client::{
        input::handle_keyboard_input,
        network::{self, client_ping, receive_server_message},
        ui::{listen_ui_input, show_main_menu},
    },
    common::{encryption::KEMClientKey, user::UserLogin},
};

const GAME_NAME: &str = "Absent Chroma";

#[derive(Component)]
struct MyCamera;

#[derive(Component)]
struct Model;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    LoadingScreen,
}

pub fn start() {
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

    // Build a new App
    let mut app = App::new();

    // register the plugins, Default plugins will pull all the necessary plugins for us
    app.add_plugins(
        DefaultPlugins
            .set(custom_window_plugin)
            .set(log_filter_plugin),
    )
    // networking plugins
    .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
    // Set Clear Color
    .insert_resource(ClearColor(Color::Srgba(Srgba::hex("171717").unwrap())))
    // Resources for Encryption and Authentication
    .insert_resource(UserLogin::default())
    .insert_resource(KEMClientKey::default())
    // Register startup systems
    .add_systems(
        Startup,
        (
            setup_camera_lights,        // initiate our main camera and some lights
            network::connect_to_server, // initiate logic to connect to the server
            render_subject,             // render a 3d model, for test for now
            disable_backface_culling.after(render_subject), // if necessary
            show_main_menu,             // render the main menu UI
        ),
    )
    // register the update system relating to networking, just simple ping pong for testing for now
    .add_systems(
        Update,
        (client_ping, receive_server_message).run_if(client_connected),
    )
    // register update systems for testing if the 3d model is working, and proper keyboard input handling
    .add_systems(
        Update,
        (rotate_subject, listen_ui_input, handle_keyboard_input),
    );

    // Run the App
    app.run();
}

fn setup_camera_lights(mut commands: Commands) {
    // Define the projection we want to use
    let projection = Projection::Perspective(PerspectiveProjection::default());

    // Define the camera properties
    let camera_config = Camera {
        order: 1,
        is_active: true,
        hdr: true,
        ..Default::default()
    };

    // Pull everything into a bundle
    // add Camera3d to provide a render graph to our camera component
    // Also add a marker component for easier querying
    let camera_bundle = (projection, camera_config, Camera3d::default(), MyCamera);

    // spawn the camera, at a certain position, and configured RenderLayer
    commands.spawn((
        camera_bundle,
        Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[0, 1]),
    ));

    // Define some light component
    let point_light = PointLight::default();
    let directional_light = DirectionalLight::default();

    // spawn the lights, at a certain position, and RenderLayer
    commands.spawn((
        point_light,
        Transform::from_xyz(-10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[0, 1]),
    ));

    commands.spawn((
        directional_light,
        Transform::from_xyz(10., 10., 20.).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[0, 1]),
    ));
}

fn render_subject(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // Load the asset
    let scene: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/malfoy.glb"));

    // Set the scale such that it is visible
    let scale = Vec3::splat(20.);

    // Add a marker compnent 'Model', spawn at given transform and draw to appropriate RenderLayer
    commands.spawn((
        Model,
        SceneRoot(scene),
        Transform::from_xyz(0., 0., 0.).with_scale(scale),
        RenderLayers::from_layers(&[0, 1]),
    ));
}

fn rotate_subject(mut query: Query<&mut Transform, With<Model>>, time: Res<Time>) {
    // for all the models, change the rotation by a defined rotation factor
    for mut model in query.iter_mut() {
        let rotation_factor = 2.;
        model.rotate_axis(Dir3::Y, rotation_factor * time.delta_secs());
    }
}

fn disable_backface_culling(
    mut query: Query<&MeshMaterial3d<StandardMaterial>, With<Model>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // disable backface culling for 3d models if necessary
    for mat_handle in query.iter_mut() {
        if let Some(mat) = materials.get_mut(mat_handle) {
            mat.cull_mode = None;
        }
    }
}
