/// Table for quick lookup of position offsets for Morton codes from 0 to 7.
pub const OFFSETS_IN_MORTON_CODE_ORDER: [(u8, u8, u8); 8] = [
    (0, 0, 0),
    (1, 0, 0),
    (0, 1, 0),
    (1, 1, 0),
    (0, 0, 1),
    (1, 0, 1),
    (0, 1, 1),
    (1, 1, 1),
];

/// Table for quick lookup of neighbor connections between two split nodes.
///
/// There are 6 possible faces on a cube and they are indexed as follows:
/// 0 = Right (1, 0, 0)
/// 1 = Back (0, 0, 1)
/// 2 = Left (-1, 0, 0)
/// 3 = Front (0, 0, -1)
/// 4 = Bottom (0, 1, 0)
/// 5 = Top (0, -1, 0)
///
/// The first array contains Morton codes for all 4 subnodes on the indexed face.
/// The second array contains Morton codes for all 4 subnodes on the neighboring face of the other node.
///
/// The resulting Morton codes are relative to the parent node. So if knowing the
/// index of a first child node, these offsets can be used to determine all other 7
pub const NEIGHBOR_CONNECTIONS: [([usize; 4], [usize; 4]); 6] = [
    ([1, 5, 3, 7], [0, 4, 2, 6]),
    ([4, 6, 5, 7], [0, 2, 1, 3]),
    ([0, 4, 2, 6], [1, 5, 3, 7]),
    ([0, 2, 1, 3], [4, 6, 5, 7]),
    ([2, 6, 3, 7], [0, 4, 1, 5]),
    ([0, 4, 1, 5], [2, 6, 3, 7]),
];

/// Table for quick lookup of subnode positions and indexes for each face
///
/// There are 6 possible faces on a cube and they are indexed as follows:
/// 0 = Right (1, 0, 0)
/// 1 = Back (0, 0, 1)
/// 2 = Left (-1, 0, 0)
/// 3 = Front (0, 0, -1)
/// 4 = Bottom (0, 1, 0)
/// 5 = Top (0, -1, 0)
///
/// The first 3 values of the resulting array are the position of the subnode
/// relative to the parent node neighboring the indexed face.
/// The last value is the Morton code index of the subnode.
pub const NEIGHBOR_SUBNODES: [[(u8, u8, u8, u8); 16]; 6] = [
    [
        (0, 0, 0, 0),
        (0, 0, 1, 4),
        (0, 0, 2, 32),
        (0, 0, 3, 36),
        (0, 1, 0, 2),
        (0, 1, 1, 6),
        (0, 1, 2, 34),
        (0, 1, 3, 38),
        (0, 2, 0, 16),
        (0, 2, 1, 20),
        (0, 2, 2, 48),
        (0, 2, 3, 52),
        (0, 3, 0, 18),
        (0, 3, 1, 22),
        (0, 3, 2, 50),
        (0, 3, 3, 54),
    ],
    [
        (0, 0, 0, 0),
        (1, 0, 0, 1),
        (2, 0, 0, 8),
        (3, 0, 0, 9),
        (0, 1, 0, 2),
        (1, 1, 0, 3),
        (2, 1, 0, 10),
        (3, 1, 0, 11),
        (0, 2, 0, 16),
        (1, 2, 0, 17),
        (2, 2, 0, 24),
        (3, 2, 0, 25),
        (0, 3, 0, 18),
        (1, 3, 0, 19),
        (2, 3, 0, 26),
        (3, 3, 0, 27),
    ],
    [
        (3, 0, 0, 9),
        (3, 0, 1, 13),
        (3, 0, 2, 41),
        (3, 0, 3, 45),
        (3, 1, 0, 11),
        (3, 1, 1, 15),
        (3, 1, 2, 43),
        (3, 1, 3, 47),
        (3, 2, 0, 25),
        (3, 2, 1, 29),
        (3, 2, 2, 57),
        (3, 2, 3, 61),
        (3, 3, 0, 27),
        (3, 3, 1, 31),
        (3, 3, 2, 59),
        (3, 3, 3, 63),
    ],
    [
        (0, 0, 3, 36),
        (1, 0, 3, 37),
        (2, 0, 3, 44),
        (3, 0, 3, 45),
        (0, 1, 3, 38),
        (1, 1, 3, 39),
        (2, 1, 3, 46),
        (3, 1, 3, 47),
        (0, 2, 3, 52),
        (1, 2, 3, 53),
        (2, 2, 3, 60),
        (3, 2, 3, 61),
        (0, 3, 3, 54),
        (1, 3, 3, 55),
        (2, 3, 3, 62),
        (3, 3, 3, 63),
    ],
    [
        (0, 0, 0, 0),
        (1, 0, 0, 1),
        (2, 0, 0, 8),
        (3, 0, 0, 9),
        (0, 0, 1, 4),
        (1, 0, 1, 5),
        (2, 0, 1, 12),
        (3, 0, 1, 13),
        (0, 0, 2, 32),
        (1, 0, 2, 33),
        (2, 0, 2, 40),
        (3, 0, 2, 41),
        (0, 0, 3, 36),
        (1, 0, 3, 37),
        (2, 0, 3, 44),
        (3, 0, 3, 45),
    ],
    [
        (0, 3, 0, 18),
        (1, 3, 0, 19),
        (2, 3, 0, 26),
        (3, 3, 0, 27),
        (0, 3, 1, 22),
        (1, 3, 1, 23),
        (2, 3, 1, 30),
        (3, 3, 1, 31),
        (0, 3, 2, 50),
        (1, 3, 2, 51),
        (2, 3, 2, 58),
        (3, 3, 2, 59),
        (0, 3, 3, 54),
        (1, 3, 3, 55),
        (2, 3, 3, 62),
        (3, 3, 3, 63),
    ],
];

