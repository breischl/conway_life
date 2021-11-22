use std::fmt;
/**
 * An immutable point on the Life "board". In keeping with Eric Lippert's implementation, the Y-coordinate increases when going *up*, rather than the screen default of going *down*
 * ie, the origin is the "bottom left" rather than the "top left"
 */
#[derive(Clone, Debug)]
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
