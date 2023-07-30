use bevy::prelude::*;
use bevy_render::prelude::shape::UVSphere;
use svo_rs::{SparseVoxelOctree, SparseVoxelOctreeBuilder, VoxelizedMesh};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gizmos)
        .run();
}

const VOXEL_SIZE: f32 = 0.1;

#[derive(Resource)]
struct SVOResource {
    tree: SparseVoxelOctree,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    },));

    let sphere = Mesh::from(UVSphere::default());

    let mut builder = SparseVoxelOctreeBuilder::new(VOXEL_SIZE);
    builder.add_mesh(
        VoxelizedMesh::from_mesh(&sphere, Transform::IDENTITY.compute_matrix(), VOXEL_SIZE)
            .expect("Failed to voxelize mesh"),
    );

    commands.insert_resource(SVOResource {
        tree: builder.build(),
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(sphere),
        material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    svo: Res<SVOResource>,
) {
    svo.tree.draw_gizmos(&mut gizmos, false);
}