use bevy_math::UVec3;

use crate::{morton_code::MortonCode, sparse_voxel_octree_link::SparseVoxelOctreeLink};

#[derive(Debug)]
pub struct SparseVoxelOctreeNode {
    pub(crate) position: UVec3,
    pub(crate) size: u32,
    pub(crate) parent: Option<SparseVoxelOctreeLink>,
    pub(crate) first_child: Option<SparseVoxelOctreeLink>,
    pub(crate) is_leaf: bool,
    pub(crate) neighbors: [Option<SparseVoxelOctreeLink>; 6],
}

impl PartialEq for SparseVoxelOctreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for SparseVoxelOctreeNode {}

impl PartialOrd for SparseVoxelOctreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SparseVoxelOctreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_morton = MortonCode::encode(self.position);
        let other_morton = MortonCode::encode(other.position);

        self_morton.cmp(&other_morton)
    }
}

impl SparseVoxelOctreeNode {
    pub fn node(position: UVec3, size: u32) -> Self {
        Self {
            position,
            parent: None,
            first_child: None,
            is_leaf: false,
            size,
            neighbors: [None; 6],
        }
    }

    pub fn leaf(position: UVec3) -> Self {
        Self {
            position,
            parent: None,
            first_child: None,
            is_leaf: true,
            size: 4,
            neighbors: [None; 6],
        }
    }
}
