#[derive(Clone, Default)]
pub struct LifeCell {
    data: u8,
}

impl LifeCell {
    const STATE_MASK: u8 = 0b00010000;
    const COUNT_MASK: u8 = 0b00001111;

    pub fn is_live(&self) -> bool {
        self.data & LifeCell::STATE_MASK != 0
    }

    pub fn neighbors_count(&self) -> u8 {
        self.data & LifeCell::COUNT_MASK
    }

    pub fn create_dead() -> LifeCell {
        LifeCell { data: 0 }
    }

    /// Indicates if this cell and all the neighboring cells are dead
    pub fn is_deadzone(&self) -> bool {
        self.data == 0
    }

    pub fn set_alive(&mut self) {
        self.data = self.data | LifeCell::STATE_MASK;
    }

    pub fn set_dead(&mut self) {
        self.data = self.data & !LifeCell::STATE_MASK;
    }

    pub fn increment_neighbors(&mut self) {
        self.data = self.data + 1;
    }

    pub fn decrement_neighbors(&mut self) {
        self.data = self.data - 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn life_cell_works() {
        let mut lc = LifeCell::create_dead();
        assert_eq!(false, lc.is_live());
        assert_eq!(0, lc.neighbors_count());

        lc.set_alive();
        assert_eq!(true, lc.is_live());
        assert_eq!(0, lc.neighbors_count());

        lc.increment_neighbors();
        assert_eq!(true, lc.is_live());
        assert_eq!(1, lc.neighbors_count());

        lc.increment_neighbors();
        assert_eq!(true, lc.is_live());
        assert_eq!(2, lc.neighbors_count());

        lc.set_dead();
        assert_eq!(false, lc.is_live());
        assert_eq!(2, lc.neighbors_count());

        lc.decrement_neighbors();
        assert_eq!(false, lc.is_live());
        assert_eq!(1, lc.neighbors_count());

        lc.decrement_neighbors();
        assert_eq!(false, lc.is_live());
        assert_eq!(0, lc.neighbors_count());
    }
}
