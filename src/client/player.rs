use bevy::{
    math::VectorSpace,
    pbr::UvChannel,
    prelude::*,
    render::{render_resource::Face, view::RenderLayers},
    scene::{SceneInstance, SceneInstanceReady},
};

use crate::client::app::MY_WORLD_RENDER_LAYER;

#[derive(Component, Clone)]
pub struct Model;

pub(super) fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/malfoy.glb"));

    commands.spawn((
        Model,
        SceneRoot(scene),
        Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&MY_WORLD_RENDER_LAYER),
    ));
}

pub fn rotate_player(mut query: Query<&mut Transform, With<Model>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let rotation_const = 1.;
        transform.rotate(Quat::from_rotation_y(rotation_const * time.delta_secs()));
    }
}

pub fn add_render_layer(
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

pub fn change_material(
    trigger: Trigger<SceneInstanceReady>,
    children: Query<&Children>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
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
}
