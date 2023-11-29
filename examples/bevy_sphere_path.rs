use bevy::prelude::*;
use bevy_render::prelude::shape::UVSphere;
use svo_rs::{SparseVoxelOctree, SparseVoxelOctreeBuilder, VoxelizedMesh};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                calculate_path,
                follow_path,
                rotate_camera_around_center,
                draw_gizmos,
            ),
        )
        .run();
}

const VOXEL_SIZE: f32 = 0.05;
const AREA_HALF_SIZE: f32 = 2.0;

#[derive(Resource)]
struct SVOResource {
    tree: SparseVoxelOctree,
}

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct CalculatedPath {
    path: Vec<Vec3>,
    current: usize,
    progress: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        RotateCamera {
            distance: AREA_HALF_SIZE * 4.0,
        },
    ));

    let sphere = Mesh::from(UVSphere::default());

    let mut builder = SparseVoxelOctreeBuilder::new(VOXEL_SIZE);

    builder.set_bounds(
        Vec3::new(-AREA_HALF_SIZE, -AREA_HALF_SIZE, -AREA_HALF_SIZE),
        Vec3::new(AREA_HALF_SIZE, AREA_HALF_SIZE, AREA_HALF_SIZE),
    );
    builder.add_mesh(
        VoxelizedMesh::from_mesh(&sphere, Transform::IDENTITY.compute_matrix(), VOXEL_SIZE)
            .expect("Failed to voxelize mesh"),
    );

    commands.insert_resource(SVOResource {
        tree: builder.build(),
    });

    let sphere_handle = meshes.add(sphere);

    commands.spawn(PbrBundle {
        mesh: sphere_handle.clone(),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
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

    commands
        .spawn(PbrBundle {
            mesh: sphere_handle.clone(),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                AREA_HALF_SIZE * 0.9,
                AREA_HALF_SIZE * 0.9,
                AREA_HALF_SIZE * 0.9,
            )
            .with_scale(Vec3::ONE * 0.1),
            ..Default::default()
        })
        .insert(Agent);
}

#[must_use]
pub fn subdivide_path(link: Vec<Vec3>, n: usize) -> Vec<Vec3> {
    let mut result = Vec::new();

    for i in 0..link.len() - 1 {
        let start = link[i];
        let end = link[i + 1];

        for j in 0..n {
            result.push(start + (end - start) * (j as f32 / n as f32));
        }
    }

    result.push(link[link.len() - 1]);

    result
}

#[must_use]
pub fn string_pulling_path(link: Vec<Vec3>, tree: &SparseVoxelOctree) -> Vec<Vec3> {
    let mut result = Vec::new();

    let mut current = link[0];

    (1..link.len()).for_each(|i| {
        let next = link[i];

        if tree.is_in_line_of_sight(current, next) {
            return;
        }

        result.push(current);
        current = next;
    });

    result.push(current);

    result
}

#[allow(clippy::type_complexity)]
fn calculate_path(
    agents: Query<(Entity, &Transform), (Without<CalculatedPath>, With<Agent>)>,
    mut commands: Commands,
    svo: Res<SVOResource>,
) {
    for (entity, transform) in agents.iter() {
        let destination = (-transform.translation).normalize() * AREA_HALF_SIZE * 0.9;

        let start_point: Vec3 = transform.translation;
        let end_point: Vec3 = destination;

        let start = svo.tree.find_node(start_point).unwrap();
        let end = svo.tree.find_node(end_point).unwrap();

        if start == end {
            continue;
        }

        let solution = pathfinding::prelude::astar(
            &start,
            |n| {
                svo.tree
                    .successors(*n)
                    .into_iter()
                    .map(|s| (s, 1))
                    .collect::<Vec<_>>()
            },
            |n| n.distance_squared(&end, &svo.tree),
            |n| *n == end,
        );

        if solution.is_none() {
            println!("No path found");
            println!("Start: {start:?}");
            println!("Destination: {end:?}");
            continue;
        }

        let solution = solution.unwrap();

        let mut path = vec![start_point];

        for i in 0..solution.0.len() - 1 {
            let start = solution.0[i];
            let end = solution.0[i + 1];

            let face = svo
                .tree
                .face_position_between(start, end)
                .unwrap_or_else(|| panic!("Failed to find face between nodes {start:?} {end:?}"));

            path.push(face);
        }

        path.push(end_point);

        let path = subdivide_path(path, 4);
        let path = string_pulling_path(path, &svo.tree);

        commands.entity(entity).insert(CalculatedPath {
            path: path.into_iter().map(std::convert::Into::into).collect(),
            current: 0,
            progress: 0.0,
        });
    }
}

fn follow_path(
    mut agents: Query<(Entity, &mut Transform, &mut CalculatedPath), With<Agent>>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    for (entity, mut transform, mut path) in &mut agents {
        if !path.path.is_empty() {
            if path.current >= path.path.len() - 1 {
                commands.entity(entity).remove::<CalculatedPath>();
                continue;
            }

            let current = path.path[path.current];
            let next = path.path[path.current + 1];
            let distance = (next - current).length();

            transform.translation = current.lerp(next, path.progress / distance);

            let speed = AREA_HALF_SIZE;
            if path.progress > distance {
                path.progress = 0.0;
                path.current += 1;
                if path.current >= path.path.len() - 1 {
                    commands.entity(entity).remove::<CalculatedPath>();
                }
            }

            path.progress += time.delta_seconds() * speed;

            for i in 0..path.path.len() - 1 {
                gizmos.line(path.path[i], path.path[i + 1], Color::rgb(1.0, 1.0, 1.0));
            }

            // gizmos.line(path.start, path.end, Color::rgb(1.0, 0.0, 0.0));
        }
    }
}

#[derive(Component)]
struct RotateCamera {
    distance: f32,
}

fn rotate_camera_around_center(
    agents: Query<&Transform, (With<Agent>, Without<Camera>)>,
    mut camera: Query<(&mut Transform, &RotateCamera), With<Camera>>,
) {
    let agent_transform = agents.get_single();
    if agent_transform.is_err() {
        return;
    }

    let agent_transform = agent_transform.unwrap();

    let camera = camera.get_single_mut();

    if camera.is_err() {
        return;
    }

    let (mut camera_transform, rotate_camera) = camera.unwrap();

    let direction = agent_transform.translation.normalize();

    camera_transform.translation = agent_transform.translation + direction * rotate_camera.distance;
    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}

fn draw_gizmos(mut _gizmos: Gizmos, _svo: Res<SVOResource>) {
    // svo.tree.draw_gizmos(&mut gizmos, false);
}
