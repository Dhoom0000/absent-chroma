use bevy::{
    anti_alias::taa::TemporalAntiAliasing,
    camera::visibility::RenderLayers,
    core_pipeline::{Skybox, tonemapping::Tonemapping},
    light::{
        AtmosphereEnvironmentMapLight, CascadeShadowConfigBuilder, SunDisk, VolumetricFog,
        VolumetricLight, light_consts::lux,
    },
    math::cubic_splines::LinearSpline,
    pbr::{Atmosphere, AtmosphereSettings, ScreenSpaceAmbientOcclusion},
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
struct Moon;

#[derive(Component, Clone)]
pub struct MainCamera;

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
                illuminance: lux::FULL_MOON_NIGHT * 500.,
                ..Default::default()
            },
            cascade_shadow_config.clone(),
            Moon,
            RenderLayers::from_layers(&[LAYER_WORLD]),
            Transform::from_xyz(5., 4., 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

        commands.spawn((
            DirectionalLight {
                shadows_enabled: true,
                illuminance: lux::DARK_OVERCAST_DAY,
                ..Default::default()
            },
            cascade_shadow_config,
            Sun,
            SunDisk::EARTH,
            RenderLayers::from_layers(&[LAYER_WORLD]),
            Transform::from_xyz(-5., -4., 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

        load_state.lights = true;
    }

    fn camera(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
        mut load_state: ResMut<LoadState>,
    ) {
        let metering_mask =
            asset_server.load(Path::new("textures").join("basic_metering_mask.png"));
        let night_sky = asset_server.load(Path::new("textures").join("night.ktx2"));
        // let day_sky = asset_server.load(Path::new("textures").join("specular.ktx2"));

        commands
            .spawn((
                Camera3d::default(),
                MainCamera,
                Camera {
                    clear_color: ClearColorConfig::Default,
                    order: 0,
                    is_active: true,
                    ..Default::default()
                },
                Projection::Perspective(PerspectiveProjection::default()),
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
                Tonemapping::AcesFitted,
                Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
                Bloom::NATURAL,
                Atmosphere::EARTH,
                AtmosphereSettings {
                    ..Default::default()
                },
                AtmosphereEnvironmentMapLight::default(),
                RenderLayers::from_layers(&[LAYER_WORLD, LAYER_PLAYER]),
                Msaa::Off,
                Visibility::Visible,
            ))
            .insert((
                Skybox {
                    image: night_sky.clone(),
                    brightness: 5000.,
                    ..Default::default()
                },
                DistanceFog {
                    color: Color::WHITE,
                    falloff: FogFalloff::Exponential { density: 0.00005 },
                    ..Default::default()
                },
                TemporalAntiAliasing::default(),
                ScreenSpaceAmbientOcclusion::default(),
            ));

        load_state.camera = true;
    }

    fn show_game(mut query: Query<&mut Visibility, With<MainCamera>>) {
        let mut visibility = query.single_mut().expect("Couldn't query Main Camera");

        *visibility = Visibility::Visible;
    }

    fn hide_game(mut query: Query<&mut Visibility, With<MainCamera>>) {
        let mut visibility = query.single_mut().expect("Couldn't query Main Camera");

        *visibility = Visibility::Hidden;
    }

    fn sun_cycle(
        mut moons: Query<&mut Transform, (With<Moon>, Without<Sun>)>,
        mut suns: Query<&mut Transform, (With<Sun>, Without<Moon>)>,
        time: Res<Time>,
        skyboxes: Query<&mut Skybox>,
    ) {
        moons.iter_mut().for_each(|mut tf| {
            tf.rotate_x(-time.delta_secs() * PI / 24.0);
            tf.rotate_z(time.delta_secs() * PI / 24.0);
        });

        suns.iter_mut().for_each(|mut tf| {
            tf.rotate_x(-time.delta_secs() * PI / 24.0);
            tf.rotate_z(time.delta_secs() * PI / 24.0);
        });

        for mut skybox in skyboxes {
            skybox.rotation = Quat::from_euler(
                EulerRot::ZYX,
                -PI / 3.0,
                0.0,
                time.elapsed_secs() * PI / 48.0,
            );
        }
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
        app.insert_resource(ClearColor(Color::Srgba(Srgba::hex("#000000").unwrap())));
        // app.insert_resource(AmbientLight {
        //     color: Color::srgb(0.05, 0.07, 0.1),
        //     brightness: 500.,
        //     affects_lightmapped_meshes: false,
        // });
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
