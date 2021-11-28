mod dynamic_vector_board;
mod fixed_vector_board;
pub mod life_board;

pub use dynamic_vector_board::DynamicVectorLifeBoard;
pub use fixed_vector_board::FixedVectorLifeBoard;
pub use life_board::LifeBoard;

pub fn new_fixed_vector_board() -> FixedVectorLifeBoard {
    FixedVectorLifeBoard::empty()
}

pub fn new_dynamic_vector_board() -> DynamicVectorLifeBoard {
    DynamicVectorLifeBoard::empty()
}