/// The 12 connections between the 8 subnodes of a node.
///
/// The first value is a face index through which the first subnode is connected.
/// The second value is a face index through which the second subnode is connected.
///
/// The second two values are the offsets to the subnodes that are connected. These offsets are
/// in Morton order relative to the parent node.
pub const SIBLING_CONNECTIONS: [(usize, usize, usize, usize); 12] = [
    (0, 2, 0, 1),
    (4, 5, 0, 2),
    (1, 3, 0, 4),
    (4, 5, 1, 3),
    (1, 3, 1, 5),
    (0, 2, 2, 3),
    (1, 3, 2, 6),
    (1, 3, 3, 7),
    (0, 2, 4, 5),
    (4, 5, 4, 6),
    (4, 5, 5, 7),
    (0, 2, 6, 7),
];

/// Lookup table containing the 6 neighbors of each subnode.
///
/// The array is indexed in Morton order.
/// The values in the neighbors array indexes of the
/// neighboring subnodes in Morton order.
pub const SUBNODE_NEIGHBORS: [[u8; 6]; 64] = [
    [1, 4, 9, 36, 2, 18],
    [8, 5, 0, 37, 3, 19],
    [3, 6, 11, 38, 16, 0],
    [10, 7, 2, 39, 17, 1],
    [5, 32, 13, 0, 6, 22],
    [12, 33, 4, 1, 7, 23],
    [7, 34, 15, 2, 20, 4],
    [14, 35, 6, 3, 21, 5],
    [9, 12, 1, 44, 10, 26],
    [0, 13, 8, 45, 11, 27],
    [11, 14, 3, 46, 24, 8],
    [2, 15, 10, 47, 25, 9],
    [13, 40, 5, 8, 14, 30],
    [4, 41, 12, 9, 15, 31],
    [15, 42, 7, 10, 28, 12],
    [6, 43, 14, 11, 29, 13],
    [17, 20, 25, 52, 18, 2],
    [24, 21, 16, 53, 19, 3],
    [19, 22, 27, 54, 0, 16],
    [26, 23, 18, 55, 1, 17],
    [21, 48, 29, 16, 22, 6],
    [28, 49, 20, 17, 23, 7],
    [23, 50, 31, 18, 4, 20],
    [30, 51, 22, 19, 5, 21],
    [25, 28, 17, 60, 26, 10],
    [16, 29, 24, 61, 27, 11],
    [27, 30, 19, 62, 8, 24],
    [18, 31, 26, 63, 9, 25],
    [29, 56, 21, 24, 30, 14],
    [20, 57, 28, 25, 31, 15],
    [31, 58, 23, 26, 12, 28],
    [22, 59, 30, 27, 13, 29],
    [33, 36, 41, 4, 34, 50],
    [40, 37, 32, 5, 35, 51],
    [35, 38, 43, 6, 48, 32],
    [42, 39, 34, 7, 49, 33],
    [37, 0, 45, 32, 38, 54],
    [44, 1, 36, 33, 39, 55],
    [39, 2, 47, 34, 52, 36],
    [46, 3, 38, 35, 53, 37],
    [41, 44, 33, 12, 42, 58],
    [32, 45, 40, 13, 43, 59],
    [43, 46, 35, 14, 56, 40],
    [34, 47, 42, 15, 57, 41],
    [45, 8, 37, 40, 46, 62],
    [36, 9, 44, 41, 47, 63],
    [47, 10, 39, 42, 60, 44],
    [38, 11, 46, 43, 61, 45],
    [49, 52, 57, 20, 50, 34],
    [56, 53, 48, 21, 51, 35],
    [51, 54, 59, 22, 32, 48],
    [58, 55, 50, 23, 33, 49],
    [53, 16, 61, 48, 54, 38],
    [60, 17, 52, 49, 55, 39],
    [55, 18, 63, 50, 36, 52],
    [62, 19, 54, 51, 37, 53],
    [57, 60, 49, 28, 58, 42],
    [48, 61, 56, 29, 59, 43],
    [59, 62, 51, 30, 40, 56],
    [50, 63, 58, 31, 41, 57],
    [61, 24, 53, 56, 62, 46],
    [52, 25, 60, 57, 63, 47],
    [63, 26, 55, 58, 44, 60],
    [54, 27, 62, 59, 45, 61],
];

