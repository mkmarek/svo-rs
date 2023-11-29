use std::error::Error;

use bevy_math::{IVec3, UVec3};

/// Collection of voxels that represent a mesh
pub struct VoxelizedMesh {
    voxels: Vec<UVec3>,
    voxel_size: f32,
    left_top_corner: IVec3,
}

impl VoxelizedMesh {
    /// Creates a new voxelized mesh
    ///
    /// # Example
    ///
    /// ```
    /// use svo_rs::{VoxelizedMesh};
    /// use bevy_math::{IVec3, UVec3};
    ///
    /// let voxels = vec![
    ///  UVec3::new(0, 0, 0),
    ///  UVec3::new(1, 0, 0),
    ///  UVec3::new(0, 1, 0),
    ///  UVec3::new(0, 0, 1),
    /// ];
    ///
    /// let voxelized_mesh = VoxelizedMesh::new(voxels, 1.0, IVec3::new(0, 0, 0));
    /// ```
    #[must_use]
    pub fn new(voxels: Vec<UVec3>, voxel_size: f32, left_top_corner: IVec3) -> Self {
        Self {
            voxels,
            voxel_size,
            left_top_corner,
        }
    }

    /// Returns the voxel size of each voxel in the mesh
    #[must_use]
    pub fn voxel_size(&self) -> f32 {
        self.voxel_size
    }

    /// Create a sphere voxelized mesh
    ///
    /// # Example
    ///
    /// ```
    /// use svo_rs::{VoxelizedMesh};
    /// use bevy_math::IVec3;
    ///
    /// let voxelized_mesh = VoxelizedMesh::sphere(1.0, 1.0, IVec3::new(0, 0, 0));
    /// ```
    #[must_use]
    pub fn sphere(radius: f32, voxel_size: f32, position: IVec3) -> Self {
        let mut voxels = Vec::new();
        #[allow(clippy::cast_possible_truncation)]
        let radius = (radius / voxel_size).ceil() as i32;

        let left_top_corner = position - radius;

        let mut x = -radius;
        while x <= radius {
            let mut y = -radius;
            while y <= radius {
                let mut z = -radius;
                while z <= radius {
                    if (x * x + y * y + z * z) <= (radius * radius) {
                        #[allow(clippy::cast_sign_loss)]
                        voxels.push(UVec3::new(
                            (x - left_top_corner.x) as u32,
                            (y - left_top_corner.y) as u32,
                            (z - left_top_corner.z) as u32,
                        ));
                    }

                    z += 1;
                }

                y += 1;
            }

            x += 1;
        }

        Self::new(voxels, voxel_size, left_top_corner)
    }

    /// Converts a bevy mesh to a voxelized mesh
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_render::prelude::shape::UVSphere;
    /// use svo_rs::{VoxelizedMesh};
    ///
    /// let sphere = Mesh::from(UVSphere::default());
    /// let mesh = VoxelizedMesh::from_mesh(&sphere, Transform::IDENTITY.compute_matrix(), 1.0)
    ///     .expect("Failed to voxelize mesh");
    /// ```
    ///
    /// # Errors
    /// Returns an error if the mesh has no indices or vertices
    ///
    #[cfg(feature = "bevy")]
    pub fn from_mesh(
        mesh: &bevy_render::prelude::Mesh,
        transform: bevy_math::Mat4,
        voxel_size: f32,
    ) -> Result<VoxelizedMesh, VoxelizeError> {
        let indices = mesh
            .indices()
            .ok_or(VoxelizeError("Mesh has no indices".to_string()))?;
        let vertices = mesh
            .attribute(bevy_render::prelude::Mesh::ATTRIBUTE_POSITION)
            .ok_or(VoxelizeError("Mesh has no vertices".to_string()))?
            .as_float3()
            .ok_or(VoxelizeError(
                "Error converting to float3 vertices".to_string(),
            ))?;

        let triangles = {
            let mut result = Vec::with_capacity(indices.len() / 3);
            for i in (0..indices.len()).step_by(3) {
                result.push(match indices {
                    bevy_render::mesh::Indices::U16(indices) => [
                        transform.transform_point3(vertices[indices[i] as usize].into()),
                        transform.transform_point3(vertices[indices[i + 1] as usize].into()),
                        transform.transform_point3(vertices[indices[i + 2] as usize].into()),
                    ],
                    bevy_render::mesh::Indices::U32(indices) => [
                        transform.transform_point3(vertices[indices[i] as usize].into()),
                        transform.transform_point3(vertices[indices[i + 1] as usize].into()),
                        transform.transform_point3(vertices[indices[i + 2] as usize].into()),
                    ],
                });
            }

            result
        };

        let triangle_min = triangles.iter().fold(
            bevy_math::Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            |min, v| min.min(v[0]).min(v[1]).min(v[2]),
        );

        let mut voxels = std::collections::HashSet::new();

        #[allow(clippy::cast_possible_truncation)]
        let left_top_corner = IVec3::new(
            (triangle_min.x / voxel_size).round() as i32 - 1,
            (triangle_min.y / voxel_size).round() as i32 - 1,
            (triangle_min.z / voxel_size).round() as i32 - 1,
        );

        for triangle in &triangles {
            let triangle_voxels = triangle_to_voxels(triangle, voxel_size, left_top_corner);

            for voxel in triangle_voxels {
                voxels.insert(voxel);
            }
        }

        Ok(VoxelizedMesh {
            voxels: voxels.into_iter().collect(),
            voxel_size,
            left_top_corner,
        })
    }

