mod life_board;
mod vector_board;

use life_board::LifeBoard;
use vector_board::VectorLifeBoard;

pub fn new_vector_board() -> VectorLifeBoard {
    VectorLifeBoard::empty()
}
