use std::{f32::consts::PI, path::Path};

use avian3d::prelude::*;
use bevy::{
    camera::visibility::{NoFrustumCulling, RenderLayers},
    prelude::*,
    scene::SceneInstanceReady,
};

use crate::client::{AppState, LAYER_PLAYER, LAYER_WORLD, world::LoadState};

pub struct PlayerPlugin;

#[derive(Clone, Component)]
pub struct Player;

// #[derive(Resource)]
// struct MyScene(pub Handle<Scene>);

#[derive(Component, Default, Debug)]
pub struct Viewpoint {
    pub translation: Vec3,
    pub rotation: Quat,
    pub pitch: f32,
    pub yaw: f32,
}

impl PlayerPlugin {
    fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
        let scene = asset_server
            .load(GltfAssetLabel::Scene(0).from_asset(Path::new("models").join("malfoy.glb")));

        commands
            .spawn((
                SceneRoot(scene.clone()),
                Player,
                RigidBody::Dynamic,
                Transform::from_xyz(0.0, 0., 1.0)
                    .looking_at(Vec3::ZERO, Vec3::Y)
                    .with_scale(Vec3::splat(1.)),
                // Collider::capsule(1. / 2., 0.01),
                ColliderConstructorHierarchy::new(ColliderConstructor::Capsule {
                    radius: 0.01,
                    height: 0.01,
                }),
                LockedAxes::ROTATION_LOCKED,
            ))
            .with_children(|parent| {
                parent.spawn((
                    PointLight {
                        intensity: 100_000.,
                        color: Color::srgba(1.0, 0.55, 0.0, 1.),
                        range: 1_000.,
                        shadows_enabled: true,
                        ..Default::default()
                    },
                    RenderLayers::layer(LAYER_PLAYER),
                    Transform::from_xyz(-1.5, 0.2, 0.0),
                ));
            });
    }

    fn edit_scene(
        observer: On<SceneInstanceReady>,
        mut commands: Commands,
        mut load_state: ResMut<LoadState>,
        children_query: Query<&Children>,
    ) {
        fn set_layer_recursive(
            commands: &mut Commands,
            children_query: &Query<&Children>,
            entity: Entity,
            layer: RenderLayers,
        ) {
            commands.entity(entity).insert((
                layer,
                // CollisionMargin(0.1),
            ));

            if let Ok(children) = children_query.get(entity) {
                for &child in children {
                    set_layer_recursive(
                        commands,
                        children_query,
                        child,
                        RenderLayers::layer(LAYER_PLAYER),
                    );
                }
            }
        }

        set_layer_recursive(
            &mut commands,
            &children_query,
            observer.entity,
            RenderLayers::layer(LAYER_PLAYER),
        );

        load_state.player = true;
    }

    fn rotate_player(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
        for mut transform in query.iter_mut() {
            transform.rotate_y(time.delta_secs() * (PI / 12.));
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Load), Self::load_gltf);
        app.add_systems(
            Update,
            Self::rotate_player.run_if(in_state(AppState::InGame)),
        );
        app.add_observer(Self::edit_scene);
    }
}
