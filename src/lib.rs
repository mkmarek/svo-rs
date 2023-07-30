//! Sparse Voxel Octree (SVO) implementation in Rust based on `3D Flight Navigation Using Sparse Voxel Octrees`
//! with integration for the Bevy engine under the bevy feature.
//!
//! [`3D Flight Navigation Using Sparse Voxel Octrees`]: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter21_3D_Flight_Navigation_Using_Sparse_Voxel_Octrees.pdf

mod voxelized_mesh;
mod sparse_voxel_octree;
mod morton_code;
mod point;
mod sparse_voxel_octree_builder;
mod sparse_voxel_octree_link;
mod sparse_voxel_octree_node;
mod consts;
mod compound_node;
mod cohen_sutherland;

pub use voxelized_mesh::VoxelizeError;
pub use voxelized_mesh::VoxelizedMesh;
pub use sparse_voxel_octree::SparseVoxelOctree;
pub use sparse_voxel_octree_builder::SparseVoxelOctreeBuilder;
pub use sparse_voxel_octree_link::SparseVoxelOctreeLink;
pub use point::FPoint;
pub use point::UPoint;
pub use point::IPoint;
pub use point::Point;