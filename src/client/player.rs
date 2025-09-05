use bevy::{prelude::*, render::view::RenderLayers, scene::SceneInstanceReady};

use crate::client::{AppState, MY_WORLD_RENDER_LAYER};

#[derive(Component, Clone)]
pub struct Model;

pub struct PlayerPlugin;

impl PlayerPlugin {
    fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
        let scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/malfoy.glb"));

        commands.spawn((
            Model,
            SceneRoot(scene),
            Viewpoint {translation: Vec3::new(0.0, 0.0, 15.0),..Default::default()},
            Transform::from_xyz(0., 0., 15.).looking_at(Vec3::ZERO, Vec3::Y),
            RenderLayers::from_layers(&MY_WORLD_RENDER_LAYER),
        ));
    }

    fn rotate_player(mut query: Query<&mut Transform, With<Model>>, time: Res<Time>) {
        for mut transform in &mut query {
            let rotation_const = 1.;
            transform.rotate(Quat::from_rotation_y(rotation_const * time.delta_secs()));
        }
    }

    fn add_render_layer(
        children: Query<&Children>,
        scene: Query<Entity, With<Model>>,
        mut render_layers: Query<&mut RenderLayers>,
        mut commands: Commands,
    ) {
        for model_scene in scene {
            for entity in children.iter_descendants(model_scene) {
                if let Ok(mut render_layer) = render_layers.get_mut(entity) {
                    *render_layer = RenderLayers::from_layers(&MY_WORLD_RENDER_LAYER);
                } else {
                    commands
                        .entity(entity)
                        .insert(RenderLayers::from_layers(&MY_WORLD_RENDER_LAYER));
                }
            }
        }
    }

    fn change_material(
        trigger: Trigger<SceneInstanceReady>,
        children: Query<&Children>,
        mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
        mut asset_materials: ResMut<Assets<StandardMaterial>>,
        scene: Query<Entity, With<Model>>,
        mut render_layers: Query<&mut RenderLayers>,
        mut commands: Commands,
    ) {
        for descendants in children.iter_descendants(trigger.target()) {
            if let Some(material) = mesh_materials
                .get(descendants)
                .ok()
                .and_then(|id| asset_materials.get_mut(id.id()))
            {
                material.metallic = 1.;
                material.cull_mode = None;
            }
        }
        for model_scene in scene {
            for entity in children.iter_descendants(model_scene) {
                if let Ok(mut render_layer) = render_layers.get_mut(entity) {
                    *render_layer = RenderLayers::from_layers(&MY_WORLD_RENDER_LAYER);
                } else {
                    commands
                        .entity(entity)
                        .insert(RenderLayers::from_layers(&MY_WORLD_RENDER_LAYER));
                }
            }
        }
    }

    fn sync_viewpoint_transform(mut query: Query<(&mut Viewpoint, &Transform), With<Model>>) {
        for (mut viewpoint, transform) in query.iter_mut() {
            viewpoint.translation = transform.translation;
            viewpoint.rotation = transform.rotation;
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadingScreen), Self::spawn_player);
        app.add_systems(
            Update,
            (Self::rotate_player).run_if(in_state(AppState::InGame)),
        );
        app.add_systems(Update, Self::sync_viewpoint_transform);
        app.add_observer(Self::change_material);
    }
}

#[derive(Component, Default, Debug)]
pub struct Viewpoint {
    pub translation: Vec3,
    pub rotation: Quat,
    pub pitch: f32,
    pub yaw: f32,
}
