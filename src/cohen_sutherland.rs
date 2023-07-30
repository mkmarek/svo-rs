use crate::Point;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CohenSutherlandResult {
    Inside,
    Outside,
    Clip
}

/// Cohen-Sutherland line clipping algorithm
/// 
/// Given two points `l1` and `l2` that define a line segment, and a bounding box defined by `min` and `max`,
/// returns a `CohenSutherlandResult` that indicates whether the line segment is inside, outside, or needs to be clipped.
pub fn cohen_sutherland<T>(l1: Point<T>, l2: Point<T>, min: Point<T>, max: Point<T>) -> CohenSutherlandResult
where
    T: PartialOrd + Copy,
{
    let code1 = assign_region_code(l1, min, max);
    let code2 = assign_region_code(l2, min, max);

    if code1 == 0 && code2 == 0 {
        CohenSutherlandResult::Inside
    } else if code1 & code2 != 0 {
        CohenSutherlandResult::Outside
    } else {
        CohenSutherlandResult::Clip
    }
}

const LEFT: u8 = 0b0001;
const RIGHT: u8 = 0b0010;
const BOTTOM: u8 = 0b0100;
const TOP: u8 = 0b1000;
const NEAR: u8 = 0b0001_0000;
const FAR: u8 = 0b0010_0000;

/// Assigns a region code to a point based on its position relative to a bounding box.
fn assign_region_code<T>(p: Point<T>, min: Point<T>, max: Point<T>) -> u8
where
    T: PartialOrd,
{
    let mut code = 0;
    if p.x < min.x {
        code |= LEFT;
    } else if p.x > max.x {
        code |= RIGHT;
    }
    if p.y < min.y {
        code |= BOTTOM;
    } else if p.y > max.y {
        code |= TOP;
    }
    if p.z < min.z {
        code |= NEAR;
    } else if p.z > max.z {
        code |= FAR;
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_region_code() {
        let min = Point::new(-1.0, -1.0, -1.0);
        let max = Point::new(1.0, 1.0, 1.0);

        let p1 = Point::new(-2.0, 0.0, 0.0);
        let p2 = Point::new(0.0, 2.0, 0.0);
        let p3 = Point::new(0.0, 0.0, 2.0);

        assert_eq!(assign_region_code(p1, min, max), LEFT);
        assert_eq!(assign_region_code(p2, min, max), TOP);
        assert_eq!(assign_region_code(p3, min, max), FAR);
    }

    #[test]
    fn test_cohen_sutherland() {
        let l1 = Point::new(0.0, 0.0, 0.0);
        let l2 = Point::new(1.0, 1.0, 1.0);

        let min = Point::new(-1.0, -1.0, -1.0);
        let max = Point::new(1.0, 1.0, 1.0);

        let result = cohen_sutherland(l1, l2, min, max);

        assert_eq!(result, CohenSutherlandResult::Inside);
    }
}