    /// Draws the voxelized mesh using bevy gizmos
    #[cfg(feature = "bevy")]
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_gizmos(&self, gizmos: &mut bevy_gizmos::prelude::Gizmos) {
        for voxel in &self.voxels {
            let voxel = voxel.as_vec3() * self.voxel_size;
            let left_top_corner = self.left_top_corner.as_vec3() * self.voxel_size;
            let half_size = bevy_math::Vec3::ONE * self.voxel_size / 2.0;

            gizmos.cuboid(
                bevy_transform::prelude::Transform::from_translation(
                    left_top_corner + voxel + half_size,
                )
                .with_scale(bevy_math::Vec3::ONE * self.voxel_size),
                bevy_render::prelude::Color::RED,
            );
        }
    }

    /// Returns the voxels of the mesh
    #[must_use]
    pub fn voxels(&self) -> Vec<IVec3> {
        self.voxels
            .iter()
            .map(|v| v.as_ivec3() + self.left_top_corner)
            .collect()
    }
}

#[derive(Debug)]
pub struct VoxelizeError(String);

impl Error for VoxelizeError {}

impl std::fmt::Display for VoxelizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Voxelize error: {:#?}", self.0)
    }
}

#[cfg(feature = "bevy")]
#[allow(clippy::cast_precision_loss)]
fn triangle_to_voxels(
    triangle: &[bevy_math::Vec3; 3],
    voxel_size: f32,
    left_top_corner: IVec3,
) -> Vec<UVec3> {
    let mut result = Vec::new();
    let left_top_corner = bevy_math::Vec3::new(
        left_top_corner.x as f32,
        left_top_corner.y as f32,
        left_top_corner.z as f32,
    ) * voxel_size
        + (bevy_math::Vec3::ONE * voxel_size / 2.0);

    let min = triangle.iter().fold(
        bevy_math::Vec3::new(f32::MAX, f32::MAX, f32::MAX),
        |min, v| bevy_math::Vec3::new(min.x.min(v.x), min.y.min(v.y), min.z.min(v.z)),
    ) - left_top_corner;

    let max = triangle.iter().fold(
        bevy_math::Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        |max, v| bevy_math::Vec3::new(max.x.max(v.x), max.y.max(v.y), max.z.max(v.z)),
    ) - left_top_corner;

    let min = (min / voxel_size).floor().as_uvec3();
    let max = (max / voxel_size).ceil().as_uvec3();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let cube = bevy_math::Vec3::new(x as f32, y as f32, z as f32) * voxel_size
                    + left_top_corner;

                if cube_triangle_intersection(cube, voxel_size / 2.0, triangle) {
                    result.push(UVec3::new(x, y, z));
                }
            }
        }
    }

    result
}

