use crate::pattern::Pattern;
use std::convert::From;

pub trait LifeBoard {
    fn is_live(&self, x: i64, y: i64) -> bool;
    fn set_liveness(&mut self, x: i64, y: i64, is_live: bool);
    fn count_live_neighbors(&self, x: i64, y: i64) -> u8;
    fn step_one(&mut self);

    fn draw_pattern(&mut self, pattern: &Pattern, center: &BoardPoint) {
        for pattern_point in pattern.get_points() {
            let board_point = pattern_point.offset(center.x, center.y);
            self.set_live_point(&board_point);
        }
    }

    fn set_live(&mut self, x: i64, y: i64) {
        self.set_liveness(x, y, true);
    }

    fn is_live_point(&self, point: &BoardPoint) -> bool {
        self.is_live(point.x, point.y)
    }

    fn set_live_point(&mut self, point: &BoardPoint) {
        self.set_liveness_point(point, true);
    }

    fn set_liveness_point(&mut self, point: &BoardPoint, liveness: bool) {
        self.set_liveness(point.x, point.y, liveness);
    }
}

pub struct BoardPoint {
    x: i64,
    y: i64,
}

impl BoardPoint {
    pub fn new(x: i64, y: i64) -> BoardPoint {
        BoardPoint { x, y }
    }

    pub fn offset(&self, x: i64, y: i64) -> BoardPoint {
        BoardPoint {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl From<&(i64, i64)> for BoardPoint {
    fn from((x, y): &(i64, i64)) -> Self {
        BoardPoint::new(x.clone(), y.clone())
    }
}
