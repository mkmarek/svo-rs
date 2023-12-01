// Resource: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter21_3D_Flight_Navigation_Using_Sparse_Voxel_Octrees.pdf

use bevy_math::{IVec3, UVec3, Vec3};

use crate::{
    cohen_sutherland::{cohen_sutherland, LineClippingResult},
    compound_node::CompoundNode,
    consts::{
        NEIGHBOR_CONNECTIONS, NEIGHBOR_POSITION_OFFSETS, NEIGHBOR_SUBNODES,
        OFFSETS_IN_MORTON_CODE_ORDER, SUBNODE_NEIGHBORS, SUBNODE_POSITIONS,
    },
    morton_code::MortonCode,
    sparse_voxel_octree_link::SparseVoxelOctreeLink,
    sparse_voxel_octree_node::SparseVoxelOctreeNode,
};

/// Implementation of a sparse voxel octree.
///
/// The sparse voxel octree is a data structure that is used to represent a 3D space.
/// It is a tree structure where each node represents a cubic volume in space.
///
/// The tree is sparse, meaning that not every node is present in the tree.
/// Only nodes that contain geometry are present in the tree.
///
/// It is structured in layers where layer zero is the root layer containing all the leaf nodes
/// and each layer above it contains the parent nodes of the layer below it.
///
/// Each leaf node is a 4x4x4 cube and each parent node is a 2x2x2 cube.
/// Leaf nodes are aggregated into compound nodes that are of type u64
/// where each bit represents a distinct position of a single voxel. The bit 1 means that the voxel is filled
/// and the bit 0 means that the voxel is empty.
///
/// Absolute position of a node is calculated by multiplying the position of the node by the size of the node and
/// the voxel size and then by adding the origin of the octree.
///
/// Each node has a position in space, parent, children and a list of neighbors to allow
/// for easy navigation through the tree.
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
pub struct SparseVoxelOctree {
    /// Size of a single voxel in worldspace units.
    ///
    /// Voxel size defines the resolution of the octree.
    pub(crate) voxel_size: f32,

    /// Origin of the octree.
    pub(crate) origin: IVec3,

    /// Layers containing the nodes of the octree.
    ///
    /// layer[0] contains all the leaf nodes.
    /// layer[1] contains all the parent nodes of the leaf nodes.
    /// ...
    /// layer[n] contains all the parent nodes of the layer[n-1].
    ///
    /// the layer[layer.len() - 1] contains the root node.
    pub(crate) layers: Vec<Vec<SparseVoxelOctreeNode>>,

    /// Compound nodes containing information about single voxels.
    /// A single compound node is a 4x4x4 cube of voxels.
    ///
    /// It has the same ordering as the layer[0] so you can use the same index
    /// to access both.
    pub(crate) leafs: Vec<CompoundNode>,
}

impl SparseVoxelOctree {
    /// Retrieves all neighbors of a node.
    ///
    /// Takes into account also if the neighboring node is subdivided or a leaf node,
    /// returning the children is available.
    ///
    /// # Arguments
    ///
    /// * `link` - Link to the node.
    ///
    /// # Returns
    ///
    /// A vector containing all the neighbors of the node.
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
    /// builder.set_bounds(Vec3::new(-4.0, -4.0, -4.0), Vec3::new(4.0, 4.0, 4.0));
    ///
    /// let octree = builder.build();
    ///
    /// let link = octree.find_node(Vec3::new(0.0, 0.0, 0.0)).unwrap();
    ///
    /// let neighbors = octree.successors(link);
    ///
    /// assert_eq!(neighbors.len(), 3);
    /// ```
    #[must_use]
    pub fn successors(&self, link: SparseVoxelOctreeLink) -> Vec<SparseVoxelOctreeLink> {
        let mut result = Vec::with_capacity(16);

        let node = &self.layers[link.layer_index][link.node_index];

        for i in 0..6 {
            if let Some(neighbor) = &node.neighbors[i] {
                let neighbor_node = &self.layers[neighbor.layer_index][neighbor.node_index];

                if let Some(subnode) = link.subnode_index {
                    if !CompoundNode::is_face(subnode, i) {
                        let neighbor_index = SUBNODE_NEIGHBORS[subnode as usize][i];
                        let leaf_node = &self.leafs[link.node_index];
                        if !leaf_node.get_by_index(neighbor_index) {
                            result.push(SparseVoxelOctreeLink::new(
                                link.layer_index,
                                link.node_index,
                                Some(neighbor_index),
                            ));
                        }
                    } else if neighbor_node.first_child.is_some() {
                        result.append(&mut self.expand_to_neighboring_children(i, neighbor));
                    } else if neighbor_node.is_leaf {
                        if let Some(neighbor) =
                            self.find_neighboring_subnode_for_subnode(i, neighbor, subnode)
                        {
                            result.push(neighbor);
                        }
                    } else {
                        result.push(*neighbor);
                    }
                } else if neighbor_node.first_child.is_some() {
                    result.append(&mut self.expand_to_neighboring_children(i, neighbor));
                } else if neighbor_node.is_leaf {
                    result.append(&mut self.expand_to_neighboring_subnodes(i, neighbor));
                } else {
                    result.push(*neighbor);
                }
            }
        }

        result
    }

