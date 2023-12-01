use bevy_math::UVec3;

use crate::{
    consts::SUBNODE_POSITIONS, point::ManhattanDistance, DistanceSquared, SparseVoxelOctree,
};

/// A link to a node in the sparse voxel octree.
///
/// # Examples
///
/// ```
/// use svo_rs::{SparseVoxelOctreeLink, SparseVoxelOctreeBuilder, VoxelizedMesh};
/// use bevy_math::{IVec3, UVec3, Vec3};
///
/// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
///
/// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 0, 0), UVec3::new(0, 3, 0)], 1.0, IVec3::ZERO));
///
/// let octree = builder.build();
/// let node = octree.find_node(Vec3::new(0.0, 3.0, 0.0)).unwrap();
///
/// assert_eq!(node, SparseVoxelOctreeLink::new(0, 0, Some(18)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SparseVoxelOctreeLink {
    /// The index of the layer the node is in.
    pub(crate) layer_index: usize,
    /// The index of the node in the layer.
    pub(crate) node_index: usize,
    /// The index of the subnode in a 4x4x4 cube in a Morton code. This is applicable only for leaf nodes.
    pub(crate) subnode_index: Option<u8>,
}

impl SparseVoxelOctreeLink {
    /// Creates a new link to a node in the sparse voxel octree.
    ///
    /// # Arguments
    ///
    /// * `layer_index` - The index of the layer the node is in.
    /// * `node_index` - The index of the node in the layer.
    /// * `subnode_index` - The index of the subnode in a 4x4x4 cube in a Morton code. This is applicable only for leaf nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use svo_rs::SparseVoxelOctreeLink;
    ///
    /// let link = SparseVoxelOctreeLink::new(0, 0, None);
    /// ```
    #[must_use]
    pub fn new(layer_index: usize, node_index: usize, subnode_index: Option<u8>) -> Self {
        Self {
            layer_index,
            node_index,
            subnode_index,
        }
    }

    /// Manhattan distance between two nodes. This distance does not take voxel size into account.
    ///
    /// # Arguments
    ///
    /// * `other` - The other node.
    /// * `tree` - The sparse voxel octree.
    ///
    /// # Examples
    ///
    /// ```
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh};
    /// use bevy_math::{IVec3, UVec3, Vec3};
    ///
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    ///
    /// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 0, 0), UVec3::new(0, 3, 0)], 1.0, IVec3::ZERO));
    ///
    /// let octree = builder.build();
    /// let node = octree.find_node(Vec3::new(0.0, 3.0, 0.0)).unwrap();
    /// let other = octree.find_node(Vec3::new(0.0, 0.0, 0.0)).unwrap();
    ///
    /// assert_eq!(node.manhattan_distance(&other, &octree), 3);
    #[must_use]
    pub fn manhattan_distance(&self, other: &Self, tree: &SparseVoxelOctree) -> u32 {
        let node = &tree.layers[self.layer_index][self.node_index];
        let other_node = &tree.layers[other.layer_index][other.node_index];

        let mut node_position = node.position * 4;
        if let Some(subnode_index) = self.subnode_index {
            let subnode_position = UVec3::new(
                SUBNODE_POSITIONS[subnode_index as usize].0.into(),
                SUBNODE_POSITIONS[subnode_index as usize].1.into(),
                SUBNODE_POSITIONS[subnode_index as usize].2.into(),
            );
            node_position += subnode_position;
        }

        let mut other_node_position = other_node.position * 4;
        if let Some(subnode_index) = other.subnode_index {
            let subnode_position = UVec3::new(
                SUBNODE_POSITIONS[subnode_index as usize].0.into(),
                SUBNODE_POSITIONS[subnode_index as usize].1.into(),
                SUBNODE_POSITIONS[subnode_index as usize].2.into(),
            );
            other_node_position += subnode_position;
        }

        node_position.manhattan_distance(&other_node_position)
    }

    /// Euclidean distance squared between two nodes. This distance does not take voxel size into account.
    ///
    /// # Arguments
    ///
    /// * `other` - The other node.
    /// * `tree` - The sparse voxel octree.
    ///
    /// # Examples
    ///
    /// ```
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh};
    /// use bevy_math::{IVec3, UVec3, Vec3};
    ///
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    ///
    /// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 0, 0), UVec3::new(0, 3, 0)], 1.0, IVec3::ZERO));
    ///
    /// let octree = builder.build();
    /// let node = octree.find_node(Vec3::new(0.0, 3.0, 0.0)).unwrap();
    /// let other = octree.find_node(Vec3::new(0.0, 0.0, 0.0)).unwrap();
    ///
    /// assert_eq!(node.distance_squared(&other, &octree), 9);
    /// ```
    #[must_use]
    pub fn distance_squared(&self, other: &Self, tree: &SparseVoxelOctree) -> u32 {
        let node = &tree.layers[self.layer_index][self.node_index];
        let other_node = &tree.layers[other.layer_index][other.node_index];

        let mut node_position = node.position * 4;
        if let Some(subnode_index) = self.subnode_index {
            let subnode_position = UVec3::new(
                SUBNODE_POSITIONS[subnode_index as usize].0.into(),
                SUBNODE_POSITIONS[subnode_index as usize].1.into(),
                SUBNODE_POSITIONS[subnode_index as usize].2.into(),
            );
            node_position += subnode_position;
        }

        let mut other_node_position = other_node.position * 4;
        if let Some(subnode_index) = other.subnode_index {
            let subnode_position = UVec3::new(
                SUBNODE_POSITIONS[subnode_index as usize].0.into(),
                SUBNODE_POSITIONS[subnode_index as usize].1.into(),
                SUBNODE_POSITIONS[subnode_index as usize].2.into(),
            );
            other_node_position += subnode_position;
        }

        (node_position).distance_squared(&other_node_position)
    }
}
