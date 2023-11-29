use std::ops::Deref;

use crate::morton_code::MortonCode;

/// Represents data in a leaf node as a 4x4x4 cube.
///
/// Encodes 64 booleans in a single u64. Each of them represents is a voxel at a specific position is filed.
/// Positions are encoded using Morton code, where the resulting index is the index of the bit in the u64.
pub struct CompoundNode(u64);

impl CompoundNode {
    /// Top face indexes in a 4x4x4 cube encoded into a u64.
    const TOP_FACE_POSITIONS: u64 = 0b11_0011_0011_0011_0000_0000_0000_0000_0011_0011_0011_0011;

    /// Bottom face indexes in a 4x4x4 cube encoded into a u64.
    const BOTTOM_FACE_POSITIONS: u64 =
        0b1100_1100_1100_1100_0000_0000_0000_0000_1100_1100_1100_1100_0000_0000_0000_0000;

    /// Left face indexes in a 4x4x4 cube encoded into a u64.
    const LEFT_FACE_POSITIONS: u64 =
        0b101_0101_0000_0000_0101_0101_0000_0000_0101_0101_0000_0000_0101_0101;

    /// Right face indexes in a 4x4x4 cube encoded into a u64.
    const RIGHT_FACE_POSITIONS: u64 =
        0b1010_1010_0000_0000_1010_1010_0000_0000_1010_1010_0000_0000_1010_1010_0000_0000;

    /// Front face indexes in a 4x4x4 cube encoded into a u64.
    const FRONT_FACE_POSITIONS: u64 = 0b1111_0000_1111_0000_1111_0000_1111;

    /// Back face indexes in a 4x4x4 cube encoded into a u64.
    const BACK_FACE_POSITIONS: u64 =
        0b1111_0000_1111_0000_1111_0000_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000;

    /// All face indexes in a 4x4x4 cube encoded into an array of u64s.
    const FACE_POSITIONS: [u64; 6] = [
        Self::RIGHT_FACE_POSITIONS,
        Self::BACK_FACE_POSITIONS,
        Self::LEFT_FACE_POSITIONS,
        Self::FRONT_FACE_POSITIONS,
        Self::BOTTOM_FACE_POSITIONS,
        Self::TOP_FACE_POSITIONS,
    ];

    /// Creates a new empty node.
    #[inline]
    pub fn new() -> Self {
        Self(0)
    }

    /// Returns true if the `node_index` is part of a cube's face defined by `face_index`.
    #[inline]
    pub fn is_face(node_index: u8, face_index: usize) -> bool {
        Self::FACE_POSITIONS[face_index] & (1 << node_index) != 0
    }

    /// Return true if the compound node is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Return true if the compound node is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.0 == u64::MAX
    }

    /// Sets voxel at a specific position (`x`, `y`, `z`) to `value` (true = filled, false = empty)
    #[inline]
    pub fn set(&mut self, x: u32, y: u32, z: u32, value: bool) {
        let index = MortonCode::encode_xyz(x, y, z).as_u64();

        if value {
            self.0 |= 1 << index;
        } else {
            self.0 &= !(1 << index);
        }
    }

    /// Gets voxel at a specific position (`x`, `y`, `z`)
    ///
    /// Returns true if the voxel is filled, false if it is empty.
    #[allow(dead_code)]
    #[inline]
    pub fn get(&self, x: u32, y: u32, z: u32) -> bool {
        let index = MortonCode::encode_xyz(x, y, z).as_u64();

        self.0 & (1 << index) != 0
    }

    /// Gets voxel at a specific Morton code index `index`
    ///
    /// Returns true if the voxel is filled, false if it is empty.
    #[inline]
    pub fn get_by_index(&self, index: u8) -> bool {
        self.0 & (1 << index) != 0
    }

    /// Gets all indexes of filled voxels.
    #[inline]
    pub fn get_occupied_indexes(&self) -> Vec<u8> {
        let mut indexes = Vec::with_capacity(64);

        for i in 0..64 {
            if self.0 & (1 << i) != 0 {
                indexes.push(i);
            }
        }

        indexes
    }
}

impl Deref for CompoundNode {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
