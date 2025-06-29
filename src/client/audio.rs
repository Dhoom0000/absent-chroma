use bevy::prelude::*;

pub struct Plugin;

impl Plugin {
    fn spawn_source_sphere(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0., 1., 0.)))),
        ));
    }
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {}
}
