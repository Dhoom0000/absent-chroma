use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::{NoFrustumCulling, RenderLayers},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use noiz::{cell_noise::PerNearestPoint, prelude::*, rng::NoiseRng};

use crate::client::{AppState, LAYER_WORLD, world::LoadState};

pub struct ScenePlugin;

impl ScenePlugin {
    fn generate_terrain(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut load_state: ResMut<LoadState>,
    ) {
        let noise = Noise::<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>::default();

        let width = 100;
        let depth = 100;

        let scale = 0.1;

        let mut heights = vec![vec![0.0; depth]; width];

        for x in 0..width {
            for z in 0..depth {
                let nx = x as f32 * scale;
                let nz = z as f32 * scale;
                heights[x][z] = noise.sample(Vec2::new(nx, nz));
            }
        }

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        for x in 0..width {
            for z in 0..depth {
                positions.push([x as f32, heights[x][z], z as f32]);
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([x as f32 / width as f32, z as f32 / depth as f32]);
            }
        }

        for x in 0..width - 1 {
            for z in 0..depth - 1 {
                let i0 = (x * depth + z) as u32;
                let i1 = i0 + 1;
                let i2 = i0 + depth as u32;
                let i3 = i2 + 1;
                indices.extend_from_slice(&[i0, i2, i1, i1, i2, i3]);
            }
        }

        // Build mesh
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        // Spawn entity
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.8, 0.3),
                perceptual_roughness: 1.0,
                metallic: 0.0,
                ..default()
            })),
            Transform::from_xyz(-50.0, -10.0, -50.0),
            LAYER_WORLD,
        ));

        load_state.terrain = true;
    }
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Load), Self::generate_terrain);
    }
}
