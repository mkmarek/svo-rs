use std::collections::HashMap;

use bevy_math::{IVec3, UVec3, Vec3};

use crate::{
    compound_node::CompoundNode,
    consts::{NEIGHBOR_CONNECTIONS, OFFSETS_IN_MORTON_CODE_ORDER, SIBLING_CONNECTIONS},
    morton_code::MortonCode,
    sparse_voxel_octree_link::SparseVoxelOctreeLink,
    sparse_voxel_octree_node::SparseVoxelOctreeNode,
    voxelized_mesh::VoxelizedMesh,
    SparseVoxelOctree,
};

/// A builder for a sparse voxel octree.
///
/// This builder accepts voxelized meshes and builds a sparse voxel octree from them.
///
/// # Example
///
/// ```
/// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh};
/// use bevy_math::{IVec3, UVec3};
///
/// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
///
/// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 3, 0)], 1.0, IVec3::ZERO));
///
/// let octree = builder.build();
/// ```
pub struct SparseVoxelOctreeBuilder {
    voxel_size: f32,
    meshes: Vec<VoxelizedMesh>,
    min: IVec3,
    max: IVec3,
}

impl SparseVoxelOctreeBuilder {
    /// Creates a new builder.
    ///
    /// The voxel size is the size of a single voxel in world space.
    ///
    /// # Example
    ///
    /// ```
    /// use svo_rs::SparseVoxelOctreeBuilder;
    ///
    /// let builder = SparseVoxelOctreeBuilder::new(1.0);
    /// ```
    #[must_use]
    pub fn new(voxel_size: f32) -> Self {
        Self {
            meshes: Vec::new(),
            voxel_size,
            min: IVec3::MAX,
            max: IVec3::MIN,
        }
    }

