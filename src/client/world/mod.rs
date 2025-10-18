use bevy::{
    anti_alias::{fxaa::Fxaa, taa::TemporalAntiAliasing},
    camera::{
        Exposure,
        visibility::{NoFrustumCulling, RenderLayers},
    },
    color::palettes,
    core_pipeline::{Skybox, tonemapping::Tonemapping},
    light::{
        AtmosphereEnvironmentMapLight, CascadeShadowConfigBuilder, SunDisk, VolumetricFog,
        VolumetricLight, light_consts::lux,
    },
    math::{VectorSpace, cubic_splines::LinearSpline},
    pbr::{Atmosphere, AtmosphereSettings},
    post_process::{
        auto_exposure::{AutoExposure, AutoExposureCompensationCurve, AutoExposurePlugin},
        bloom::Bloom,
    },
    prelude::*,
    render::view::Hdr,
};

use std::{f32::consts::PI, path::Path};

use crate::client::{AppState, LAYER_PLAYER, LAYER_WORLD};

pub mod player;
pub mod scene;

pub struct WorldPlugin;

#[derive(Resource, Default, Debug)]
pub struct LoadState {
    pub lights: bool,
    pub camera: bool,
    pub player: bool,
    pub terrain: bool,
}

#[derive(Component, Clone)]
struct Sun;

#[derive(Component, Clone)]
struct MainCamera;

impl WorldPlugin {
    fn lights(mut commands: Commands, mut load_state: ResMut<LoadState>) {
        let cascade_shadow_config = CascadeShadowConfigBuilder {
            first_cascade_far_bound: 0.3,
            maximum_distance: 10.0,
            ..Default::default()
        }
        .build();

        commands.spawn((
            DirectionalLight {
                shadows_enabled: true,
                illuminance: lux::FULL_MOON_NIGHT * 500.0,
                ..Default::default()
            },
            cascade_shadow_config,
            Sun,
            SunDisk::EARTH,
            LAYER_PLAYER.with(2),
            Transform::default(),
            VolumetricLight,
        ));

        load_state.lights = true;
    }

    fn camera(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
        mut load_state: ResMut<LoadState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let metering_mask =
            asset_server.load(Path::new("textures").join("basic_metering_mask.png"));
        let night_sky = asset_server.load(Path::new("textures").join("night.ktx2"));

        commands.spawn((
            Camera3d::default(),
            Camera {
                clear_color: ClearColorConfig::None,
                is_active: true,
                order: 1,
                ..Default::default()
            },
            LAYER_PLAYER,
            Transform::from_xyz(10.0, 0.10, -10.0).looking_at(Vec3::Y * 10., Vec3::Y),
            Hdr,
            Msaa::Sample4,
        ));

        commands.spawn((
            Camera3d::default(),
            MainCamera,
            Camera {
                clear_color: ClearColorConfig::None,
                order: 0,
                is_active: true,
                ..Default::default()
            },
            Hdr,
            AutoExposure {
                range: -4.5..=14.0,
                speed_brighten: 60.0,
                speed_darken: 20.0,
                metering_mask: metering_mask.clone(),
                compensation_curve: compensation_curves.add(
                    AutoExposureCompensationCurve::from_curve(LinearSpline::new([
                        vec2(-8.0, 0.5),
                        vec2(4.0, -2.0),
                    ]))
                    .unwrap(),
                ),
                ..Default::default()
            },
            Skybox {
                image: night_sky,
                brightness: 500.0,
                rotation: Quat::default(),
            },
            Tonemapping::AcesFitted,
            Transform::from_xyz(0.0, 0.15, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
            Bloom::NATURAL,
            Atmosphere::EARTH,
            AtmosphereSettings {
                aerial_view_lut_max_distance: 3.2e5,
                scene_units_to_m: 1e+4,
                ..Default::default()
            },
            LAYER_WORLD,
            Visibility::Hidden,
            Msaa::Sample4,
        ));

        load_state.camera = true;

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.74, 0.89, 0.99),
                perceptual_roughness: 0.0,
                cull_mode: None,
                ..default()
            })),
            Transform::from_xyz(0.0, -0.1, 0.0),
            LAYER_WORLD,
        ));
    }

    fn show_game(mut query: Query<&mut Visibility, With<MainCamera>>) {
        let mut visibility = query.single_mut().expect("Couldn't query Main Camera");

        *visibility = Visibility::Visible;
    }

    fn hide_game(mut query: Query<&mut Visibility, With<MainCamera>>) {
        let mut visibility = query.single_mut().expect("Couldn't query Main Camera");

        *visibility = Visibility::Hidden;
    }

    fn sun_cycle(mut suns: Query<&mut Transform, With<Sun>>, time: Res<Time>) {
        let t = ((time.elapsed_secs() - 1.0).max(0.0) + 90.0 * 0.3);
        let day_fract = ((t % 90.0) / 90.0).clamp(0.0, 1.0);
        // For each sun, rotate them around X-axis.
        suns.iter_mut().for_each(|mut tf| {
            tf.rotation = Quat::from_euler(EulerRot::ZXY, PI / 3.0, 0.0, -day_fract * PI * 2.0);
        });
    }

    fn is_loaded(load_state: Res<LoadState>, mut next_state: ResMut<NextState<AppState>>) {
        if load_state.camera && load_state.lights && load_state.player && load_state.terrain {
            next_state.set(AppState::InGame);
        }
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadState::default());
        app.insert_resource(AmbientLight::NONE);
        app.add_plugins(AutoExposurePlugin);
        app.add_systems(
            OnEnter(AppState::Load),
            (Self::lights, Self::camera).chain(),
        );

        app.add_systems(OnEnter(AppState::InGame), Self::show_game);
        app.add_systems(OnExit(AppState::InGame), Self::hide_game);

        app.add_systems(Update, Self::is_loaded.run_if(in_state(AppState::Load)));

        app.add_systems(Update, Self::sun_cycle.run_if(in_state(AppState::InGame)));

        app.add_plugins(player::PlayerPlugin);
        app.add_plugins(scene::ScenePlugin);
    }
}
