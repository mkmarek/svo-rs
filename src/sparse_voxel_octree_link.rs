use crate::{SparseVoxelOctree, morton_code::MortonCode};

/// A link to a node in the sparse voxel octree.
/// 
/// # Examples
/// 
/// ```
/// use svo_rs::{SparseVoxelOctreeLink, SparseVoxelOctreeBuilder, VoxelizedMesh, IPoint, UPoint, FPoint};
///
/// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
///
/// builder.add_mesh(VoxelizedMesh::new(vec![UPoint::new(0, 0, 0), UPoint::new(0, 3, 0)], 1.0, IPoint::ZERO));
///
/// let octree = builder.build();
/// let node = octree.find_node(FPoint::new(0.0, 3.0, 0.0)).unwrap();
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
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh, IPoint, UPoint, FPoint};
    /// 
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    /// 
    /// builder.add_mesh(VoxelizedMesh::new(vec![UPoint::new(0, 0, 0), UPoint::new(0, 3, 0)], 1.0, IPoint::ZERO));
    /// 
    /// let octree = builder.build();
    /// let node = octree.find_node(FPoint::new(0.0, 3.0, 0.0)).unwrap();
    /// let other = octree.find_node(FPoint::new(0.0, 0.0, 0.0)).unwrap();
    /// 
    /// assert_eq!(node.manhattan_distance(&other, &octree), 3);
    pub fn manhattan_distance(&self, other: &Self, tree: &SparseVoxelOctree) -> i32 {
        let node = &tree.layers[self.layer_index][self.node_index];
        let other_node = &tree.layers[other.layer_index][other.node_index];

        let mut node_position = node.position.to_i32() * 4;
        if let Some(subnode_index) = self.subnode_index {
            node_position += MortonCode::from_u8(subnode_index).decode().to_i32();
        }

        let mut other_node_position = other_node.position.to_i32() * 4;
        if let Some(subnode_index) = other.subnode_index {
            other_node_position += MortonCode::from_u8(subnode_index).decode().to_i32();
        }

        (node_position - other_node_position).manhattan_length()
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
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh, IPoint, UPoint, FPoint};
    /// 
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    /// 
    /// builder.add_mesh(VoxelizedMesh::new(vec![UPoint::new(0, 0, 0), UPoint::new(0, 3, 0)], 1.0, IPoint::ZERO));
    /// 
    /// let octree = builder.build();
    /// let node = octree.find_node(FPoint::new(0.0, 3.0, 0.0)).unwrap();
    /// let other = octree.find_node(FPoint::new(0.0, 0.0, 0.0)).unwrap();
    /// 
    /// assert_eq!(node.distance_squared(&other, &octree), 9);
    /// ```
    pub fn distance_squared(&self, other: &Self, tree: &SparseVoxelOctree) -> i32 {
        let node = &tree.layers[self.layer_index][self.node_index];
        let other_node = &tree.layers[other.layer_index][other.node_index];

        let mut node_position = node.position.to_i32() * 4;
        if let Some(subnode_index) = self.subnode_index {
            node_position += MortonCode::from_u8(subnode_index).decode().to_i32();
        }

        let mut other_node_position = other_node.position.to_i32() * 4;
        if let Some(subnode_index) = other.subnode_index {
            other_node_position += MortonCode::from_u8(subnode_index).decode().to_i32();
        }

        (node_position - other_node_position).length_squared()
    }
}
