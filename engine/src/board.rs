mod vector_grid;

mod board {
    use crate::board::vector_grid::VectorGrid;
    use std::convert::From;

    #[derive(Clone, Debug)]
    pub struct LifeBoard {
        grid: VectorGrid,
    }

    impl LifeBoard {
        pub fn new(grid: VectorGrid) -> LifeBoard {
            LifeBoard { grid }
        }

        // pub fn step_one(&self) -> LifeBoard {
        //     LifeBoard { grid: vec![] }
        // }

        // pub fn step_many(&self, num_steps: u64) -> LifeBoard {
        //     if num_steps == 0 {
        //         //We didn't change anything, return the same state
        //         self.clone()
        //     } else {
        //         let mut board = self.step_one();
        //         for _step_num in 0..(num_steps - 1) {
        //             board = board.step_one();
        //         }
        //         board
        //     }
        // }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        pub fn can_create_empty_board() {
            let grid = VectorGrid::from(vec![]);
            let board = LifeBoard::new(grid);
        }

        #[test]
        pub fn can_create_filled_board() {
            let points = vec![vec![false, true], vec![true, false]];
            let grid = VectorGrid::from(points);
            LifeBoard::new(grid);
        }
    }
}
