use std::cmp::max;
use std::convert::From;
use super::life_board::LifeBoard;

pub struct DynamicVectorLifeBoard {
    grid: Vec<Vec<bool>>,
    x_size: usize,
    y_size: usize,
}

impl DynamicVectorLifeBoard {
    pub fn empty() -> DynamicVectorLifeBoard {
        DynamicVectorLifeBoard::from(vec![])
    }

    fn is_live_num(&self, x: i64, y: i64) -> u8 {
        if self.is_live(x, y) {
            1
        } else {
            0
        }
    }

    fn convert_coordinates(&self, x: i64, y: i64) -> (usize, usize) {
        (x as usize, y as usize)
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

    #[allow(dead_code)]
    fn get_live_count(&self) -> u64 {
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

impl LifeBoard for DynamicVectorLifeBoard{
    fn empty() -> DynamicVectorLifeBoard {
        DynamicVectorLifeBoard::from(vec![])
    }
    
    /// Count the live neighbors of this cell, not counting the cell itself
    fn count_live_neighbors(&self, x: i64, y: i64) -> u8 {
        self.is_live_num(x - 1, y - 1) 
        + self.is_live_num(x - 1, y)
        + self.is_live_num(x - 1, y + 1)
        + self.is_live_num(x, y - 1)
        //Note we skipped counting at (x, y) here, because that's the cell itself
        + self.is_live_num(x, y + 1)
        + self.is_live_num(x + 1, y - 1)
        + self.is_live_num(x + 1, y)
        + self.is_live_num(x + 1, y + 1)
    }

    fn set_live(&mut self, x: i64, y: i64) {
        let (xu, yu) = self.convert_coordinates(x, y);
        self.ensure_size(xu + 1, yu + 1);
        self.grid.get_mut(xu).unwrap()[yu] = true;
    }
    
    fn is_live(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            false
        } else {
            let (xu, yu) = self.convert_coordinates(x, y);
            self.grid
                .get(xu)
                .and_then(|row| row.get(yu))
                .map(|b| b.clone())
                .unwrap_or(false)
        }
    }

    fn step_one(&mut self) { 
        //Duplicate the internal vectors so that we don't lose the prior state halfway through
        let mut new_state = self.grid.clone();

        for xi in 0..(self.x_size as i64) {
            for yi in 0..(self.y_size as i64) {
                let count = self.count_live_neighbors(xi, yi);
                let live = count == 3 || (count == 2 && self.is_live(xi, yi));
                
                let (xu, yu) = self.convert_coordinates(xi, yi);
                new_state.get_mut(xu).unwrap()[yu] = live;
            }
        }

        self.grid = new_state;
    }
}

/// Create a new `VectorGrid` from the given set of booleans. Each live cell should be indicated with a `true`, dead cells with a `false`.
/// The board implicitly starts at the origin, ie cell `(0, 0)`.
/// All of the vectors must be the same length and capacity.
impl From<Vec<Vec<bool>>> for DynamicVectorLifeBoard {
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

        DynamicVectorLifeBoard {
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
    pub fn step_one_works_empty(){
        let mut board = DynamicVectorLifeBoard::empty();
        board.step_one();
        assert_eq!(0, board.get_live_count());
    }

    #[test]
    pub fn step_one_works_blinker(){
        let mut board = DynamicVectorLifeBoard::empty();
        board.set_live(2, 2);
        board.set_live(2, 3);
        board.set_live(2, 4);
        board.set_live(5, 5);
        
        board.step_one();
        assert_eq!(3, board.get_live_count());
        assert_eq!(true, board.is_live(1, 3));
        assert_eq!(true, board.is_live(2, 3));
        assert_eq!(true, board.is_live(3, 3));
        assert_eq!(false, board.is_live(2, 2));
        assert_eq!(false, board.is_live(2, 4));
        assert_eq!(false, board.is_live(5, 5));

        board.step_one();
        assert_eq!(3, board.get_live_count());
        assert_eq!(true, board.is_live(2, 2));
        assert_eq!(true, board.is_live(2, 3));
        assert_eq!(true, board.is_live(2, 4));
        assert_eq!(false, board.is_live(1, 3));
        assert_eq!(false, board.is_live(3, 3));
        assert_eq!(false, board.is_live(5, 5));
    }


    #[test]
    pub fn can_create_empty_board() {
        let board = DynamicVectorLifeBoard::from(vec![]);
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
        let board = DynamicVectorLifeBoard::from(points);
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

        DynamicVectorLifeBoard::from(vec![vec1, vec2]);
    }

    #[test]
    pub fn can_create_filled_board() {
        let points = vec![vec![false, true, false], vec![true, false, true]];
        let board = DynamicVectorLifeBoard::from(points);
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
        let mut board = DynamicVectorLifeBoard::from(vec![vec![true, true]]);
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
    pub fn count_live_neighbors_works_at_borders() {
        let mut board = DynamicVectorLifeBoard::empty();
        board.set_live(0, 0);
        board.set_live(0, 1);

        let neighbors = board.count_live_neighbors(0, 0);
        assert_eq!(neighbors, 1);
    }

    #[test]
    pub fn count_live_neighbors_doesnt_count_self() {
        let mut board = DynamicVectorLifeBoard::empty();
        for xi in 0..3{
            for yi in 0..3{
                board.set_live(xi, yi);
            }
        }

        assert_eq!(9, board.get_live_count());

        let neighbors = board.count_live_neighbors(1, 1);
        assert_eq!(neighbors, 8);
    }
}
