# svo-rs

> :warning: **Work in progress** I would call it almost feature complete. It's however untested in real world scenarios. :warning:

Sparse Voxel Octree (SVO) implementation in Rust based on [3D Flight Navigation Using
Sparse Voxel Octrees](https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter21_3D_Flight_Navigation_Using_Sparse_Voxel_Octrees.pdf) with integration for the [Bevy engine](https://bevyengine.org/) under the `bevy` feature.

This crate contains only the SVO data structure with a builder and an algorithm to voxelize meshes from Bevy. It does not contain any pathfinding code as there are much better crates for that. Also depending on your 3D environment you might want to use different pathfinding algorithms.

## Usage

Example using the [pathfinding crate](https://crates.io/crates/pathfinding).

```rust
let mut builder = svo_rs::SparseVoxelOctreeBuilder::new(voxel_size);
builder.add_mesh(VoxelizedMesh::sphere(1.0, voxel_size, IPoint::ZERO));

let tree = builder.build()

let start = tree
    .find_node(FPoint::new(...))
    .unwrap();
let end = tree
    .find_node(FPoint::new(...))
    .unwrap();

let solution = pathfinding::prelude::astar(
    &start,
    |n| {
        tree.successors(*n)
            .into_iter()
            .map(|s| (s, 1))
            .collect::<Vec<_>>()
    },
    |n| n.manhattan_distance(&end, &tree),
    |n| *n == end,
);
```

For more examples see the examples directory or the benches directory.

## Credit and References

### 3D Flight Navigation Using Sparse Voxel Octrees

```
https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter21_3D_Flight_Navigation_Using_Sparse_Voxel_Octrees.pdf
```

### Morton Code library

The library was not used directly it was rather used as a reference for the implementation of the morton code generation.

```
@Misc{libmorton18,
author = "Jeroen Baert",
title = "Libmorton: C++ Morton Encoding/Decoding Library",
howpublished = "\url{https://github.com/Forceflow/libmorton}",
year = "2018"}
```

### AABB Triangle intersection

```
https://gdbooks.gitbooks.io/3dcollisions/content/Chapter4/aabb-triangle.html
```