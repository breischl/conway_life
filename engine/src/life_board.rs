pub trait LifeBoard {
    fn is_live(&self, x: i64, y: i64) -> bool;
    fn set_live(&mut self, x: i64, y: i64);
    fn count_live_neighbors(&self, x: i64, y: i64) -> u8;
    fn step_one(&mut self);

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
