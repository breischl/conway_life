mod board;

use std::fmt;
/**
 * An immutable point on the Life "board". In keeping with Eric Lippert's implementation, the Y-coordinate increases when going *up*, rather than the screen default of going *down*
 * ie, the origin is the "bottom left" rather than the "top left"
 */
#[derive(Clone, Debug, PartialEq)]
pub struct LifePoint {
    x: f32,
    y: f32,
}

impl LifePoint {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
}

impl fmt::Display for LifePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_LifePoint() {
        let p = LifePoint { x: 1.2, y: 3.4 };
        assert_eq!(p.get_x(), 1.2);
        assert_eq!(p.get_y(), 3.4);
    }
}
