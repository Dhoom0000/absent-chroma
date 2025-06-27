use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, AtmosphereSettings, CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
    render::{camera::Exposure, view::RenderLayers},
};
use bevy_renet::client_connected;

use crate::client::{
    input::handle_keyboard_input,
    network::{self, client_ping, receive_server_message},
    player::{add_render_layer, change_material, rotate_player, spawn_player},
    plugins::super_plugins,
    ui::{listen_ui_input, show_main_menu},
};

const MY_CAMERA_RENDER_LAYERS: [usize; 1] = [2];
pub const MY_WORLD_RENDER_LAYER: [usize; 1] = [2];

#[derive(Component, Clone)]
struct MyCamera;

#[derive(Component, Clone)]
pub struct Sun;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    // InGame,
    // LoadingScreen,
}

pub fn start() {
    // Build a new App
    let mut app = App::new();

    // register the plugins, Default plugins will pull all the necessary plugins for us
    app.add_plugins(super_plugins)
        .add_systems(
            Startup,
            (
                setup_camera_lights,        // initiate our main camera and some lights
                network::connect_to_server, // initiate logic to connect to the server
                show_main_menu,             // render the main menu UI
                spawn_player,
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
            (
                handle_keyboard_input,
                listen_ui_input,
                rotate_player,
                add_render_layer,
                sun_cycle,
            ),
        )
        .add_observer(change_material);

    // Run the App
    app.run();
}

fn setup_camera_lights(mut commands: Commands) {
    let directional_light = DirectionalLight {
        shadows_enabled: true,
        illuminance: lux::RAW_SUNLIGHT,
        ..default()
    };

    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    commands.spawn((
        Sun,
        directional_light,
        RenderLayers::from_layers(&MY_CAMERA_RENDER_LAYERS),
        cascade_shadow_config,
    ));

    let projection = Projection::Perspective(PerspectiveProjection::default());

    let camera_config = Camera {
        order: 1,
        is_active: true,
        hdr: true,
        clear_color: ClearColorConfig::Default,
        msaa_writeback: true,
        ..Default::default()
    };

    let camera = (
        Camera3d::default(),
        projection,
        camera_config,
        MyCamera,
        Transform::from_xyz(0., 0., 30.).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&MY_WORLD_RENDER_LAYER),
        Atmosphere::EARTH,
        AtmosphereSettings {
            aerial_view_lut_max_distance: 3.2e5,
            scene_units_to_m: 1e+4,
            ..Default::default()
        },
        Exposure::SUNLIGHT,
        Tonemapping::AcesFitted,
        Bloom::NATURAL,
        AmbientLight {
            brightness: 7e3,
            color: Color::WHITE,
            affects_lightmapped_meshes: true,
        },
    );
    commands.spawn(camera);
}

fn sun_cycle(mut suns: Query<&mut Transform, With<Sun>>, time: Res<Time>) {
    suns.iter_mut()
        .for_each(|mut tf| tf.rotate_x(-time.delta_secs() * PI / 12.0));
}
