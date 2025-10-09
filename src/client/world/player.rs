use std::path::Path;

use bevy::{
    camera::visibility::{NoFrustumCulling, RenderLayers},
    prelude::*,
    scene::SceneInstanceReady,
};

use crate::client::LAYER_WORLD;

pub struct PlayerPlugin;

#[derive(Resource, Clone)]
struct _PlayerSceneLoaded;

#[derive(Clone, Component)]
pub struct Player;

#[derive(Clone, Component)]
struct _GltfModel;

#[derive(Component, Default, Debug)]
pub struct Viewpoint {
    pub translation: Vec3,
    pub rotation: Quat,
    pub pitch: f32,
    pub yaw: f32,
}

impl PlayerPlugin {
    fn _load_gltf(asset_server: Res<AssetServer>, mut commands: Commands) {
        let scene_handle: Handle<Scene> = asset_server
            .load(GltfAssetLabel::Scene(0).from_asset(Path::new("models").join("malfoy.glb")));

        commands.spawn((
            Player,
            _GltfModel,
            Viewpoint::default(),
            SceneRoot(scene_handle),
            RenderLayers::from_layers(&[LAYER_WORLD]),
            Transform::from_xyz(0., 1., 0.)
                .looking_at(Vec3::Z, Vec3::Y)
                .with_scale(Vec3::splat(1.0)),
            NoFrustumCulling,
        ));
    }

    pub fn spawn_player(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let player_skin: Handle<Image> = asset_server.load(Path::new("images").join("malfoy.png"));

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(360. / 400., 640. / 400.)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(player_skin.clone()),
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                emissive_texture: Some(player_skin.clone()),
                thickness: 5.,
                ..Default::default()
            })),
            Transform::from_xyz(0., 3., 0.)
                .looking_at(Vec3::Y * 3., Vec3::Y)
                .with_scale(Vec3::splat(1.)),
            RenderLayers::from_layers(&[LAYER_WORLD]),
            Player,
            Viewpoint::default(),
            NoFrustumCulling,
        ));
    }

    fn _change_render_layers(
        _trigger: On<SceneInstanceReady>,
        mut commands: Commands,
        query: Query<Entity, With<_GltfModel>>,
    ) {
        for entity in query.iter() {
            commands
                .entity(entity)
                .insert_recursive::<Children>(RenderLayers::from_layers(&[LAYER_WORLD]));
        }
    }

    fn _change_material(
        trigger: On<SceneInstanceReady>,
        material: Query<&MeshMaterial3d<StandardMaterial>>,
        mut asset_materials: ResMut<Assets<StandardMaterial>>,
        children: Query<&mut Children>,
    ) {
        for entity in children.iter_descendants(trigger.entity) {
            if let Some(std_material) = material
                .get(entity)
                .ok()
                .and_then(|id| asset_materials.get_mut(id.id()))
            {
                std_material.metallic = 0.7;
                std_material.perceptual_roughness = 0.5;
                std_material.cull_mode = None;
            }
        }
    }

    fn _set_resource(
        trigger: On<SceneInstanceReady>,
        mut commands: Commands,
        query: Query<&_GltfModel>,
    ) {
        let observed = trigger.entity;

        if query.get(observed).is_ok() {
            commands.insert_resource(_PlayerSceneLoaded);
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, Self::load_gltf);
        app.add_systems(Startup, Self::spawn_player);
        // app.add_observer(Self::set_resource);
        // app.add_observer(Self::change_material);
        // app.add_observer(Self::change_render_layers);
    }
}
