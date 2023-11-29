//! Sparse Voxel Octree (SVO) implementation in Rust based on `3D Flight Navigation Using Sparse Voxel Octrees`
//! with integration for the Bevy engine under the bevy feature.
//!
//! [`3D Flight Navigation Using Sparse Voxel Octrees`]: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter21_3D_Flight_Navigation_Using_Sparse_Voxel_Octrees.pdf

#![warn(clippy::pedantic)]

mod cohen_sutherland;
mod compound_node;
mod consts;
mod morton_code;
mod point;
mod sparse_voxel_octree;
mod sparse_voxel_octree_builder;
mod sparse_voxel_octree_link;
mod sparse_voxel_octree_node;
mod voxelized_mesh;

#[cfg(not(feature = "bevy"))]
mod bevy_vec {
    pub use bevy_math::{IVec3, UVec3, Vec3};
}

#[cfg(feature = "bevy")]
mod bevy_vec {}

pub use bevy_vec::*;
pub use point::DistanceSquared;
pub use point::ManhattanDistance;
pub use sparse_voxel_octree::SparseVoxelOctree;
pub use sparse_voxel_octree_builder::SparseVoxelOctreeBuilder;
pub use sparse_voxel_octree_link::SparseVoxelOctreeLink;
pub use voxelized_mesh::VoxelizeError;
pub use voxelized_mesh::VoxelizedMesh;
