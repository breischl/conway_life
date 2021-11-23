mod board {
    use crate::LifePoint;
    use std::convert::From;
    use std::fmt;

    #[derive(Clone, Debug)]
    pub struct LifeBoard {
        grid: Vec<Vec<bool>>,
    }

    impl LifeBoard {
        pub fn step_one(&self) -> LifeBoard {
            LifeBoard { grid: vec![] }
        }

        pub fn step_many(&self, num_steps: u64) -> LifeBoard {
            if num_steps == 0 {
                //We didn't change anything, return the same state
                self.clone()
            } else {
                let mut board = self.step_one();
                for _step_num in 0..(num_steps - 1) {
                    board = board.step_one();
                }
                board
            }
        }

        pub fn is_live(&self, x: usize, y: usize) -> bool {
            self.grid
                .get(x)
                .and_then(|row| row.get(y))
                .map(|b| b.clone())
                .unwrap_or(false)
        }

        pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<Vec<bool>> {
            vec![
                vec![
                    self.is_live(x - 1, y - 1),
                    self.is_live(x, y - 1),
                    self.is_live(x + 1, y - 1),
                ],
                vec![
                    self.is_live(x - 1, y),
                    self.is_live(x, y),
                    self.is_live(x + 1, y),
                ],
                vec![
                    self.is_live(x - 1, y + 1),
                    self.is_live(x, y + 1),
                    self.is_live(x + 1, y + 1),
                ],
            ]
        }

        pub fn get_live_count(&self) -> u64 {
            let mut count: u64 = 0;
            for row in &self.grid {
                for cell in row {
                    if *cell {
                        count = count + 1;
                    }
                }
            }
            count
        }
    }

    impl From<Vec<Vec<bool>>> for LifeBoard {
        fn from(points: Vec<Vec<bool>>) -> Self {
            LifeBoard { grid: points }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        pub fn can_create_empty_board() {
            let board = LifeBoard::from(vec![]);
            assert_eq!(0, board.get_live_count());
            assert_eq!(false, board.is_live(0, 0));
            assert_eq!(false, board.is_live(0, 1));
            assert_eq!(false, board.is_live(1, 1));
        }

        #[test]
        pub fn can_create_filled_board() {
            let points = vec![vec![false, true], vec![true, false]];
            let board = LifeBoard::from(points.clone());
            assert_eq!(2, board.get_live_count());
            assert_eq!(false, board.is_live(0, 0));
            assert_eq!(true, board.is_live(0, 1));
            assert_eq!(true, board.is_live(1, 0));
            assert_eq!(false, board.is_live(1, 1));
        }
    }
}