    fn find_neighboring_subnode_for_subnode(
        &self,
        neighbor_index: usize,
        neighbor: &SparseVoxelOctreeLink,
        subnode: u8,
    ) -> Option<SparseVoxelOctreeLink> {
        let leaf = &self.leafs[neighbor.node_index];

        if leaf.is_empty() {
            return Some(*neighbor);
        }

        if leaf.is_full() {
            return None;
        }

        let neighbor_index = SUBNODE_NEIGHBORS[subnode as usize][neighbor_index];

        if !leaf.get_by_index(neighbor_index) {
            return Some(SparseVoxelOctreeLink::new(
                neighbor.layer_index,
                neighbor.node_index,
                Some(neighbor_index),
            ));
        }

        None
    }

    #[inline]
    fn expand_to_neighboring_subnodes(
        &self,
        neighbor_index: usize,
        neighbor: &SparseVoxelOctreeLink,
    ) -> Vec<SparseVoxelOctreeLink> {
        let leaf = &self.leafs[neighbor.node_index];

        if leaf.is_empty() {
            return vec![*neighbor];
        }

        if leaf.is_full() {
            return vec![];
        }

        let mut result = Vec::with_capacity(16);

        let neighboring_nodes = &NEIGHBOR_SUBNODES[neighbor_index];

        for node in neighboring_nodes {
            if leaf.get_by_index(node.3) {
                continue;
            }

            result.push(SparseVoxelOctreeLink::new(
                neighbor.layer_index,
                neighbor.node_index,
                Some(node.3),
            ));
        }

        result
    }

    fn expand_to_neighboring_children(
        &self,
        neighbor_index: usize,
        neighbor: &SparseVoxelOctreeLink,
    ) -> Vec<SparseVoxelOctreeLink> {
        let mut close = Vec::new();
        let mut open = vec![*neighbor];

        while let Some(neighbor) = open.pop() {
            let node = &self.layers[neighbor.layer_index][neighbor.node_index];
            let first_child = node.first_child.unwrap();

            let neighboring_nodes = &NEIGHBOR_CONNECTIONS[neighbor_index].1;
            for node in neighboring_nodes.iter().take(4) {
                let child = SparseVoxelOctreeLink::new(
                    first_child.layer_index,
                    first_child.node_index + node,
                    None,
                );
                let child_node = &self.layers[child.layer_index][child.node_index];

                if child_node.first_child.is_some() {
                    open.push(child);
                } else if child_node.is_leaf {
                    close.append(&mut self.expand_to_neighboring_subnodes(neighbor_index, &child));
                } else {
                    close.push(child);
                }
            }
        }

        close
    }

