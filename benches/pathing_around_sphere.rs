use std::collections::HashSet;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use svo_rs::{FPoint, IPoint, SparseVoxelOctree, VoxelizedMesh};

fn compute_path_hashset(points: &HashSet<IPoint>, voxel_size: f32, area_haf_size: f32) {
    let start = (FPoint::new(-area_haf_size, -area_haf_size, -area_haf_size) / voxel_size).to_i32();
    let end = (FPoint::new(area_haf_size, area_haf_size, area_haf_size) / voxel_size).to_i32();

    let solution = pathfinding::prelude::astar(
        &start,
        |n| {
            vec![
                *n + IPoint::new(1, 0, 0),
                *n + IPoint::new(-1, 0, 0),
                *n + IPoint::new(0, 1, 0),
                *n + IPoint::new(0, -1, 0),
                *n + IPoint::new(0, 0, 1),
                *n + IPoint::new(0, 0, -1),
            ]
            .into_iter()
            .filter(|p| !points.contains(p))
            .map(|s| (s, (s - *n).manhattan_length()))
            .collect::<Vec<_>>()
        },
        |n| (*n - end).manhattan_length(),
        |n| *n == end,
    );

    if solution.is_none() {
        println!("No path found");
        println!("Start: {:?}", start);
        println!("Destination: {:?}", end);
    }
}

fn compute_path_octree(tree: &SparseVoxelOctree, area_haf_size: f32) {
    let start = tree
        .find_node(FPoint::new(-area_haf_size, -area_haf_size, -area_haf_size))
        .unwrap();
    let end = tree
        .find_node(FPoint::new(area_haf_size, area_haf_size, area_haf_size))
        .unwrap();

    let solution = pathfinding::prelude::astar(
        &start,
        |n| {
            tree.successors(*n)
                .into_iter()
                .map(|s| (s, 1))
                .collect::<Vec<_>>()
        },
        |n| n.manhattan_distance(&end, tree),
        |n| *n == end,
    );

    if solution.is_none() {
        println!("No path found");
        println!("Start: {:?}", start);
        println!("Destination: {:?}", end);
    }
}

fn create_tree(voxel_size: f32, area_haf_size: f32) -> SparseVoxelOctree {
    let mut builder = svo_rs::SparseVoxelOctreeBuilder::new(voxel_size);

    builder.add_mesh(VoxelizedMesh::sphere(1.0, voxel_size, IPoint::ZERO));
    builder.set_bounds(
        FPoint::new(-area_haf_size, -area_haf_size, -area_haf_size) * 1.2,
        FPoint::new(area_haf_size, area_haf_size, area_haf_size) * 1.2,
    );

    builder.build()
}

fn create_hashset(voxel_size: f32) -> HashSet<IPoint> {
    let mut set = HashSet::new();
    for voxel in VoxelizedMesh::sphere(1.0, voxel_size, IPoint::ZERO).voxels() {
        set.insert(voxel);
    }

    set
}

fn from_elem(c: &mut Criterion) {
    let voxel_sizes = [0.1, 0.05, 0.01];
    let area_haf_sizes = [2.0, 4.0, 40.0, 400.0];

    for voxel_size in voxel_sizes.iter() {
        for area_haf_size in area_haf_sizes.iter() {
            let tree = create_tree(*voxel_size, *area_haf_size);
            let hash_set = create_hashset(*voxel_size);

            let mut group = c.benchmark_group("Compute path");
            group.bench_with_input(
                BenchmarkId::new("octree", format!("{} {}", voxel_size, area_haf_size)),
                &tree,
                |b, t| {
                    b.iter(|| compute_path_octree(black_box(t), *area_haf_size));
                },
            );
            group.bench_with_input(
                BenchmarkId::new("hashset", format!("{} {}", voxel_size, area_haf_size)),
                &hash_set,
                |b, hs| {
                    b.iter(|| compute_path_hashset(black_box(hs), *voxel_size, *area_haf_size));
                },
            );
            group.finish();
        }
    }
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
