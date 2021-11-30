use super::life_board::BoardPoint;

pub struct Pattern {
    points: Vec<BoardPoint>,
}

impl Pattern {
    pub fn new(points: Vec<BoardPoint>) -> Pattern {
        Pattern { points }
    }

    pub fn from_tuples(points: Vec<(i64, i64)>) -> Pattern {
        Pattern::new(points.iter().map(BoardPoint::from).collect())
    }

    pub fn get_points(&self) -> &Vec<BoardPoint> {
        &self.points
    }

    #[allow(non_snake_case)]
    pub fn BLOCK() -> Pattern {
        Pattern::from_tuples(vec![(0, 0), (0, 1), (1, 0), (1, 1)])
    }

    #[allow(non_snake_case)]
    pub fn BEACON() -> Pattern {
        Pattern::from_tuples(vec![
            (-2, -2),
            (-2, -1),
            (-1, -1),
            (-1, -2),
            (1, 1),
            (1, 0),
            (0, 0),
            (0, 1),
        ])
    }

    #[allow(non_snake_case)]
    pub fn ACORN() -> Pattern {
        Pattern::from_tuples(vec![
            (-2, -1),
            (0, 0),
            (-3, 1),
            (-2, 1),
            (1, 1),
            (2, 1),
            (3, 1),
        ])
    }

    #[allow(non_snake_case)]
    pub fn GLIDER_SOUTHEAST() -> Pattern {
        Pattern::from_tuples(vec![(0, -1), (1, 0), (-1, 1), (0, 1), (1, 1)])
    }

    #[allow(non_snake_case)]
    pub fn PULSAR() -> Pattern {
        Pattern::from_tuples(vec![
            (-4, -6),
            (-3, -6),
            (-2, -6),
            (2, -6),
            (3, -6),
            (4, -6),
            (-6, -2),
            (-1, -2),
            (1, -2),
            (6, -2),
            (-6, -3),
            (-1, -3),
            (1, -3),
            (6, -3),
            (-6, -4),
            (-1, -4),
            (1, -4),
            (6, -4),
            (-4, -1),
            (-3, -1),
            (-2, -1),
            (2, -1),
            (3, -1),
            (4, -1),
            (-4, 6),
            (-3, 6),
            (-2, 6),
            (2, 6),
            (3, 6),
            (4, 6),
            (-6, 2),
            (-1, 2),
            (1, 2),
            (6, 2),
            (-6, 3),
            (-1, 3),
            (1, 3),
            (6, 3),
            (-6, 4),
            (-1, 4),
            (1, 4),
            (6, 4),
            (-4, 1),
            (-3, 1),
            (-2, 1),
            (2, 1),
            (3, 1),
            (4, 1),
        ])
    }
}
