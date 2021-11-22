mod board;

use std::convert::From;
use std::fmt;
/**
 * An immutable point on the Life board. In keeping with Eric Lippert's implementation, the Y-coordinate increases when going *up*, rather than the screen default of going *down*
 * ie, the origin is the "bottom left" rather than the "top left"
 */
#[derive(Clone, Debug, PartialEq)]
pub struct LifePoint {
    x: i64,
    y: i64,
}

impl LifePoint {
    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    pub fn new(x: i64, y: i64) {
        LifePoint { x, y };
    }
}

impl fmt::Display for LifePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for LifePoint {
    fn from(point_coords: (i32, i32)) -> LifePoint {
        LifePoint {
            x: point_coords.0 as i64,
            y: point_coords.1 as i64,
        }
    }
}

impl From<(i64, i64)> for LifePoint {
    fn from(point_coords: (i64, i64)) -> LifePoint {
        LifePoint {
            x: point_coords.0,
            y: point_coords.1,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn can_create_LifePoint() {
        let p = LifePoint { x: 1, y: 3 };
        assert_eq!(p.get_x(), 1);
        assert_eq!(p.get_y(), 3);
    }

    #[test]
    fn can_create_LifePoint_from_i32_tuple() {
        let p = LifePoint::from((1 as i32, 2 as i32));
        assert_eq!(p.get_x(), 1);
        assert_eq!(p.get_y(), 2);
    }

    #[test]
    fn can_create_LifePoint_from_i64_tuple() {
        let p = LifePoint::from((1 as i64, 2 as i64));
        assert_eq!(p.get_x(), 1);
        assert_eq!(p.get_y(), 2);
    }
}