    /// Finds a node based on a worldspace position.
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
    ///
    /// let octree = builder.build();
    ///
    /// let link = octree.find_node(Vec3::new(0.0, 3.0, 0.0));
    ///
    /// assert!(link.is_some());
    /// ```
    #[must_use]
    pub fn find_node(&self, position: Vec3) -> Option<SparseVoxelOctreeLink> {
        let voxel_position = (position / self.voxel_size).as_ivec3();
        let voxel_position = (voxel_position - self.origin).as_uvec3();

        let mut current_node = SparseVoxelOctreeLink::new(self.layers.len() - 1, 0, None);

        loop {
            let node = &self.layers[current_node.layer_index][current_node.node_index];

            if node.is_leaf {
                let leaf = &self.leafs[current_node.node_index];
                if leaf.is_empty() {
                    return Some(SparseVoxelOctreeLink::new(
                        current_node.layer_index,
                        current_node.node_index,
                        None,
                    ));
                }

                let node_position: UVec3 = node.position;
                let local_coords = UVec3::new(voxel_position.x, voxel_position.y, voxel_position.z)
                    - node_position;
                let voxel_index = MortonCode::encode(local_coords).as_u8();

                if let Ok(voxel_index) = voxel_index {
                    return Some(SparseVoxelOctreeLink::new(
                        current_node.layer_index,
                        current_node.node_index,
                        Some(voxel_index),
                    ));
                }
            }

            if let Some(first_child) = node.first_child {
                let mut found = false;

                let offset = (voxel_position - node.position) / (node.size / 2);

                for (i, item) in OFFSETS_IN_MORTON_CODE_ORDER.iter().enumerate() {
                    if offset.x == Into::<u32>::into(item.0)
                        && offset.y == Into::<u32>::into(item.1)
                        && offset.z == Into::<u32>::into(item.2)
                    {
                        current_node = first_child;
                        current_node.node_index += i;
                        found = true;
                        break;
                    }
                }

                if !found {
                    break;
                }
            } else {
                return Some(current_node);
            }
        }

        None
    }

    /// Draw lines between the node and all of its neighbors using bevy gizmos.
    #[cfg(feature = "bevy")]
    pub fn draw_connected_nodes(
        &self,
        node: SparseVoxelOctreeLink,
        gizmos: &mut bevy_gizmos::prelude::Gizmos,
    ) {
        let from = self.node_position(node);

        let successors = self.successors(node);

        for successor in successors {
            gizmos.line(
                from,
                self.node_position(successor),
                bevy_render::prelude::Color::GREEN,
            );
        }
    }

    /// Returns a boolean indicating whether two points are in line of sight.
    ///
    /// Uses Cohen-Sutherland line clipping algorithm.
    ///
    /// # Example
    ///
    /// ```
    /// use svo_rs::{SparseVoxelOctreeBuilder, VoxelizedMesh};
    /// use bevy_math::{IVec3, UVec3, Vec3};
    ///
    /// let mut builder = SparseVoxelOctreeBuilder::new(1.0);
    ///
    /// builder.add_mesh(VoxelizedMesh::new(vec![UVec3::new(0, 1, 0)], 1.0, IVec3::ZERO));
    /// builder.set_bounds(Vec3::new(-4.0, -4.0, -4.0), Vec3::new(4.0, 4.0, 4.0));
    ///
    /// let octree = builder.build();
    ///
    /// assert_eq!(octree.is_in_line_of_sight(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)), false);
    /// assert_eq!(octree.is_in_line_of_sight(Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0)), true);
    /// ```
    #[must_use]
    pub fn is_in_line_of_sight(&self, from: Vec3, to: Vec3) -> bool {
        let from = ((from / self.voxel_size).as_ivec3() - self.origin)
            .max(IVec3::ZERO)
            .as_uvec3();

        let to = ((to / self.voxel_size).as_ivec3() - self.origin)
            .max(IVec3::ZERO)
            .as_uvec3();

        let mut open = vec![SparseVoxelOctreeLink::new(self.layers.len() - 1, 0, None)];

        while let Some(link) = open.pop() {
            let node = &self.layers[link.layer_index][link.node_index];
            if let Some(first_child) = node.first_child {
                for i in 0..8 {
                    let child_node =
                        &self.layers[first_child.layer_index][first_child.node_index + i];

                    if cohen_sutherland(
                        &from.to_array(),
                        &to.to_array(),
                        &child_node.position.to_array(),
                        &(child_node.position + node.size).to_array(),
                    ) == LineClippingResult::Outside
                    {
                        continue;
                    }

                    if child_node.is_leaf {
                        let leaf = &self.leafs[first_child.node_index + i];

                        if leaf.is_empty() {
                            continue;
                        }

                        if leaf.is_full() {
                            return false;
                        }

                        let occupied = leaf.get_occupied_indexes();

                        for index in occupied {
                            if let Ok(local_coords) = MortonCode::from_u8(index).decode() {
                                if cohen_sutherland(
                                    &from.to_array(),
                                    &to.to_array(),
                                    &(child_node.position + local_coords).to_array(),
                                    &(child_node.position + local_coords + 1).to_array(),
                                ) != LineClippingResult::Outside
                                {
                                    return false;
                                }
                            }
                        }
                    } else {
                        open.push(SparseVoxelOctreeLink::new(
                            first_child.layer_index,
                            first_child.node_index + i,
                            None,
                        ));
                    }
                }
            }
        }

        true
    }