#[cfg(feature = "bevy")]
// Algorithm obtained from https://gdbooks.gitbooks.io/3dcollisions/content/Chapter4/aabb-triangle.html
fn cube_triangle_intersection(
    cube: bevy_math::Vec3,
    cube_size: f32,
    triangle: &[bevy_math::Vec3; 3],
) -> bool {
    // Get the triangle points as vectors
    let mut v0 = triangle[0];
    let mut v1 = triangle[1];
    let mut v2 = triangle[2];

    // Convert AABB to center-extents form
    let c = cube;
    let e = bevy_math::Vec3::ONE * cube_size;

    // Translate the triangle as conceptually moving the AABB to origin
    // This is the same as we did with the point in triangle test
    v0 -= c;
    v1 -= c;
    v2 -= c;

    // Compute the edge vectors of the triangle  (ABC)
    // That is, get the lines between the points as vectors
    let f0 = v1 - v0; // B - A
    let f1 = v2 - v1; // C - B
    let f2 = v0 - v2; // A - C

    // Compute the face normals of the AABB, because the AABB
    // is at center, and of course axis aligned, we know that
    // it's normals are the X, Y and Z axis.
    let u0 = bevy_math::Vec3::new(1.0, 0.0, 0.0);
    let u1 = bevy_math::Vec3::new(0.0, 1.0, 0.0);
    let u2 = bevy_math::Vec3::new(0.0, 0.0, 1.0);

    // There are a total of 13 axis to test!

    // We first test against 9 axis, these axis are given by
    // cross product combinations of the edges of the triangle
    // and the edges of the AABB. You need to get an axis testing
    // each of the 3 sides of the AABB against each of the 3 sides
    // of the triangle. The result is 9 axis of seperation
    // https://awwapp.com/b/umzoc8tiv/

    // Compute the 9 axis
    let axis_u0_f0 = u0.cross(f0);
    let axis_u0_f1 = u0.cross(f1);
    let axis_u0_f2 = u0.cross(f2);

    let axis_u1_f0 = u1.cross(f0);
    let axis_u1_f1 = u1.cross(f1);
    let axis_u1_f2 = u2.cross(f2);

    let axis_u2_f0 = u2.cross(f0);
    let axis_u2_f1 = u2.cross(f1);
    let axis_u2_f2 = u2.cross(f2);

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u0_f0) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u0_f1) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u0_f2) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u1_f0) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u1_f1) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u1_f2) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u2_f0) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u2_f1) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, axis_u2_f2) {
        return false;
    }

    // Next, we have 3 face normals from the AABB
    // for these tests we are conceptually checking if the bounding box
    // of the triangle intersects the bounding box of the AABB
    // that is to say, the seperating axis for all tests are axis aligned:
    // axis1: (1, 0, 0), axis2: (0, 1, 0), axis3 (0, 0, 1)

    // Test the 3 face normals from the AABB
    if !sat_test(v0, v1, v2, u0, u1, u2, e, u0) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, u1) {
        return false;
    }

    if !sat_test(v0, v1, v2, u0, u1, u2, e, u2) {
        return false;
    }

    // Do the SAT given the 3 primary axis of the AABB
    // You already have vectors for this: u0, u1 & u2

    // Finally, we have one last axis to test, the face normal of the triangle
    // We can get the normal of the triangle by crossing the first two line segments
    let triangle_normal = f0.cross(f1);

    // Test the face normal of the triangle
    if !sat_test(v0, v1, v2, u0, u1, u2, e, triangle_normal) {
        return false;
    }

    // Passed testing for all 13 seperating axis that exist!
    true
}

#[cfg(feature = "bevy")]
#[allow(clippy::too_many_arguments)]
fn sat_test(
    v0: bevy_math::Vec3,
    v1: bevy_math::Vec3,
    v2: bevy_math::Vec3,
    u0: bevy_math::Vec3,
    u1: bevy_math::Vec3,
    u2: bevy_math::Vec3,
    e: bevy_math::Vec3,
    axis: bevy_math::Vec3,
) -> bool {
    // Testing axis: axis_u0_f0
    // Project all 3 vertices of the triangle onto the Seperating axis
    let p0 = v0.dot(axis);
    let p1 = v1.dot(axis);
    let p2 = v2.dot(axis);
    // Project the AABB onto the seperating axis
    // We don't care about the end points of the prjection
    // just the length of the half-size of the AABB
    // That is, we're only casting the extents onto the
    // seperating axis, not the AABB center. We don't
    // need to cast the center, because we know that the
    // aabb is at origin compared to the triangle!
    let r = e.x * (u0.dot(axis)).abs() + e.y * (u1.dot(axis)).abs() + e.z * (u2.dot(axis)).abs();
    // Now do the actual test, basically see if either of
    // the most extreme of the triangle points intersects r
    // You might need to write Min & Max functions that take 3 arguments
    if p0.min(p1).min(p2).max(-(p0.max(p1).max(p2))) > r {
        // This means BOTH of the points of the projected triangle
        // are outside the projected half-length of the AABB
        // Therefore the axis is seperating and we can exit
        return false;
    }

    true
}
