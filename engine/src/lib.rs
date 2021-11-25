mod fixed_vector_board;
mod life_board;

use fixed_vector_board::FixedVectorLifeBoard;

pub fn new_vector_board() -> FixedVectorLifeBoard {
    FixedVectorLifeBoard::empty()
}
