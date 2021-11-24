use crate::grid::life_grid::LifeGrid;

pub struct LifeBoard {
    grid: Box<dyn LifeGrid>,
}

impl LifeBoard {
    pub fn new(grid: Box<dyn LifeGrid>) -> LifeBoard {
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
    use crate::grid::vector_grid::VectorGrid;

    #[test]
    pub fn can_create_empty_board() {
        let grid: Box<dyn LifeGrid> = Box::new(VectorGrid::empty());
        LifeBoard::new(grid);
    }

    #[test]
    pub fn can_create_filled_board() {
        let mut grid: Box<dyn LifeGrid> = Box::new(VectorGrid::empty());
        grid.set_live(0, 0);
        grid.set_live(1, 1);
        LifeBoard::new(grid);
    }
}
