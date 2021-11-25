mod dynamic_vector_board;
mod fixed_vector_board;
mod life_board;

use dynamic_vector_board::DynamicVectorLifeBoard;
use fixed_vector_board::FixedVectorLifeBoard;
use life_board::LifeBoard;

pub fn new_fixed_vector_board() -> FixedVectorLifeBoard {
    FixedVectorLifeBoard::empty()
}

pub fn new_dynamic_vector_board() -> DynamicVectorLifeBoard {
    DynamicVectorLifeBoard::empty()
}
