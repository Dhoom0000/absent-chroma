// This module/Plugin will set-up our Main Camera, Ambient Light, Atmosphere/Sun etc.

use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, AtmosphereSettings, CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
    render::{camera::Exposure, view::RenderLayers},
};

use crate::client::{AppState, MY_CAMERA_RENDER_LAYER, audio, player};

pub struct SetupPlugin;

impl SetupPlugin {
    fn lights(mut commands: Commands) {
        // This will be our Sun
        let directional_light = DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        };

        // Configure shadows to limit the distance it casts the shadows
        let cascade_shadow_config = CascadeShadowConfigBuilder {
            first_cascade_far_bound: 0.3,
            maximum_distance: 3.0,
            ..default()
        }
        .build();

        // Spawn the Sun with needed components configuration
        commands.spawn((
            Sun,
            directional_light.clone(),
            RenderLayers::from_layers(&MY_CAMERA_RENDER_LAYER),
            cascade_shadow_config.clone(),
        ));

        // commands.spawn((
        //     Sun,
        //     directional_light,
        //     RenderLayers::from_layers(&MY_CAMERA_RENDER_LAYER),
        //     cascade_shadow_config,
        //     Transform::from_rotation(Quat::from_axis_angle(Vec3::X, 0.1)),
        // ));
    }

    fn camera(mut commands: Commands) {
        // define camera projection
        let projection = Projection::Perspective(PerspectiveProjection::default());

        // configure camera
        let camera_config = Camera {
            order: 0,
            is_active: false,
            hdr: true,
            msaa_writeback: true,
            clear_color: ClearColorConfig::Default,
            ..Default::default()
        };

        // make a Camera Bundle to spawn
        let camera = (
            Camera3d::default(),
            projection,
            camera_config,
            MainCamera,
            Transform::from_xyz(0., 1., 20.).looking_at(Vec3::ZERO, Vec3::Y),
            RenderLayers::from_layers(&MY_CAMERA_RENDER_LAYER),
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

        // Spawn the camera
        commands.spawn((camera, Visibility::Hidden));
        commands.insert_resource(GameLoaded);
        commands.set_state(AppState::InGame);
    }

    fn sun_cycle(mut suns: Query<&mut Transform, With<Sun>>, time: Res<Time>) {
        // For each sun, rotate them around X-axis
        suns.iter_mut().for_each(|mut tf| {
            tf.rotate_x(-time.delta_secs() * PI / 12.0);
        });
    }

    fn hide_scene(mut cameras: Query<(&mut Camera, &mut Visibility), With<MainCamera>>) {
        for (mut cam, mut visibility) in cameras.iter_mut() {
            cam.is_active = false;
            *visibility = Visibility::Hidden;
        }
    }

    fn show_scene(mut cameras: Query<(&mut Camera, &mut Visibility), With<MainCamera>>) {
        for (mut cam, mut visibility) in cameras.iter_mut() {
            cam.is_active = true;
            *visibility = Visibility::Visible;
        }
    }

    fn setup_audio(
        mut commands: Commands,
        mut audio: ResMut<audio::Audio>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let simulation_flags =
            audionimbus::SimulationFlags::DIRECT | audionimbus::SimulationFlags::REFLECTIONS;
        let source = audionimbus::Source::try_new(
            &audio.simulator,
            &audionimbus::SourceSettings {
                flags: simulation_flags,
            },
        )
        .unwrap();
        audio.simulator.add_source(&source);
        let source = audionimbus::Source::try_new(
            &audio.simulator,
            &audionimbus::SourceSettings {
                flags: simulation_flags,
            },
        )
        .unwrap();
        audio.simulator.add_source(&source);
        audio.simulator.commit();

        let mut spawn_position = Vec3::new(0.0, 0.0, 10.0);
        let mut viewpoint = player::Viewpoint {
            translation: Vec3::new(0.0, 2.0, 0.0),
            ..Default::default()
        };

        let samples = audio::sine_wave(440., 48000, 0.5, 48000 * 2 );

        let sphere = meshes.add(Sphere { radius: 0.1 });
        let sphere_material = materials.add(StandardMaterial {
            emissive: LinearRgba {
                red: 0.0,
                green: 0.0,
                blue: 1000.0,
                alpha: 1.0,
            },
            ..default()
        });

        let source_position = Transform::from_xyz(0.0, 2.0, 0.0);
        commands.spawn((
            Mesh3d(sphere.clone()),
            MeshMaterial3d(sphere_material.clone()),
            source_position,
            audio::AudioSource {
                source,
                data: samples,
                is_repeating: true,
                position: 0,
            },
        ));

        spawn_position = Vec3::new(0.0, 0.0, 15.0);



        audio.scene.commit();


    }
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), Self::setup_audio);


        app.add_systems(
            OnEnter(AppState::LoadingScreen),
            (Self::lights, Self::camera).chain(),
        );
        app.add_systems(OnEnter(AppState::InGame), Self::show_scene);
        app.add_systems(OnExit(AppState::InGame), Self::hide_scene);
        app.add_systems(Update, Self::sun_cycle.run_if(in_state(AppState::InGame)));
    }
}

// Our Main Camera tag
#[derive(Component)]
struct MainCamera;

// Our Sun tag
#[derive(Component)]
struct Sun;

#[derive(Resource)]
pub(super) struct GameLoaded;
