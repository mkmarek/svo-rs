use bevy_math::{IVec3, UVec3, Vec3};

pub trait DistanceSquared<T> {
    fn distance_squared(&self, other: &Self) -> T;
}

pub trait ManhattanDistance<T> {
    fn manhattan_distance(&self, other: &Self) -> T;
}

impl DistanceSquared<f32> for Vec3 {
    #[inline]
    fn distance_squared(&self, other: &Self) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }
}

impl DistanceSquared<i32> for IVec3 {
    #[inline]
    fn distance_squared(&self, other: &Self) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl DistanceSquared<u32> for UVec3 {
    #[inline]
    fn distance_squared(&self, other: &Self) -> u32 {
        let diff_x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let diff_y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        let diff_z = if self.z > other.z {
            self.z - other.z
        } else {
            other.z - self.z
        };

        diff_x.pow(2) + diff_y.pow(2) + diff_z.pow(2)
    }
}

impl ManhattanDistance<f32> for Vec3 {
    #[inline]
    fn manhattan_distance(&self, other: &Self) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl ManhattanDistance<i32> for IVec3 {
    #[inline]
    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl ManhattanDistance<u32> for UVec3 {
    #[inline]
    fn manhattan_distance(&self, other: &Self) -> u32 {
        let diff_x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let diff_y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        let diff_z = if self.z > other.z {
            self.z - other.z
        } else {
            other.z - self.z
        };

        diff_x + diff_y + diff_z
    }
}
