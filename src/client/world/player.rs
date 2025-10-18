use std::{f32::consts::PI, path::Path};

use bevy::{
    camera::visibility::{NoFrustumCulling, RenderLayers},
    gltf::GltfMesh,
    prelude::*,
    scene::SceneInstanceReady,
};

use crate::client::{AppState, LAYER_PLAYER, LAYER_WORLD, world::LoadState};

pub struct PlayerPlugin;

#[derive(Clone, Component)]
pub struct Player;

#[derive(Resource)]
struct MyScene(pub Handle<Scene>);

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

        commands.spawn((
            SceneRoot(scene.clone()),
            Player,
            Transform::from_xyz(0.0, 10., 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y)
                .with_scale(Vec3::splat(1.)),
        ));

        commands.insert_resource(MyScene(scene));
    }

    fn edit_scene(
        observer: On<SceneInstanceReady>,
        mut commands: Commands,
        mut load_state: ResMut<LoadState>,
    ) {
        commands
            .entity(observer.entity)
            .insert(LAYER_PLAYER)
            .insert_recursive::<Children>(LAYER_PLAYER);

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
