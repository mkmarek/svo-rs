#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LineClippingResult {
    Inside,
    Outside,
    Clip,
}

/// Cohen-Sutherland line clipping algorithm
///
/// Given two points `l1` and `l2` that define a line segment, and a bounding box defined by `min` and `max`,
/// returns a `LineClippingResult` that indicates whether the line segment is inside, outside, or needs to be clipped.
pub fn cohen_sutherland<T>(
    l1: &[T; 3],
    l2: &[T; 3],
    min: &[T; 3],
    max: &[T; 3],
) -> LineClippingResult
where
    T: PartialOrd + Copy,
{
    let code1 = assign_region_code(l1, min, max);
    let code2 = assign_region_code(l2, min, max);

    if code1 == 0 && code2 == 0 {
        LineClippingResult::Inside
    } else if code1 & code2 != 0 {
        LineClippingResult::Outside
    } else {
        LineClippingResult::Clip
    }
}

const LEFT: u8 = 0b0001;
const RIGHT: u8 = 0b0010;
const BOTTOM: u8 = 0b0100;
const TOP: u8 = 0b1000;
const NEAR: u8 = 0b0001_0000;
const FAR: u8 = 0b0010_0000;

/// Assigns a region code to a point based on its position relative to a bounding box.
fn assign_region_code<T>(p: &[T; 3], min: &[T; 3], max: &[T; 3]) -> u8
where
    T: PartialOrd,
{
    let mut code = 0;
    if p[0] < min[0] {
        code |= LEFT;
    } else if p[0] > max[0] {
        code |= RIGHT;
    }
    if p[1] < min[1] {
        code |= BOTTOM;
    } else if p[1] > max[1] {
        code |= TOP;
    }
    if p[2] < min[2] {
        code |= NEAR;
    } else if p[2] > max[2] {
        code |= FAR;
    }
    code
}

#[cfg(test)]
mod tests {
    use bevy_math::Vec3;

    use super::*;

    #[test]
    fn test_assign_region_code() {
        let min = Vec3::new(-1.0, -1.0, -1.0).to_array();
        let max = Vec3::new(1.0, 1.0, 1.0).to_array();

        let p1 = Vec3::new(-2.0, 0.0, 0.0).to_array();
        let p2 = Vec3::new(0.0, 2.0, 0.0).to_array();
        let p3 = Vec3::new(0.0, 0.0, 2.0).to_array();

        assert_eq!(assign_region_code(&p1, &min, &max), LEFT);
        assert_eq!(assign_region_code(&p2, &min, &max), TOP);
        assert_eq!(assign_region_code(&p3, &min, &max), FAR);
    }

    #[test]
    fn test_cohen_sutherland() {
        let l1 = Vec3::new(0.0, 0.0, 0.0).to_array();
        let l2 = Vec3::new(1.0, 1.0, 1.0).to_array();

        let min = Vec3::new(-1.0, -1.0, -1.0).to_array();
        let max = Vec3::new(1.0, 1.0, 1.0).to_array();

        let result = cohen_sutherland(&l1, &l2, &min, &max);

        assert_eq!(result, LineClippingResult::Inside);
    }
}
