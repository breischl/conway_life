pub trait LifeGrid {
    fn is_live(&self, x: i64, y: i64) -> bool;
    fn set_live(&mut self, x: i64, y: i64);
    fn count_live_neighbors(&self, x: i64, y: i64) -> u8;
}
