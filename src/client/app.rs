use std::f32::consts::PI;

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

pub fn start() {
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

    let log_filter_plugin = LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
        level: bevy::log::Level::DEBUG,
        ..default()
    };

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(custom_window_plugin)
            .set(log_filter_plugin),
    )
    .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
    .insert_resource(ClearColor(Color::Srgba(Srgba::hex("171717").unwrap())))
    .insert_resource(UserLogin::default())
    .insert_resource(KEMClientKey::default())
    .add_systems(
        Startup,
        (
            setup_camera_lights,
            network::connect_to_server,
            render_subject,
            disable_backface_culling.after(render_subject),
            show_main_menu,
        ),
    )
    .add_systems(
        Update,
        (client_ping, receive_server_message).run_if(client_connected),
    )
    .add_systems(Update, (rotate_subject, listen_ui_input));

    app.run();
}

fn setup_camera_lights(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let projection = Projection::Perspective(PerspectiveProjection::default());

    let camera_config = Camera {
        order: 1,
        is_active: true,
        hdr: true,
        ..Default::default()
    };

    let camera_bundle = (projection, camera_config, Camera3d::default());

    commands.spawn((
        MyCamera,
        camera_bundle,
        Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[0, 1]),
    ));

    let point_light = PointLight::default();
    let directional_light = DirectionalLight::default();

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

    // // spawn an entity to test the setup
    // let cube = (Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),MeshMaterial3d(materials.add(Color::BLACK)),Transform::from_xyz(0., 0., 0.));

    // commands.spawn((cube,RenderLayers::from_layers(&[0,1])));
}

fn render_subject(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let scene: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/malfoy.glb"));

    let scale = Vec3::splat(20.);

    commands.spawn((
        Model,
        SceneRoot(scene),
        Transform::from_xyz(0., 0., 0.).with_scale(scale),
        RenderLayers::from_layers(&[0, 1]),
    ));
}

fn rotate_subject(mut query: Query<&mut Transform, With<Model>>, time: Res<Time>) {
    for mut model in query.iter_mut() {
        let rotation_factor = 2.;
        model.rotate_axis(Dir3::Y, rotation_factor * time.delta_secs());
    }
}

fn disable_backface_culling(
    mut query: Query<&MeshMaterial3d<StandardMaterial>, With<Model>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for mat_handle in query.iter_mut() {
        if let Some(mat) = materials.get_mut(mat_handle) {
            mat.cull_mode = None;
        }
    }
}