/// Position offsets of the 6 neighbors of a node.
///
/// The indexes are:
///
/// 0 = Right side
/// 1 = Back side
/// 2 = Left side
/// 3 = Front side
/// 4 = Bottom side
/// 5 = Top side
pub const NEIGHBOR_POSITION_OFFSETS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, 0, -1),
    (0, 1, 0),
    (0, -1, 0),
];

pub const SUBNODE_POSITIONS: [(u8, u8, u8); 64] = [
    (0, 0, 0),
    (1, 0, 0),
    (0, 1, 0),
    (1, 1, 0),
    (0, 0, 1),
    (1, 0, 1),
    (0, 1, 1),
    (1, 1, 1),
    (2, 0, 0),
    (3, 0, 0),
    (2, 1, 0),
    (3, 1, 0),
    (2, 0, 1),
    (3, 0, 1),
    (2, 1, 1),
    (3, 1, 1),
    (0, 2, 0),
    (1, 2, 0),
    (0, 3, 0),
    (1, 3, 0),
    (0, 2, 1),
    (1, 2, 1),
    (0, 3, 1),
    (1, 3, 1),
    (2, 2, 0),
    (3, 2, 0),
    (2, 3, 0),
    (3, 3, 0),
    (2, 2, 1),
    (3, 2, 1),
    (2, 3, 1),
    (3, 3, 1),
    (0, 0, 2),
    (1, 0, 2),
    (0, 1, 2),
    (1, 1, 2),
    (0, 0, 3),
    (1, 0, 3),
    (0, 1, 3),
    (1, 1, 3),
    (2, 0, 2),
    (3, 0, 2),
    (2, 1, 2),
    (3, 1, 2),
    (2, 0, 3),
    (3, 0, 3),
    (2, 1, 3),
    (3, 1, 3),
    (0, 2, 2),
    (1, 2, 2),
    (0, 3, 2),
    (1, 3, 2),
    (0, 2, 3),
    (1, 2, 3),
    (0, 3, 3),
    (1, 3, 3),
    (2, 2, 2),
    (3, 2, 2),
    (2, 3, 2),
    (3, 3, 2),
    (2, 2, 3),
    (3, 2, 3),
    (2, 3, 3),
    (3, 3, 3),
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::morton_code::MortonCode;

    use super::*;

    #[test]
    fn test_offsets_in_morton_code_order() {
        (0_u8..6_u8).for_each(|i| {
            let offset_from_table = OFFSETS_IN_MORTON_CODE_ORDER[i as usize];
            let offset_from_morton_code = MortonCode::from_u8(i).decode().unwrap();
            let offset_from_morton_code = (
                offset_from_morton_code.x.try_into().unwrap(),
                offset_from_morton_code.y.try_into().unwrap(),
                offset_from_morton_code.z.try_into().unwrap(),
            );

            assert_eq!(offset_from_table, offset_from_morton_code);
        });
    }

    #[test]
    fn test_neighbor_connections() {
        // Check right face
        {
            let mut from_nodes = Vec::new();
            let mut to_nodes = Vec::new();

            for y in 0..2 {
                for z in 0..2 {
                    let from_node = MortonCode::encode_xyz(1, y, z).as_usize().unwrap();
                    let to_node = MortonCode::encode_xyz(0, y, z).as_usize().unwrap();
                    from_nodes.push(from_node);
                    to_nodes.push(to_node);
                }
            }

            assert_eq!(NEIGHBOR_CONNECTIONS[0].0, from_nodes.as_slice());
            assert_eq!(NEIGHBOR_CONNECTIONS[0].1, to_nodes.as_slice());
        }

        // Check back face
        {
            let mut from_nodes = Vec::new();
            let mut to_nodes = Vec::new();

            for x in 0..2 {
                for y in 0..2 {
                    let from_node = MortonCode::encode_xyz(x, y, 1).as_usize().unwrap();
                    let to_node = MortonCode::encode_xyz(x, y, 0).as_usize().unwrap();
                    from_nodes.push(from_node);
                    to_nodes.push(to_node);
                }
            }

            assert_eq!(NEIGHBOR_CONNECTIONS[1].0, from_nodes.as_slice());
            assert_eq!(NEIGHBOR_CONNECTIONS[1].1, to_nodes.as_slice());
        }

        // Check left face
        {
            let mut from_nodes = Vec::new();
            let mut to_nodes = Vec::new();

            for y in 0..2 {
                for z in 0..2 {
                    let from_node = MortonCode::encode_xyz(0, y, z).as_usize().unwrap();
                    let to_node = MortonCode::encode_xyz(1, y, z).as_usize().unwrap();
                    from_nodes.push(from_node);
                    to_nodes.push(to_node);
                }
            }

            assert_eq!(NEIGHBOR_CONNECTIONS[2].0, from_nodes.as_slice());
            assert_eq!(NEIGHBOR_CONNECTIONS[2].1, to_nodes.as_slice());
        }

        // Check front face
        {
            let mut from_nodes = Vec::new();
            let mut to_nodes = Vec::new();

            for x in 0..2 {
                for y in 0..2 {
                    let from_node = MortonCode::encode_xyz(x, y, 0).as_usize().unwrap();
                    let to_node = MortonCode::encode_xyz(x, y, 1).as_usize().unwrap();
                    from_nodes.push(from_node);
                    to_nodes.push(to_node);
                }
            }

            assert_eq!(NEIGHBOR_CONNECTIONS[3].0, from_nodes.as_slice());
            assert_eq!(NEIGHBOR_CONNECTIONS[3].1, to_nodes.as_slice());
        }

        // Check bottom face
        {
            let mut from_nodes = Vec::new();
            let mut to_nodes = Vec::new();

            for x in 0..2 {
                for z in 0..2 {
                    let from_node = MortonCode::encode_xyz(x, 1, z).as_usize().unwrap();
                    let to_node = MortonCode::encode_xyz(x, 0, z).as_usize().unwrap();
                    from_nodes.push(from_node);
                    to_nodes.push(to_node);
                }
            }

            assert_eq!(NEIGHBOR_CONNECTIONS[4].0, from_nodes.as_slice());
            assert_eq!(NEIGHBOR_CONNECTIONS[4].1, to_nodes.as_slice());
        }

        // Check top face
        {
            let mut from_nodes = Vec::new();
            let mut to_nodes = Vec::new();

            for x in 0..2 {
                for z in 0..2 {
                    let from_node = MortonCode::encode_xyz(x, 0, z).as_usize().unwrap();
                    let to_node = MortonCode::encode_xyz(x, 1, z).as_usize().unwrap();
                    from_nodes.push(from_node);
                    to_nodes.push(to_node);
                }
            }

            assert_eq!(NEIGHBOR_CONNECTIONS[5].0, from_nodes.as_slice());
            assert_eq!(NEIGHBOR_CONNECTIONS[5].1, to_nodes.as_slice());
        }
    }

    #[test]
    fn test_neighbor_subnodes() {
        // Check right face
        {
            let mut subnodes = Vec::new();

            for y in 0_u8..4_u8 {
                for z in 0_u8..4_u8 {
                    let subnode = MortonCode::encode_xyz(0, y.into(), z.into())
                        .as_u8()
                        .unwrap();
                    subnodes.push((0_u8, y, z, subnode));
                }
            }

            assert_eq!(NEIGHBOR_SUBNODES[0], subnodes.as_slice());
        }

        // Check back face
        {
            let mut subnodes = Vec::new();

            for y in 0_u8..4_u8 {
                for x in 0_u8..4_u8 {
                    let subnode = MortonCode::encode_xyz(x.into(), y.into(), 0)
                        .as_u8()
                        .unwrap();
                    subnodes.push((x, y, 0, subnode));
                }
            }

            assert_eq!(NEIGHBOR_SUBNODES[1], subnodes.as_slice());
        }

        // Check left face
        {
            let mut subnodes = Vec::new();

            for y in 0_u8..4_u8 {
                for z in 0_u8..4_u8 {
                    let subnode = MortonCode::encode_xyz(3, y.into(), z.into())
                        .as_u8()
                        .unwrap();
                    subnodes.push((3, y, z, subnode));
                }
            }

            assert_eq!(NEIGHBOR_SUBNODES[2], subnodes.as_slice());
        }

        // Check front face
        {
            let mut subnodes = Vec::new();

            for y in 0_u8..4_u8 {
                for x in 0_u8..4_u8 {
                    let subnode = MortonCode::encode_xyz(x.into(), y.into(), 3)
                        .as_u8()
                        .unwrap();
                    subnodes.push((x, y, 3, subnode));
                }
            }

            assert_eq!(NEIGHBOR_SUBNODES[3], subnodes.as_slice());
        }

        // Check bottom face
        {
            let mut subnodes = Vec::new();

            for z in 0_u8..4_u8 {
                for x in 0_u8..4_u8 {
                    let subnode = MortonCode::encode_xyz(x.into(), 0, z.into())
                        .as_u8()
                        .unwrap();
                    subnodes.push((x, 0, z, subnode));
                }
            }

            assert_eq!(NEIGHBOR_SUBNODES[4], subnodes.as_slice());
        }

        // Check top face
        {
            let mut subnodes = Vec::new();

            for z in 0_u8..4_u8 {
                for x in 0_u8..4_u8 {
                    let subnode = MortonCode::encode_xyz(x.into(), 3, z.into())
                        .as_u8()
                        .unwrap();
                    subnodes.push((x, 3, z, subnode));
                }
            }

            assert_eq!(NEIGHBOR_SUBNODES[5], subnodes.as_slice());
        }
    }

    #[test]
    fn test_sibling_connections() {
        const RIGHT_FACE: usize = 0;
        const BACK_FACE: usize = 1;
        const LEFT_FACE: usize = 2;
        const FRONT_FACE: usize = 3;
        const BOTTOM_FACE: usize = 4;
        const TOP_FACE: usize = 5;

        let mut duplicity_check = HashSet::new();
        let mut result = Vec::new();

        for x in 0_u8..2_u8 {
            for y in 0_u8..2_u8 {
                for z in 0_u8..2_u8 {
                    let node = MortonCode::encode_xyz(x.into(), y.into(), z.into())
                        .as_u8()
                        .unwrap();

                    for (face, subnode) in NEIGHBOR_POSITION_OFFSETS.iter().enumerate() {
                        let nx = i32::from(x) + subnode.0;
                        let ny = i32::from(y) + subnode.1;
                        let nz = i32::from(z) + subnode.2;

                        if !(0..=1).contains(&nx)
                            || !(0..=1).contains(&ny)
                            || !(0..=1).contains(&nz)
                        {
                            continue;
                        }

                        let opposite_face = {
                            if face == RIGHT_FACE {
                                LEFT_FACE
                            } else if face == BACK_FACE {
                                FRONT_FACE
                            } else if face == LEFT_FACE {
                                RIGHT_FACE
                            } else if face == FRONT_FACE {
                                BACK_FACE
                            } else if face == BOTTOM_FACE {
                                TOP_FACE
                            } else if face == TOP_FACE {
                                BOTTOM_FACE
                            } else {
                                unreachable!()
                            }
                        };

                        let sibling = MortonCode::encode_xyz(
                            u32::try_from(nx).unwrap(),
                            u32::try_from(ny).unwrap(),
                            u32::try_from(nz).unwrap(),
                        )
                        .as_u8()
                        .unwrap();

                        if duplicity_check.contains(&(node, sibling)) {
                            continue;
                        }

                        result.push((face, opposite_face, node as usize, sibling as usize));

                        duplicity_check.insert((node, sibling));
                        duplicity_check.insert((sibling, node));
                    }
                }
            }
        }

        result.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.3.cmp(&b.3)));

        assert_eq!(SIBLING_CONNECTIONS, result.as_slice());
    }

    #[test]
    fn test_subnode_neighbors() {
        let mut neighbors = Vec::new();

        for x in 0..4_u8 {
            for y in 0..4_u8 {
                for z in 0..4_u8 {
                    let node = MortonCode::encode_xyz(x.into(), y.into(), z.into())
                        .as_u8()
                        .unwrap();
                    let mut arr = [0; 6];

                    for (i, neighbor) in NEIGHBOR_POSITION_OFFSETS.iter().enumerate() {
                        let nx = u32::try_from((i32::from(x) + neighbor.0 + 4) % 4).unwrap();
                        let ny = u32::try_from((i32::from(y) + neighbor.1 + 4) % 4).unwrap();
                        let nz = u32::try_from((i32::from(z) + neighbor.2 + 4) % 4).unwrap();

                        let neighbor = MortonCode::encode_xyz(nx, ny, nz).as_u8().unwrap();

                        arr[i] = neighbor;
                    }

                    neighbors.push((node, arr));
                }
            }
        }

        neighbors.sort_by(|a, b| a.0.cmp(&b.0));

        let neighbors = neighbors
            .into_iter()
            .map(|(_, arr)| arr)
            .collect::<Vec<_>>();

        assert_eq!(SUBNODE_NEIGHBORS, neighbors.as_slice());
    }

    #[test]
    fn test_subnode_positions() {
        let mut positions = Vec::new();

        for x in 0..4_u8 {
            for y in 0..4_u8 {
                for z in 0..4_u8 {
                    positions.push((x, y, z));
                }
            }
        }

        positions.sort_by(|a, b| {
            let a = MortonCode::encode_xyz(a.0.into(), a.1.into(), a.2.into())
                .as_u8()
                .unwrap();
            let b = MortonCode::encode_xyz(b.0.into(), b.1.into(), b.2.into())
                .as_u8()
                .unwrap();

            a.cmp(&b)
        });

        assert_eq!(SUBNODE_POSITIONS, positions.as_slice());
    }
}