    /// Returns position of the center of the face between two neighboring nodes.
    /// Returns None if the nodes are not neighbors.
    #[must_use]
    pub fn face_position_between(
        &self,
        a: SparseVoxelOctreeLink,
        b: SparseVoxelOctreeLink,
    ) -> Option<Vec3> {
        if a.layer_index == b.layer_index {
            let a = self.node_position(a);
            let b = self.node_position(b);

            return Some((a + b) / 2.0);
        }

        if a.layer_index > b.layer_index {
            return self.face_position_between(b, a);
        }

        // a is now always the lower layer
        for (i, neighbor_offset) in NEIGHBOR_POSITION_OFFSETS
            .iter()
            .enumerate()
            .take(self.layers[a.layer_index][a.node_index].neighbors.len())
        {
            if let Some(n) = self.layers[a.layer_index][a.node_index].neighbors[i] {
                if n == b {
                    let node = &self.layers[n.layer_index][n.node_index];
                    #[allow(clippy::cast_precision_loss)]
                    return Some(
                        self.node_position(n)
                            + Vec3::new(
                                neighbor_offset.0 as f32 * node.size as f32 / 2.0 * self.voxel_size,
                                neighbor_offset.1 as f32 * node.size as f32 / 2.0 * self.voxel_size,
                                neighbor_offset.2 as f32 * node.size as f32 / 2.0 * self.voxel_size,
                            ),
                    );
                }
            }
        }

        None
    }

    /// Returns position of the center of a node in world space.
    #[must_use]
    pub fn node_position(&self, link: SparseVoxelOctreeLink) -> Vec3 {
        let node = &self.layers[link.layer_index][link.node_index];
        let position = (node.position.as_vec3() + self.origin.as_vec3()) * self.voxel_size;

        #[allow(clippy::cast_precision_loss)]
        let scale_f32 =
            Vec3::new(node.size as f32, node.size as f32, node.size as f32) * self.voxel_size;

        if let Some(subnode) = link.subnode_index {
            let point = SUBNODE_POSITIONS[subnode as usize];
            #[allow(clippy::cast_precision_loss)]
            let position = position
                + Vec3::new(
                    f32::from(point.0) * self.voxel_size,
                    f32::from(point.1) * self.voxel_size,
                    f32::from(point.2) * self.voxel_size,
                )
                + Vec3::new(
                    self.voxel_size / 2.0,
                    self.voxel_size / 2.0,
                    self.voxel_size / 2.0,
                );

            return position;
        }

        position + scale_f32 / 2.0
    }

    #[cfg(feature = "bevy")]
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_node_gizmo(
        &self,
        gizmos: &mut bevy_gizmos::prelude::Gizmos,
        link: SparseVoxelOctreeLink,
        color: bevy_render::prelude::Color,
    ) {
        let position = self.node_position(link);
        let size = if link.subnode_index.is_some() {
            self.voxel_size
        } else {
            let node = &self.layers[link.layer_index][link.node_index];
            node.size as f32 * self.voxel_size
        };
        gizmos.cuboid(
            bevy_transform::prelude::Transform::from_translation(position)
                .with_scale(Vec3::ONE * size),
            color,
        );
    }