    /// Adds a voxelized mesh to the builder.
    ///
    /// # Example
    ///
    /// ```
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh};
    /// use bevy_math::{IVec3, UVec3};
    ///
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    ///
    /// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 3, 0)], 1.0, IVec3::ZERO));
    /// ```
    pub fn add_mesh(&mut self, mesh: VoxelizedMesh) {
        self.meshes.push(mesh);
    }

    /// Sets the minimal bounds of the octree Bounds are specified in world space.
    /// If some of the meshes are outside of the bounds, then the bounds will be expanded to include them.
    /// The final bounds of the octree will be also extended to be a power of two.
    ///
    /// The bounds are specified in world space.
    ///
    /// # Example
    ///
    /// ```
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh};
    /// use bevy_math::{IVec3, UVec3, Vec3};
    ///
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    ///
    /// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 3, 0)], 1.0, IVec3::ZERO));
    /// builder.set_bounds(Vec3::new(-10.0, -10.0, -10.0), Vec3::new(10.0, 10.0, 10.0));
    /// ```
    pub fn set_bounds(&mut self, min: Vec3, max: Vec3) {
        self.min = (min / self.voxel_size).floor().as_ivec3();
        self.max = (max / self.voxel_size).ceil().as_ivec3();
    }

    /// Builds the sparse voxel octree.
    ///
    /// # Example
    ///
    /// ```
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh};
    /// use bevy_math::{IVec3, UVec3};
    ///
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    ///
    /// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 3, 0)], 1.0, IVec3::ZERO));
    ///
    /// let octree = builder.build();
    /// ```
    #[must_use]
    pub fn build(self) -> SparseVoxelOctree {
        let mut voxels = Vec::new();

        for mesh in self.meshes {
            voxels.append(&mut mesh.voxels());
        }

        let (min, max) = Self::get_min_max(&voxels);
        let min = self.min.min(min);
        let max = self.max.max(max);

        let (origin, size) = Self::get_origin_and_size(min, max);

        let (layer_zero, leafs) = Self::collect_leafs_and_zero_layer_nodes(&voxels, origin);

        let mut current_node_size: u32 = 4;

        let mut layers = vec![layer_zero];

        while current_node_size < size {
            let (next_node_size, layer) =
                Self::create_next_layer(&mut layers, current_node_size, size);

            layers.push(layer);
            current_node_size = next_node_size;
        }

        Self::fill_parents(&mut layers);
        Self::fill_neighbors(&mut layers);

        SparseVoxelOctree {
            origin,
            layers,
            leafs,
            voxel_size: self.voxel_size,
        }
    }

    /// Assigns parents to all nodes in the octree.
    fn fill_parents(layers: &mut [Vec<SparseVoxelOctreeNode>]) {
        for i in (0..layers.len()).rev() {
            for y in 0..layers[i].len() {
                if let Some(first_child) = layers[i][y].first_child {
                    let layer = &mut layers[first_child.layer_index];

                    for z in 0..8_usize {
                        let node_index = first_child.node_index + z;
                        let node = &mut layer[node_index];
                        node.parent = Some(SparseVoxelOctreeLink::new(i, y, None));
                    }
                }
            }
        }
    }

    /// When the octree is built, the nodes are not connected to their neighbors
    /// (nodes that share a face).
    fn fill_neighbors(layers: &mut [Vec<SparseVoxelOctreeNode>]) {
        if layers.is_empty() {
            return;
        }

        if layers[0].is_empty() {
            return;
        }

        let mut nodes = vec![SparseVoxelOctreeLink::new(layers.len() - 1, 0, None)];

        while let Some(node) = nodes.pop() {
            // interconnect children
            if let Some(first_child) = layers[node.layer_index][node.node_index].first_child {
                for i in 0..8 {
                    nodes.push(SparseVoxelOctreeLink::new(
                        first_child.layer_index,
                        first_child.node_index + i,
                        None,
                    ));
                }

                for (neighbor_index_1, neighbor_index_2, offset_1, offset_2) in &SIBLING_CONNECTIONS
                {
                    layers[first_child.layer_index][first_child.node_index + offset_1].neighbors
                        [*neighbor_index_1] = Some(SparseVoxelOctreeLink::new(
                        first_child.layer_index,
                        first_child.node_index + offset_2,
                        None,
                    ));

                    layers[first_child.layer_index][first_child.node_index + offset_2].neighbors
                        [*neighbor_index_2] = Some(SparseVoxelOctreeLink::new(
                        first_child.layer_index,
                        first_child.node_index + offset_1,
                        None,
                    ));
                }

                // interconnect children with own neighbors
                for (from_neighbor_index, (from_nodes, to_nodes)) in
                    NEIGHBOR_CONNECTIONS.iter().enumerate()
                {
                    if let Some(neighbor) =
                        layers[node.layer_index][node.node_index].neighbors[from_neighbor_index]
                    {
                        if let Some(own_first_child) =
                            layers[node.layer_index][node.node_index].first_child
                        {
                            if let Some(neighbor_first_child) =
                                layers[neighbor.layer_index][neighbor.node_index].first_child
                            {
                                for i in 0..4 {
                                    layers[own_first_child.layer_index]
                                        [own_first_child.node_index + from_nodes[i]]
                                        .neighbors[from_neighbor_index] =
                                        Some(SparseVoxelOctreeLink::new(
                                            neighbor_first_child.layer_index,
                                            neighbor_first_child.node_index + to_nodes[i],
                                            None,
                                        ));
                                }
                            } else {
                                for node in from_nodes.iter().take(4) {
                                    layers[own_first_child.layer_index]
                                        [own_first_child.node_index + node]
                                        .neighbors[from_neighbor_index] = Some(neighbor);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn validate_all_children_present(nodes: &[SparseVoxelOctreeNode], node_size: u32) -> bool {
        if nodes.len() % 8 != 0 {
            return false;
        }

        for i in (0..nodes.len()).step_by(8) {
            let first_position = nodes[i].position / node_size;
            for y in 0..8 {
                let node_pos = nodes[i + y].position / node_size;
                let diff = node_pos - first_position;
                if diff.x != OFFSETS_IN_MORTON_CODE_ORDER[y].0.into()
                    || diff.y != OFFSETS_IN_MORTON_CODE_ORDER[y].1.into()
                    || diff.z != OFFSETS_IN_MORTON_CODE_ORDER[y].2.into()
                {
                    return false;
                }
            }
        }

        true
    }

    fn collect_leafs_and_zero_layer_nodes(
        voxels: &[IVec3],
        origin: IVec3,
    ) -> (Vec<SparseVoxelOctreeNode>, Vec<CompoundNode>) {
        let mut leafs = HashMap::new();
        let mut layer_zero = HashMap::new();

        // create 4x4 leafs
        for voxel in voxels {
            let offset = (*voxel - origin).as_uvec3();
            let leaf_parent_coords = offset >> 3 << 3;

            for x in 0..2 {
                for y in 0..2 {
                    for z in 0..2 {
                        let leaf_coords = UVec3::new(x * 4, y * 4, z * 4);
                        let leaf_coords: UVec3 = leaf_parent_coords + leaf_coords;

                        leafs.entry(leaf_coords).or_insert_with(CompoundNode::new);
                        layer_zero
                            .entry(leaf_coords)
                            .or_insert_with(|| SparseVoxelOctreeNode::leaf(leaf_coords));
                    }
                }
            }
        }

        for voxel in voxels {
            let offset = (*voxel - origin).as_uvec3();
            let leaf_coords: UVec3 = offset >> 2 << 2;

            let layer_zero = layer_zero
                .get_mut(&leaf_coords)
                .expect("layer zero node not found");

            let leafs = leafs.get_mut(&leaf_coords).expect("leaf node not found");

            let local_coords = offset - leaf_coords;
            leafs.set(local_coords.x, local_coords.y, local_coords.z, true);

            layer_zero.is_leaf = true;
        }

        let mut leafs = leafs
            .into_iter()
            .map(|(position, node)| (node, MortonCode::encode(position)))
            .collect::<Vec<_>>();

        leafs.sort_by(|(_, a), (_, b)| a.cmp(b));

        let leafs = leafs.into_iter().map(|(node, _)| node).collect();

        let mut layer_zero: Vec<_> = layer_zero.into_values().collect();

        layer_zero.sort();

        (layer_zero, leafs)
    }

    fn get_origin_and_size(min: IVec3, max: IVec3) -> (IVec3, u32) {
        if min == max {
            return (min, 1);
        }

        let size = max - min;

        #[allow(clippy::cast_sign_loss)]
        let mut size = size.max_element() as u32;

        if !size.is_power_of_two() {
            size = size.next_power_of_two();
        }

        (min, size)
    }

    fn get_min_max(voxels: &[IVec3]) -> (IVec3, IVec3) {
        if voxels.is_empty() {
            return (IVec3::ZERO, IVec3::ZERO);
        }

        let mut min = IVec3::MAX;
        let mut max = IVec3::MIN;

        for voxel in voxels {
            min = min.min(*voxel);
            max = max.max(*voxel);
        }

        (min, max)
    }

    fn create_next_layer(
        layers: &mut Vec<Vec<SparseVoxelOctreeNode>>,
        current_node_size: u32,
        size: u32,
    ) -> (u32, Vec<SparseVoxelOctreeNode>) {
        let next_layer_index = layers.len();
        let next_node_size = current_node_size * 2;
        let last_layer = layers.last_mut().unwrap();

        assert!(
            Self::validate_all_children_present(last_layer, current_node_size),
            "not all children present"
        );

        let mut layer = Vec::new();

        for i in (0..last_layer.len()).step_by(8) {
            let position = last_layer[i].position;
            let mut node = SparseVoxelOctreeNode::node(position, next_node_size);

            node.first_child = Some(SparseVoxelOctreeLink::new(next_layer_index - 1, i, None));

            layer.push(node);
        }

        if next_node_size < size {
            // There will be next iteration so we need to fill empty children spots
            let mut i = 0;
            loop {
                let first_position = {
                    if layer.len() <= i {
                        UVec3::ZERO
                    } else {
                        (layer[i].position / (next_node_size * 2)) * next_node_size * 2
                    }
                };

                for (y, item) in OFFSETS_IN_MORTON_CODE_ORDER.iter().enumerate() {
                    Self::fill_node_if_it_doesnt_exist(
                        &mut layer,
                        i + y,
                        first_position,
                        next_node_size,
                        UVec3::new(item.0.into(), item.1.into(), item.2.into()),
                    );
                }

                i += 8;

                if i >= layer.len() {
                    break;
                }
            }
        }

        (next_node_size, layer)
    }

    fn fill_node_if_it_doesnt_exist(
        layer: &mut Vec<SparseVoxelOctreeNode>,
        node_index: usize,
        first_position: UVec3,
        node_size: u32,
        offset: UVec3,
    ) {
        let node_1_exists = {
            if layer.len() > node_index {
                let node_1_pos = layer[node_index].position;

                if first_position.x > node_1_pos.x
                    || first_position.y > node_1_pos.y
                    || first_position.z > node_1_pos.z
                {
                    false
                } else {
                    (node_1_pos - first_position) / node_size == offset
                }
            } else {
                false
            }
        };

        if !node_1_exists {
            let pos = first_position + offset * node_size;

            let node = SparseVoxelOctreeNode::node(pos, node_size);
            layer.insert(node_index, node);
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::{IVec3, UVec3};

    use super::*;

    #[test]
    fn test_get_min_max() {
        let voxels = vec![
            IVec3::new(0, 0, 0),
            IVec3::new(1, 1, 1),
            IVec3::new(2, 2, 2),
            IVec3::new(3, 3, 3),
        ];

        let (min, max) = SparseVoxelOctreeBuilder::get_min_max(&voxels);

        assert_eq!(min, IVec3::new(0, 0, 0));
        assert_eq!(max, IVec3::new(3, 3, 3));
    }

    #[test]
    fn test_get_min_max_no_voxels() {
        let (min, max) = SparseVoxelOctreeBuilder::get_min_max(&[]);

        assert_eq!(min, IVec3::new(0, 0, 0));
        assert_eq!(max, IVec3::new(0, 0, 0));
    }

    #[test]
    fn test_get_origin_and_size() {
        let voxels = vec![
            IVec3::new(0, 0, 0),
            IVec3::new(1, 1, 1),
            IVec3::new(2, 2, 2),
            IVec3::new(3, 3, 3),
        ];

        let (min, max) = SparseVoxelOctreeBuilder::get_min_max(&voxels);
        let (origin, size) = SparseVoxelOctreeBuilder::get_origin_and_size(min, max);

        assert_eq!(origin, IVec3::new(0, 0, 0));
        assert_eq!(size, 4);
    }

    #[test]
    fn test_get_origin_and_size_no_voxels() {
        let (min, max) = SparseVoxelOctreeBuilder::get_min_max(&[]);
        let (origin, size) = SparseVoxelOctreeBuilder::get_origin_and_size(min, max);

        assert_eq!(origin, IVec3::new(0, 0, 0));
        assert_eq!(size, 1);
    }

    #[test]
    fn test_collect_leafs_and_zero_layer_nodes() {
        let voxels = vec![
            IVec3::new(4, 4, 4),
            IVec3::new(5, 5, 5),
            IVec3::new(6, 6, 6),
            IVec3::new(0, 0, 0),
            IVec3::new(1, 1, 1),
            IVec3::new(2, 2, 2),
            IVec3::new(3, 3, 3),
        ];

        let (min, max) = SparseVoxelOctreeBuilder::get_min_max(&voxels);
        let (origin, _size) = SparseVoxelOctreeBuilder::get_origin_and_size(min, max);

        let (layer_zero, leafs) =
            SparseVoxelOctreeBuilder::collect_leafs_and_zero_layer_nodes(&voxels, origin);

        assert_eq!(layer_zero.len(), 8);
        assert_eq!(leafs.len(), 8);

        assert_eq!(layer_zero[0].position, UVec3::new(0, 0, 0));
        assert_eq!(layer_zero[0].size, 4);
        assert!(leafs[0].get(0, 0, 0));
        assert!(leafs[0].get(1, 1, 1));
        assert!(leafs[0].get(2, 2, 2));
        assert!(leafs[0].get(3, 3, 3));

        assert_eq!(layer_zero[1].position, UVec3::new(4, 0, 0));
        assert_eq!(layer_zero[1].size, 4);
        assert_eq!(*leafs[1], 0);

        assert_eq!(layer_zero[2].position, UVec3::new(0, 4, 0));
        assert_eq!(layer_zero[2].size, 4);
        assert_eq!(*leafs[2], 0);

        assert_eq!(layer_zero[3].position, UVec3::new(4, 4, 0));
        assert_eq!(layer_zero[3].size, 4);
        assert_eq!(*leafs[3], 0);

        assert_eq!(layer_zero[4].position, UVec3::new(0, 0, 4));
        assert_eq!(layer_zero[4].size, 4);
        assert_eq!(*leafs[4], 0);

        assert_eq!(layer_zero[5].position, UVec3::new(4, 0, 4));
        assert_eq!(layer_zero[5].size, 4);
        assert_eq!(*leafs[5], 0);

        assert_eq!(layer_zero[6].position, UVec3::new(0, 4, 4));
        assert_eq!(layer_zero[6].size, 4);
        assert_eq!(*leafs[6], 0);

        assert_eq!(layer_zero[7].position, UVec3::new(4, 4, 4));
        assert_eq!(layer_zero[7].size, 4);
        assert!(leafs[7].get(0, 0, 0));
        assert!(leafs[7].get(1, 1, 1));
        assert!(leafs[7].get(2, 2, 2));
        assert!(!leafs[7].get(3, 3, 3));
    }

    #[test]
    fn test_build_octree_with_no_voxels() {
        let mut builder = SparseVoxelOctreeBuilder::new(1.0);
        builder.set_bounds(Vec3::new(-4.0, -4.0, -4.0), Vec3::new(4.0, 4.0, 4.0));
        let octree = builder.build();

        assert_eq!(octree.layers.len(), 2);
    }
}
