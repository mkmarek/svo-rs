use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use svo_rs::{SparseVoxelOctree, SparseVoxelOctreeBuilder, VoxelizedMesh};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(WorldInspectorPlugin::new())
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
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let sphere = Sphere::new(1.0).mesh().uv(32, 18);

    let mut builder = SparseVoxelOctreeBuilder::new(VOXEL_SIZE);
    builder.add_mesh(
        VoxelizedMesh::from_mesh(&sphere, Transform::IDENTITY.compute_matrix(), VOXEL_SIZE)
            .expect("Failed to voxelize mesh"),
    );

    commands.insert_resource(SVOResource {
        tree: builder.build(),
    });

    commands.spawn((
        Mesh3d(meshes.add(sphere)),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.4, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y), // Point directly at sphere
    ));
}

fn draw_gizmos(mut gizmos: Gizmos, svo: Res<SVOResource>) {
    svo.tree.draw_gizmos(&mut gizmos, false);
}
