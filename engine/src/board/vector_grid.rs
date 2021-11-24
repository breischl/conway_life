use std::cmp::max;
use std::convert::From;
#[derive(Clone, Debug)]
pub struct VectorGrid {
    grid: Vec<Vec<bool>>,
    x_size: usize,
    y_size: usize,
}

impl VectorGrid {
    pub fn empty() -> VectorGrid {
        VectorGrid::from(vec![])
    }
    pub fn is_live(&self, x: usize, y: usize) -> bool {
        self.grid
            .get(x)
            .and_then(|row| row.get(y))
            .map(|b| b.clone())
            .unwrap_or(false)
    }

    pub fn set_live(&mut self, x: usize, y: usize) {
        self.ensure_size(x + 1, y + 1);
        self.grid.get_mut(x).unwrap()[y] = true;
    }

    fn ensure_size(&mut self, req_x_size: usize, req_y_size: usize) {
        let new_x_size = max(self.x_size, req_x_size);
        let new_y_size = max(self.y_size, req_y_size);

        //If y-size is greater, resize all the existing y vectors
        if new_y_size > self.y_size {
            for y_vec in self.grid.iter_mut() {
                y_vec.resize(new_y_size, false);
            }
        }

        //If the x-size is greater, resize the main vector, being sure the newly-added rows use the new y-max
        if new_x_size > self.x_size {
            let mut new_vec: Vec<bool> = Vec::with_capacity(new_y_size);
            new_vec.resize(new_y_size, false);

            self.grid.resize(new_x_size, new_vec);
        }

        self.x_size = new_x_size;
        self.y_size = new_y_size;
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<Vec<bool>> {
        vec![
            vec![
                self.is_live(x - 1, y - 1),
                self.is_live(x, y - 1),
                self.is_live(x + 1, y - 1),
            ],
            vec![
                self.is_live(x - 1, y),
                self.is_live(x, y),
                self.is_live(x + 1, y),
            ],
            vec![
                self.is_live(x - 1, y + 1),
                self.is_live(x, y + 1),
                self.is_live(x + 1, y + 1),
            ],
        ]
    }

    pub fn get_live_count(&self) -> u64 {
        let mut count: u64 = 0;
        for row in &self.grid {
            for cell in row {
                if *cell {
                    count = count + 1;
                }
            }
        }
        count
    }
}

/// Create a new `VectorGrid` from the given set of booleans. Each live cell should be indicated with a `true`, dead cells with a `false`.
/// The board implicitly starts at the origin, ie cell `(0, 0)`.
/// All of the vectors must be the same length and capacity.
impl From<Vec<Vec<bool>>> for VectorGrid {
    fn from(grid: Vec<Vec<bool>>) -> Self {
        let x_size = grid.capacity();
        let y_size = grid.get(0).map(|v| v.capacity()).unwrap_or(0);

        if grid.len() > 1 {
            let y_used = grid.get(0).unwrap().len();
            for vec in grid.iter() {
                if vec.len() != y_used {
                    panic!("All vectors in VectorGrid must be the same length");
                }
                if vec.capacity() != y_size {
                    panic!("All vectors in VectorGrid must have the same capacity");
                }
            }
        }

        VectorGrid {
            grid,
            x_size,
            y_size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn can_create_empty_board() {
        let board = VectorGrid::from(vec![]);
        assert_eq!(0, board.get_live_count());
        assert_eq!(0, board.x_size);
        assert_eq!(0, board.y_size);
        assert_eq!(false, board.is_live(0, 0));
        assert_eq!(false, board.is_live(0, 1));
        assert_eq!(false, board.is_live(1, 1));
    }

    #[test]
    #[should_panic]
    pub fn cant_create_jagged_length_board() {
        let points = vec![vec![false, true], vec![true, false, true]];
        let board = VectorGrid::from(points);
        assert_eq!(3, board.get_live_count());
    }

    #[test]
    #[should_panic]
    pub fn cant_create_jagged_capacity_board() {
        let vec1 = vec![false, true];
        let mut vec2: Vec<bool> = Vec::with_capacity(50);
        vec2.push(false);
        vec2.push(true);

        assert_eq!(vec1.len(), vec2.len());
        assert_ne!(vec1.capacity(), vec2.capacity());

        VectorGrid::from(vec![vec1, vec2]);
    }

    #[test]
    pub fn can_create_filled_board() {
        let points = vec![vec![false, true, false], vec![true, false, true]];
        let board = VectorGrid::from(points);
        assert_eq!(2, board.x_size);
        assert_eq!(3, board.y_size);
        assert_eq!(3, board.get_live_count());
        assert_eq!(false, board.is_live(0, 0));
        assert_eq!(true, board.is_live(0, 1));
        assert_eq!(false, board.is_live(0, 2));
        assert_eq!(true, board.is_live(1, 0));
        assert_eq!(false, board.is_live(1, 1));
        assert_eq!(true, board.is_live(1, 2));
    }

    #[test]
    pub fn set_live_ensures_capacity() {
        let mut board = VectorGrid::from(vec![vec![true, true]]);
        assert_eq!(true, board.is_live(0, 0));
        assert_eq!(true, board.is_live(0, 1));
        assert_eq!(1, board.x_size);
        assert_eq!(2, board.y_size);
        assert_eq!(2, board.get_live_count());

        board.set_live(1, 2);
        assert_eq!(2, board.x_size);
        assert_eq!(3, board.y_size);
        assert_eq!(true, board.is_live(1, 2));
        assert_eq!(3, board.get_live_count());

        board.set_live(4, 5);
        assert_eq!(5, board.x_size);
        assert_eq!(6, board.y_size);
        assert_eq!(true, board.is_live(4, 5));
        assert_eq!(4, board.get_live_count());
    }

    #[test]
    pub fn get_neighbors_extends_with_false() {
        let mut board = VectorGrid::empty();
        board.set_live(0, 0);
        board.set_live(0, 1);

        let neighbors = board.get_neighbors(0, 0);
        assert_eq!(
            neighbors,
            vec![
                vec![false, false, false],
                vec![false, true, true],
                vec![false, false, false]
            ]
        )
    }
}
