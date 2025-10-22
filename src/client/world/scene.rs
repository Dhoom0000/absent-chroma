use avian3d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::{NoFrustumCulling, RenderLayers},
    light::FogVolume,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::render_resource::Face,
};
use noiz::{cell_noise::PerNearestPoint, prelude::*, rng::NoiseRng};

use crate::client::{AppState, LAYER_PLAYER, LAYER_WORLD, world::LoadState};

pub struct ScenePlugin;

impl ScenePlugin {
    fn generate_terrain(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut load_state: ResMut<LoadState>,
    ) {
        let noise =
            Noise::<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients, true>>::default();

        let width = 100;
        let depth = 100;

        let scale = 0.1;

        let mut heights = vec![0.0; width * depth];
        for x in 0..width {
            for z in 0..depth {
                let nx = x as f32 * scale;
                let nz = z as f32 * scale;
                heights[x * depth + z] = noise.sample(Vec2::new(nx, nz));
            }
        }

        let mut positions = Vec::with_capacity(width * depth);
        let mut normals = Vec::with_capacity(width * depth);
        let mut uvs = Vec::with_capacity(width * depth);
        let mut indices = Vec::with_capacity((width - 1) * (depth - 1) * 6);

        for x in 0..width {
            for z in 0..depth {
                let h = heights[x * depth + z];
                positions.push([x as f32, h, z as f32]);

                // Central differences for smooth normals
                let h_l = if x > 0 {
                    heights[(x - 1) * depth + z]
                } else {
                    h
                };
                let h_r = if x < width - 1 {
                    heights[(x + 1) * depth + z]
                } else {
                    h
                };
                let h_d = if z > 0 {
                    heights[x * depth + (z - 1)]
                } else {
                    h
                };
                let h_u = if z < depth - 1 {
                    heights[x * depth + (z + 1)]
                } else {
                    h
                };

                let dx = h_r - h_l;
                let dz = h_u - h_d;
                let normal = Vec3::new(-dx, 2.0, -dz).normalize();
                normals.push([normal.x, normal.y, normal.z]);

                uvs.push([x as f32 / width as f32, z as f32 / depth as f32]);
            }
        }

        for x in 0..width - 1 {
            for z in 0..depth - 1 {
                let i0 = (x * depth + z) as u32;
                let i1 = (x * depth + z + 1) as u32;
                let i2 = ((x + 1) * depth + z) as u32;
                let i3 = ((x + 1) * depth + z + 1) as u32;

                indices.extend_from_slice(&[i0, i1, i2, i1, i3, i2]);
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
                base_color: Color::srgba(0.4, 0.8, 0.3, 1.),
                perceptual_roughness: 1.,
                depth_bias: -100.,
                alpha_mode: AlphaMode::Premultiplied,
                cull_mode: None,
                ..default()
            })),
            Transform::from_xyz(-(width as f32) / 2.0, -1.0, -(depth as f32) / 2.0),
            RenderLayers::layer(LAYER_WORLD),
            RigidBody::Static,
            ColliderConstructor::TrimeshFromMesh,
            CollisionMargin(0.1),
        ));

        load_state.terrain = true;
    }
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Load), Self::generate_terrain);
    }
}
