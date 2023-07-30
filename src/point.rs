use std::ops::{Add, Div, Mul, Shl, Shr, Sub, AddAssign};

/// A point in 3D space.
/// 
/// This is a generic type that can be used with any numeric type.
/// 
/// # Examples
/// 
/// ```
/// use svo_rs::Point;
/// 
/// let point = Point::new(1, 2, 3);
/// 
/// assert_eq!(point.x, 1);
/// assert_eq!(point.y, 2);
/// assert_eq!(point.z, 3);
/// 
/// let other_point = Point::new(4, 5, 6) + point;
/// 
/// assert_eq!(other_point.x, 5);
/// assert_eq!(other_point.y, 7);
/// assert_eq!(other_point.z, 9);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default, Eq, Hash)]
#[repr(C)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T>
where
    T: Copy,
{
    /// Creates a new point.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::Point;
    /// 
    /// let point = Point::new(1, 2, 3);
    /// 
    /// assert_eq!(point.x, 1);
    /// assert_eq!(point.y, 2);
    /// assert_eq!(point.z, 3);
    /// ```
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Creates a new point from an array.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::Point;
    /// 
    /// let point = Point::from_array([1, 2, 3]);
    /// 
    /// assert_eq!(point.x, 1);
    /// assert_eq!(point.y, 2);
    /// assert_eq!(point.z, 3);
    /// ```
    #[inline]
    pub fn from_array(array: [T; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }

    /// Converts the point to an array.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::Point;
    /// 
    /// let point = Point::new(1, 2, 3);
    /// 
    /// assert_eq!(point.to_array(), [1, 2, 3]);
    /// ```
    #[inline]
    pub fn to_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    /// Returns the maximum element of the point.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::Point;
    /// 
    /// let point = Point::new(1, 2, 3);
    /// 
    /// assert_eq!(point.max_element(), 3);
    /// ```
    #[inline]
    pub fn max_element(&self) -> T
    where
        T: Ord,
    {
        self.x.max(self.y).max(self.z)
    }

    /// Returns the maximum of the point and another point.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::Point;
    /// 
    /// let point = Point::new(1, 5, 3);
    /// let other_point = Point::new(4, 2, 6);
    /// 
    /// assert_eq!(point.max(other_point), Point::new(4, 5, 6));
    #[inline]
    pub fn max(&self, other: Self) -> Self
    where
        T: Ord,
    {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    /// Returns the minimum of the point and another point.
    ///
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::Point;
    /// 
    /// let point = Point::new(1, 5, 3);
    /// let other_point = Point::new(4, 2, 6);
    /// 
    /// assert_eq!(point.min(other_point), Point::new(1, 2, 3));
    /// ```
    #[inline]
    pub fn min(&self, other: Self) -> Self
    where
        T: Ord,
    {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    /// Returns the squared length of the vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::IPoint;
    /// 
    /// let point = IPoint::new(3, 4, 5);
    /// 
    /// assert_eq!(point.length_squared(), 50);
    /// ```
    pub fn length_squared(&self) -> T where T: Mul<Output = T> + Add<Output = T> + Copy {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<T> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> Mul for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Sub<T> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T> Div for Point<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> Div<T> for Point<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Shl<T> for Point<T>
where
    T: Shl<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn shl(self, rhs: T) -> Self::Output {
        Self {
            x: self.x << rhs,
            y: self.y << rhs,
            z: self.z << rhs,
        }
    }
}

impl<T> PartialEq<&(T, T, T)> for Point<T> where T: PartialEq {
    fn eq(&self, other: &&(T, T, T)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == other.2
    }
}

impl<T> Shr<T> for Point<T>
where
    T: Shr<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn shr(self, rhs: T) -> Self::Output {
        Self {
            x: self.x >> rhs,
            y: self.y >> rhs,
            z: self.z >> rhs,
        }
    }
}

impl AddAssign for Point<i32> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> From<[T; 3]> for Point<T> where T: Copy {
    #[inline]
    fn from(array: [T; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<T> From<(T, T, T)> for Point<T> {
    #[inline]
    fn from(tuple: (T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T> From<Point<T>> for (T, T, T) {
    #[inline]
    fn from(point: Point<T>) -> Self {
        (point.x, point.y, point.z)
    }
}

impl Point<f32> {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const MAX: Self = Self {
        x: f32::MAX,
        y: f32::MAX,
        z: f32::MAX,
    };

    pub const MIN: Self = Self {
        x: f32::MIN,
        y: f32::MIN,
        z: f32::MIN,
    };

    /// Returns the length of the vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::FPoint;
    /// 
    /// let point = FPoint::new(3.0, 4.0, 0.0);
    /// assert_eq!(point.length(), 5.0);
    /// ```
    #[inline]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Floors each component of the vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::FPoint;
    /// 
    /// let point = FPoint::new(3.5, 4.5, 5.5);
    /// 
    /// assert_eq!(point.floor(), FPoint::new(3.0, 4.0, 5.0));
    /// ```
    #[inline]
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    /// Ceils each component of the vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::FPoint;
    /// 
    /// let point = FPoint::new(3.5, 4.5, 5.5);
    /// 
    /// assert_eq!(point.ceil(), FPoint::new(4.0, 5.0, 6.0));
    /// ```
    #[inline]
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    /// Converts the vector to an `i32` vector.
    ///
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::{FPoint, Point};
    /// 
    /// let point = FPoint::new(3.5, 4.5, 5.5);
    /// 
    /// assert_eq!(point.to_i32(), Point::new(3, 4, 5));
    #[inline]
    pub fn to_i32(&self) -> Point<i32> {
        Point {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }
}

impl Point<i32> {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };

    pub const MAX: Self = Self {
        x: i32::MAX,
        y: i32::MAX,
        z: i32::MAX,
    };

    pub const MIN: Self = Self {
        x: i32::MIN,
        y: i32::MIN,
        z: i32::MIN,
    };

    /// Returns the manhattan length of the vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::IPoint;
    /// 
    /// let point = IPoint::new(3, 4, 5);
    /// 
    /// assert_eq!(point.manhattan_length(), 12);
    /// ```
    #[inline]
    pub fn manhattan_length(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    /// Converts the vector to an `u32` vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::{IPoint, UPoint};
    /// 
    /// let point = IPoint::new(3, 4, 5);
    /// 
    /// assert_eq!(point.to_u32(), UPoint::new(3, 4, 5));
    /// ```
    #[inline]
    pub fn to_u32(&self) -> Point<u32> {
        Point {
            x: self.x as u32,
            y: self.y as u32,
            z: self.z as u32,
        }
    }
}

impl Point<u32> {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };

    pub const MAX: Self = Self {
        x: u32::MAX,
        y: u32::MAX,
        z: u32::MAX,
    };

    pub const MIN: Self = Self {
        x: u32::MIN,
        y: u32::MIN,
        z: u32::MIN,
    };

    /// Returns the manhattan length of the vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::UPoint;
    /// 
    /// let point = UPoint::new(3, 4, 5);
    /// 
    /// assert_eq!(point.manhattan_length(), 12);
    /// ```
    #[inline]
    pub fn manhattan_length(&self) -> u32 {
        self.x + self.y + self.z
    }

    /// Converts the vector to an `i32` vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use svo_rs::{IPoint, UPoint};
    /// 
    /// let point = UPoint::new(3, 4, 5);
    /// 
    /// assert_eq!(point.to_i32(), IPoint::new(3, 4, 5));
    /// ```
    #[inline]
    pub fn to_i32(&self) -> Point<i32> {
        Point {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }
}

#[cfg(feature = "bevy")]
impl From<Point<f32>> for bevy_math::Vec3 {
    #[inline]
    fn from(point: Point<f32>) -> Self {
        Self::new(point.x, point.y, point.z)
    }
}

#[cfg(feature = "bevy")]
impl From<bevy_math::Vec3> for Point<f32> {
    #[inline]
    fn from(point: bevy_math::Vec3) -> Self {
        Self::new(point.x, point.y, point.z)
    }
}


/// A point with `f32` components.
/// 
/// # Examples
/// 
/// ```
/// use svo_rs::FPoint;
/// 
/// let point = FPoint::new(3.5, 4.5, 5.5);
/// 
/// assert_eq!(point.x, 3.5);
/// assert_eq!(point.y, 4.5);
/// assert_eq!(point.z, 5.5);
/// ```
pub type FPoint = Point<f32>;

/// A point with `i32` components.
/// 
/// # Examples
/// 
/// ```
/// use svo_rs::IPoint;
/// 
/// let point = IPoint::new(3, 4, 5);
/// 
/// assert_eq!(point.x, 3);
/// assert_eq!(point.y, 4);
/// assert_eq!(point.z, 5);
/// ```
pub type IPoint = Point<i32>;

/// A point with `u32` components.
/// 
/// # Examples
/// 
/// ```
/// use svo_rs::UPoint;
/// 
/// let point = UPoint::new(3, 4, 5);
/// 
/// assert_eq!(point.x, 3);
/// assert_eq!(point.y, 4);
/// assert_eq!(point.z, 5);
/// ```
pub type UPoint = Point<u32>;
