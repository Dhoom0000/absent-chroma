use bevy::{
    anti_alias::fxaa::Fxaa,
    camera::{
        Exposure,
        visibility::{NoFrustumCulling, RenderLayers},
    },
    core_pipeline::tonemapping::Tonemapping,
    light::{CascadeShadowConfigBuilder, SunDisk, light_consts::lux},
    pbr::{Atmosphere, AtmosphereSettings},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
};

use std::f32::consts::PI;

use crate::client::{AppState, LAYER_PLAYER, LAYER_WORLD};

pub mod player;

pub struct WorldPlugin;

#[derive(Component, Clone)]
struct Sun;

#[derive(Component, Clone)]
struct MainCamera;

impl WorldPlugin {
    fn lights(mut commands: Commands) {
        let sun = DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..Default::default()
        };

        let cascade_shadow_config = CascadeShadowConfigBuilder {
            first_cascade_far_bound: 0.3,
            maximum_distance: 3.0,
            ..Default::default()
        }
        .build();

        commands.spawn((
            sun,
            cascade_shadow_config,
            Sun,
            SunDisk::EARTH,
            RenderLayers::from_layers(&[LAYER_WORLD]),
            Transform::from_xyz(0., 0.0, 10.).looking_at(Vec3::ZERO, Vec3::Y),
            bevy::camera::visibility::NoFrustumCulling,
        ));
    }

    fn camera(mut commands: Commands, _asset_server: Res<AssetServer>) {
        let projection = PerspectiveProjection::default();

        let camera_config = Camera {
            order: 0,
            is_active: true,
            msaa_writeback: true,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        };

        commands
            .spawn((
                RenderLayers::from_layers(&[LAYER_WORLD]),
                camera_config,
                Projection::Perspective(projection),
                MainCamera,
                Camera3d::default(),
                Atmosphere::EARTH,
                AtmosphereSettings {
                    aerial_view_lut_max_distance: 3.2e5,
                    scene_units_to_m: 1e+4,
                    rendering_method: bevy::pbr::AtmosphereMode::Raymarched,
                    ..Default::default()
                },
                Exposure::SUNLIGHT,
                Tonemapping::AcesFitted,
                Bloom::NATURAL,
                bevy::light::AtmosphereEnvironmentMapLight::default(),
                Hdr,
                Fxaa::default(),
                Transform::from_xyz(0., 10., -10.).looking_at(Vec3::ZERO, Vec3::Y),
            ))
            .insert((DistanceFog {
                color: Color::srgba(0.53, 0.7, 0.9, 1.0),
                directional_light_color: Color::srgba(1., 0.95, 0.85, 1.),
                directional_light_exponent: 30.,
                falloff: FogFalloff::from_visibility_colors(
                    100.,
                    Color::srgb(0.35, 0.5, 0.75),
                    Color::srgb(0.7, 0.85, 1.0),
                ),
            },));
    }

    fn player_camera(mut commands: Commands) {
        let projection = PerspectiveProjection::default();

        let camera_config = Camera {
            order: 1,
            is_active: true,
            msaa_writeback: true,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        };

        commands
            .spawn((
                RenderLayers::from_layers(&[LAYER_PLAYER]),
                camera_config,
                Projection::Perspective(projection),
                Camera3d::default(),
                Exposure::SUNLIGHT,
                Tonemapping::AcesFitted,
                Bloom::NATURAL,
                bevy::light::AtmosphereEnvironmentMapLight::default(),
                Hdr,
                Fxaa::default(),
                Transform::from_xyz(0., 10., -10.).looking_at(Vec3::ZERO, Vec3::Y),
            ))
            .insert((DistanceFog {
                color: Color::srgba(0.53, 0.7, 0.9, 1.0),
                directional_light_color: Color::srgba(1., 0.95, 0.85, 1.),
                directional_light_exponent: 30.,
                falloff: FogFalloff::from_visibility_colors(
                    100.,
                    Color::srgb(0.35, 0.5, 0.75),
                    Color::srgb(0.7, 0.85, 1.0),
                ),
            },));
    }

    fn spawn_plane(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            RenderLayers::from_layers(&[LAYER_WORLD]),
            NoFrustumCulling,
        ));
    }

    fn show_game(mut query: Query<&mut Visibility, With<MainCamera>>, mut commands: Commands) {
        let mut visibility = query.single_mut().expect("Couldn't query Main Camera");

        *visibility = Visibility::Visible;

        commands.set_state(AppState::InGame);
    }

    fn hide_game(mut query: Query<&mut Visibility, With<MainCamera>>) {
        let mut visibility = query.single_mut().expect("Couldn't query Main Camera");

        *visibility = Visibility::Hidden;
    }

    fn sun_cycle(mut suns: Query<&mut Transform, With<Sun>>, time: Res<Time>) {
        // For each sun, rotate them around X-axis.
        suns.iter_mut().for_each(|mut tf| {
            tf.rotate_x(-time.delta_secs() * PI / 12.0);
        });
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (Self::lights, Self::camera, Self::player_camera).chain(),
        );

        app.add_systems(Startup, Self::spawn_plane);

        app.add_systems(OnEnter(AppState::Load), Self::show_game);
        app.add_systems(OnExit(AppState::InGame), Self::hide_game);

        app.add_systems(Update, Self::sun_cycle.run_if(in_state(AppState::InGame)));

        app.add_plugins(player::PlayerPlugin);
    }
}