    /// Draws cubes for each node in the octree.
    #[cfg(feature = "bevy")]
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_gizmos(&self, gizmos: &mut bevy_gizmos::prelude::Gizmos, draw_connections: bool) {
        for layer in &self.layers {
            for node in layer {
                let position = (node.position.as_vec3() + self.origin.as_vec3()) * self.voxel_size;
                let scale_f32 = Vec3::ONE * (node.size as f32) * self.voxel_size;
                let position = position + scale_f32 / 2.0;

                if node.is_leaf {
                    gizmos.cuboid(
                        bevy_transform::prelude::Transform::from_translation(position)
                            .with_scale(scale_f32),
                        bevy_render::prelude::Color::RED,
                    );
                } else {
                    gizmos.cuboid(
                        bevy_transform::prelude::Transform::from_translation(position)
                            .with_scale(scale_f32),
                        bevy_render::prelude::Color::YELLOW,
                    );
                }

                if draw_connections {
                    for neighbor in node.neighbors.iter().flatten() {
                        let neighbor = &self.layers[neighbor.layer_index][neighbor.node_index];

                        let neighbor_position =
                            (neighbor.position.as_vec3() + self.origin.as_vec3()) * self.voxel_size;

                        #[allow(clippy::cast_precision_loss)]
                        let neighbor_scale_f32 = Vec3::new(
                            neighbor.size as f32,
                            neighbor.size as f32,
                            neighbor.size as f32,
                        ) * self.voxel_size;

                        let neighbor_position = neighbor_position + neighbor_scale_f32 / 2.0;

                        gizmos.line(
                            position,
                            neighbor_position,
                            bevy_render::prelude::Color::RED,
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::{IVec3, UVec3, Vec3};

    use crate::{compound_node::CompoundNode, SparseVoxelOctreeBuilder, VoxelizedMesh};

    use super::*;

    #[test]
    #[ignore]
    fn test_compound_node_assignments() {
        let mut node = CompoundNode::new();

        node.set(0, 0, 0, true);

        assert!(node.get(0, 0, 0));

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }

                    assert!(!node.get(x, y, z));
                }
            }
        }

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    node.set(x, y, z, true);
                }
            }
        }

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    assert!(node.get(x, y, z));
                }
            }
        }

        assert_eq!(*node, u64::MAX);
    }

    #[test]
    fn test_find_node() {
        let voxels = vec![
            UVec3::new(4, 4, 4),
            UVec3::new(5, 5, 5),
            UVec3::new(6, 6, 6),
            UVec3::new(0, 0, 0),
            UVec3::new(1, 1, 1),
            UVec3::new(2, 2, 2),
            UVec3::new(3, 3, 3),
            UVec3::new(26, 25, 17),
            UVec3::new(80, 80, 80),
            UVec3::new(41, 41, 41),
        ];

        let mut builder = SparseVoxelOctreeBuilder::new(1.0);
        builder.add_mesh(VoxelizedMesh::new(voxels, 1.0, IVec3::ZERO));

        let tree = builder.build();

        let node = tree
            .find_node(Vec3::new(42.0, 41.0, 41.0))
            .expect("node not found");

        assert_eq!(node.layer_index, 0);
        assert_eq!(node.node_index, 16);
        assert_eq!(
            node.subnode_index.unwrap(),
            MortonCode::encode_xyz(2, 1, 1).as_u8().unwrap()
        );
    }

    #[test]
    fn test_successors_air_node() {
        let voxels = vec![UVec3::new(4, 4, 4), UVec3::new(80, 80, 80)];

        let mut builder = SparseVoxelOctreeBuilder::new(1.0);
        builder.add_mesh(VoxelizedMesh::new(voxels, 1.0, IVec3::ZERO));

        let tree = builder.build();

        let node_link = tree
            .find_node(Vec3::new(40.0, 40.0, 40.0))
            .expect("node not found");

        let node = &tree.layers[node_link.layer_index][node_link.node_index];
        assert_eq!(node.position, UVec3::new(32, 32, 32));

        let successors = tree.successors(node_link);

        assert_eq!(successors.len(), 6);

        let successor_0 = &tree.layers[successors[0].layer_index][successors[0].node_index];
        assert_eq!(successor_0.position, UVec3::new(64, 0, 0));

        let successor_1 = &tree.layers[successors[1].layer_index][successors[1].node_index];
        assert_eq!(successor_1.position, UVec3::new(0, 0, 64));

        let successor_2 = &tree.layers[successors[2].layer_index][successors[2].node_index];
        assert_eq!(successor_2.position, UVec3::new(0, 32, 32));

        let successor_3 = &tree.layers[successors[3].layer_index][successors[3].node_index];
        assert_eq!(successor_3.position, UVec3::new(32, 32, 0));

        let successor_4 = &tree.layers[successors[4].layer_index][successors[4].node_index];
        assert_eq!(successor_4.position, UVec3::new(0, 64, 0));

        let successor_5 = &tree.layers[successors[5].layer_index][successors[5].node_index];
        assert_eq!(successor_5.position, UVec3::new(32, 0, 32));
    }

    #[test]
    fn test_successors_from_low_resolution_to_higher_resolution() {
        let voxels = vec![UVec3::new(4, 4, 4), UVec3::new(80, 80, 80)];

        let mut builder = SparseVoxelOctreeBuilder::new(1.0);
        builder.add_mesh(VoxelizedMesh::new(voxels, 1.0, IVec3::ZERO));

        let tree = builder.build();

        let node_link = tree
            .find_node(Vec3::new(12.0, 24.0, 12.0))
            .expect("node not found");

        let node = &tree.layers[node_link.layer_index][node_link.node_index];
        assert_eq!(node.position, UVec3::new(0, 16, 0));

        let successors = tree.successors(node_link);

        assert_eq!(successors.len(), 7);

        let successor_0 = &tree.layers[successors[0].layer_index][successors[0].node_index];
        assert_eq!(successor_0.position, UVec3::new(16, 16, 0));

        let successor_1 = &tree.layers[successors[1].layer_index][successors[1].node_index];
        assert_eq!(successor_1.position, UVec3::new(0, 16, 16));

        let successor_2 = &tree.layers[successors[2].layer_index][successors[2].node_index];
        assert_eq!(successor_2.position, UVec3::new(0, 32, 0));

        let successor_3 = &tree.layers[successors[3].layer_index][successors[3].node_index];
        assert_eq!(successor_3.position, UVec3::new(0, 8, 0));

        let successor_4 = &tree.layers[successors[4].layer_index][successors[4].node_index];
        assert_eq!(successor_4.position, UVec3::new(0, 8, 8));

        let successor_5 = &tree.layers[successors[5].layer_index][successors[5].node_index];
        assert_eq!(successor_5.position, UVec3::new(8, 8, 0));

        let successor_6 = &tree.layers[successors[6].layer_index][successors[6].node_index];
        assert_eq!(successor_6.position, UVec3::new(8, 8, 8));
    }

    #[test]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    fn test_successors_from_low_resolution_to_leaf_nodes() {
        let voxels = vec![
            UVec3::new(4, 4, 4),
            UVec3::new(4, 11, 4),
            UVec3::new(80, 80, 80),
        ];

        let mut builder = SparseVoxelOctreeBuilder::new(1.0);
        builder.add_mesh(VoxelizedMesh::new(voxels, 1.0, IVec3::ZERO));

        let tree = builder.build();

        let node_link = tree
            .find_node(Vec3::new(8.0, 16.0, 8.0))
            .expect("node not found");

        let node = &tree.layers[node_link.layer_index][node_link.node_index];
        assert_eq!(node.position, UVec3::new(0, 8, 0));

        let successors = tree.successors(node_link);

        assert_eq!(successors.len(), 21);

        let successor_0 = &tree.layers[successors[0].layer_index][successors[0].node_index];
        assert_eq!(successor_0.position, UVec3::new(8, 8, 0));

        let successor_1 = &tree.layers[successors[1].layer_index][successors[1].node_index];
        assert_eq!(successor_1.position, UVec3::new(0, 8, 8));

        let successor_2 = &tree.layers[successors[2].layer_index][successors[2].node_index];
        assert_eq!(successor_2.position, UVec3::new(0, 16, 0));

        // Subnodes
        let successor_3 = &tree.layers[successors[3].layer_index][successors[3].node_index];
        assert_eq!(successor_3.position, UVec3::new(0, 4, 0));
        assert!(successor_3.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[3].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(1, 3, 0)
        );

        let successor_4 = &tree.layers[successors[4].layer_index][successors[4].node_index];
        assert_eq!(successor_4.position, UVec3::new(0, 4, 0));
        assert!(successor_4.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[4].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(2, 3, 0)
        );

        let successor_5 = &tree.layers[successors[5].layer_index][successors[5].node_index];
        assert_eq!(successor_5.position, UVec3::new(0, 4, 0));
        assert!(successor_5.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[5].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(3, 3, 0)
        );

        let successor_6 = &tree.layers[successors[6].layer_index][successors[6].node_index];
        assert_eq!(successor_6.position, UVec3::new(0, 4, 0));
        assert!(successor_6.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[6].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(0, 3, 1)
        );

        let successor_7 = &tree.layers[successors[7].layer_index][successors[7].node_index];
        assert_eq!(successor_7.position, UVec3::new(0, 4, 0));
        assert!(successor_7.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[7].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(1, 3, 1)
        );

        let successor_8 = &tree.layers[successors[8].layer_index][successors[8].node_index];
        assert_eq!(successor_8.position, UVec3::new(0, 4, 0));
        assert!(successor_8.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[8].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(2, 3, 1)
        );

        let successor_9 = &tree.layers[successors[9].layer_index][successors[9].node_index];
        assert_eq!(successor_9.position, UVec3::new(0, 4, 0));
        assert!(successor_9.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[9].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(3, 3, 1)
        );

        let successor_10 = &tree.layers[successors[10].layer_index][successors[10].node_index];
        assert_eq!(successor_10.position, UVec3::new(0, 4, 0));
        assert!(successor_10.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[10].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(0, 3, 2)
        );

        let successor_11 = &tree.layers[successors[11].layer_index][successors[11].node_index];
        assert_eq!(successor_11.position, UVec3::new(0, 4, 0));
        assert!(successor_11.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[11].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(1, 3, 2)
        );

        let successor_12 = &tree.layers[successors[12].layer_index][successors[12].node_index];
        assert_eq!(successor_12.position, UVec3::new(0, 4, 0));
        assert!(successor_12.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[12].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(2, 3, 2)
        );

        let successor_13 = &tree.layers[successors[13].layer_index][successors[13].node_index];
        assert_eq!(successor_13.position, UVec3::new(0, 4, 0));
        assert!(successor_13.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[13].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(3, 3, 2)
        );

        let successor_14 = &tree.layers[successors[14].layer_index][successors[14].node_index];
        assert_eq!(successor_14.position, UVec3::new(0, 4, 0));
        assert!(successor_14.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[14].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(0, 3, 3)
        );

        let successor_15 = &tree.layers[successors[15].layer_index][successors[15].node_index];
        assert_eq!(successor_15.position, UVec3::new(0, 4, 0));
        assert!(successor_15.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[15].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(1, 3, 3)
        );

        let successor_16 = &tree.layers[successors[16].layer_index][successors[16].node_index];
        assert_eq!(successor_16.position, UVec3::new(0, 4, 0));
        assert!(successor_16.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[16].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(2, 3, 3)
        );

        let successor_17 = &tree.layers[successors[17].layer_index][successors[17].node_index];
        assert_eq!(successor_17.position, UVec3::new(0, 4, 0));
        assert!(successor_17.is_leaf);
        assert_eq!(
            MortonCode::from_u8(successors[17].subnode_index.unwrap())
                .decode()
                .unwrap(),
            UVec3::new(3, 3, 3)
        );

        let successor_18 = &tree.layers[successors[18].layer_index][successors[18].node_index];
        assert_eq!(successor_18.position, UVec3::new(0, 4, 4));

        let successor_19 = &tree.layers[successors[19].layer_index][successors[19].node_index];
        assert_eq!(successor_19.position, UVec3::new(4, 4, 0));

        let successor_20 = &tree.layers[successors[20].layer_index][successors[20].node_index];
        assert_eq!(successor_20.position, UVec3::new(4, 4, 4));
    }
}
