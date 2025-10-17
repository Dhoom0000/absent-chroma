use std::path::Path;

use bevy::{
    camera::visibility::{NoFrustumCulling, RenderLayers},
    prelude::*,
    scene::SceneInstanceReady,
};

use crate::client::{LAYER_PLAYER, LAYER_WORLD};

pub struct PlayerPlugin;

#[derive(Clone, Component)]
pub struct Player;

#[derive(Component, Default, Debug)]
pub struct Viewpoint {
    pub translation: Vec3,
    pub rotation: Quat,
    pub pitch: f32,
    pub yaw: f32,
}

impl PlayerPlugin {
    fn load_gltf() {}
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, Self::load_gltf);
        // app.add_systems(Startup, Self::spawn_player);
        // app.add_observer(Self::set_resource);
        // app.add_observer(Self::change_material);
        // app.add_observer(Self::change_render_layers);
    }
}
