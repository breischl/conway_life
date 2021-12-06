use crate::array_grid::ArrayGrid;
use crate::life_board::LifeBoard;
use crate::life_cell::LifeCell;

const GRID_SIZE: usize = 258;
const BOARD_SIZE: i64 = GRID_SIZE as i64 - 2;

/// A fixed-size `LifeBoard` where each cell is represented by a bitfield.
pub struct FixedBitfieldLifeBoard {
    grid: ArrayGrid<LifeCell>,
}

impl FixedBitfieldLifeBoard {
    pub fn empty() -> FixedBitfieldLifeBoard {
        let grid: ArrayGrid<LifeCell> = ArrayGrid::create(GRID_SIZE, GRID_SIZE);
        FixedBitfieldLifeBoard { grid: grid }
    }

    fn convert_coordinates(&self, x: i64, y: i64) -> (usize, usize) {
        ((x + 1) as usize, (y + 1) as usize)
    }

    fn get_live_count(&self) -> u64 {
        self.grid
            .iter()
            .map(|lc| if lc.is_live() { 1 } else { 0 })
            .sum()
    }
}

impl LifeBoard for FixedBitfieldLifeBoard {
    /// Count the live neighbors of this cell, not counting the cell itself
    fn count_live_neighbors(&self, x: i64, y: i64) -> u8 {
        let (xu, yu) = self.convert_coordinates(x, y);
        self.grid.get(xu, yu).neighbors_count()
    }

    fn set_liveness(&mut self, x: i64, y: i64, is_live: bool) {
        let (xu, yu) = self.convert_coordinates(x, y);
        if is_live {
            self.grid.get_mut(xu - 1, yu - 1).increment_neighbors();
            self.grid.get_mut(xu - 1, yu).increment_neighbors();
            self.grid.get_mut(xu - 1, yu + 1).increment_neighbors();
            self.grid.get_mut(xu, yu - 1).increment_neighbors();
            self.grid.get_mut(xu, yu).set_alive();
            self.grid.get_mut(xu, yu + 1).increment_neighbors();
            self.grid.get_mut(xu + 1, yu - 1).increment_neighbors();
            self.grid.get_mut(xu + 1, yu).increment_neighbors();
            self.grid.get_mut(xu + 1, yu + 1).increment_neighbors();
        } else {
            self.grid.get_mut(xu - 1, yu - 1).decrement_neighbors();
            self.grid.get_mut(xu - 1, yu).decrement_neighbors();
            self.grid.get_mut(xu - 1, yu + 1).decrement_neighbors();
            self.grid.get_mut(xu, yu - 1).decrement_neighbors();
            self.grid.get_mut(xu, yu).set_dead();
            self.grid.get_mut(xu, yu + 1).decrement_neighbors();
            self.grid.get_mut(xu + 1, yu - 1).decrement_neighbors();
            self.grid.get_mut(xu + 1, yu).decrement_neighbors();
            self.grid.get_mut(xu + 1, yu + 1).decrement_neighbors();
        };
    }

    fn is_live(&self, x: i64, y: i64) -> bool {
        let (xu, yu) = self.convert_coordinates(x, y);
        self.grid.get(xu, yu).is_live()
    }

    fn step_one(&mut self) {
        //Duplicate the internal vectors so that we don't lose the prior state halfway through
        let old_state = self.grid.clone();

        for xi in 0..BOARD_SIZE {
            for yi in 0..BOARD_SIZE {
                let (xu, yu) = self.convert_coordinates(xi, yi);
                let old_cell = old_state.get(xu, yu);
                if old_cell.is_deadzone() {
                    continue;
                }

                let count = old_cell.neighbors_count();
                if old_cell.is_live() {
                    if count != 2 && count != 3 {
                        self.set_liveness(xi, yi, false);
                    }
                } else if count == 3 {
                    self.set_liveness(xi, yi, true);
                }
            }
        }
    }

    fn get_stats(&self) -> Vec<(&str, String)> {
        vec![
            ("implementation", "Fixed vector".to_owned()),
            ("live_cells", self.get_live_count().to_string()),
            ("grid_size", GRID_SIZE.to_string()),
            ("board_size", (GRID_SIZE - 2).to_string()),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn step_one_works_empty() {
        let mut board = FixedBitfieldLifeBoard::empty();
        board.step_one();
        assert_eq!(0, board.get_live_count());
    }

    #[test]
    pub fn step_one_works_blinker() {
        let mut board = FixedBitfieldLifeBoard::empty();
        board.set_live(2, 2);
        board.set_live(2, 3);
        board.set_live(2, 4);
        board.set_live(5, 5);
        board.step_one();
        assert_eq!(3, board.get_live_count());
        assert_eq!(true, board.is_live(1, 3));
        assert_eq!(true, board.is_live(2, 3));
        assert_eq!(true, board.is_live(3, 3));
        assert_eq!(false, board.is_live(2, 2));
        assert_eq!(false, board.is_live(2, 4));
        assert_eq!(false, board.is_live(5, 5));

        board.step_one();
        assert_eq!(3, board.get_live_count());
        assert_eq!(true, board.is_live(2, 2));
        assert_eq!(true, board.is_live(2, 3));
        assert_eq!(true, board.is_live(2, 4));
        assert_eq!(false, board.is_live(1, 3));
        assert_eq!(false, board.is_live(3, 3));
        assert_eq!(false, board.is_live(5, 5));
    }

    #[test]
    pub fn can_create_empty_board() {
        let board = FixedBitfieldLifeBoard::empty();
        assert_eq!(0, board.get_live_count());
        assert_eq!(false, board.is_live(0, 0));
        assert_eq!(false, board.is_live(0, 1));
        assert_eq!(false, board.is_live(1, 1));
    }

    #[test]
    pub fn count_live_neighbors_works_at_borders() {
        let mut board = FixedBitfieldLifeBoard::empty();
        board.set_live(0, 0);
        board.set_live(0, 1);

        let neighbors = board.count_live_neighbors(0, 0);
        assert_eq!(neighbors, 1);
    }

    #[test]
    pub fn count_live_neighbors_doesnt_count_self() {
        let mut board = FixedBitfieldLifeBoard::empty();
        for xi in 0..3 {
            for yi in 0..3 {
                board.set_live(xi, yi);
            }
        }

        assert_eq!(9, board.get_live_count());

        let neighbors = board.count_live_neighbors(1, 1);
        assert_eq!(neighbors, 8);
    }
}
