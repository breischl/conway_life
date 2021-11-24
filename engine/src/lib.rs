mod board;
mod grid;

use board::LifeBoard;
use grid::life_grid::LifeGrid;
use grid::vector_grid::VectorGrid;

pub fn new_vector_board() -> LifeBoard {
    let grid: Box<dyn LifeGrid> = Box::new(VectorGrid::empty());
    LifeBoard::new(grid)
}
