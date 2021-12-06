mod array_grid;
mod dynamic_array2d_board;
mod dynamic_vector_board;
mod fixed_bitfield_board;
mod fixed_vector_board;
pub mod life_board;
mod life_cell;
pub mod pattern;

pub use dynamic_array2d_board::DynamicArray2dLifeBoard;
pub use dynamic_vector_board::DynamicVectorLifeBoard;
pub use fixed_bitfield_board::FixedBitfieldLifeBoard;
pub use fixed_vector_board::FixedVectorLifeBoard;
pub use life_board::LifeBoard;

pub fn new_fixed_vector_board() -> FixedVectorLifeBoard {
    FixedVectorLifeBoard::empty()
}

pub fn new_dynamic_vector_board() -> DynamicVectorLifeBoard {
    DynamicVectorLifeBoard::empty()
}

pub fn new_dynamic_array2d_board() -> DynamicArray2dLifeBoard {
    DynamicArray2dLifeBoard::empty()
}

pub fn new_fixed_bitfield_board() -> FixedBitfieldLifeBoard {
    FixedBitfieldLifeBoard::empty()
}
