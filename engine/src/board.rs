mod board {
    use crate::LifePoint;
    use std::fmt;

    #[derive(Clone, Debug)]
    pub struct LifeBoard {
        live_points: Vec<LifePoint>,
    }

    impl LifeBoard {
        pub fn new(state: Vec<LifePoint>) -> LifeBoard {
            LifeBoard { live_points: state }
        }

        pub fn step_one(&self) -> LifeBoard {
            LifeBoard {
                live_points: vec![],
            }
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
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        pub fn can_create_empty_board() {
            let board = LifeBoard::new(vec![]);
            assert_eq!(board.live_points.len(), 0)
        }

        #[test]
        pub fn can_create_filled_board() {
            let points = vec![LifePoint { x: 0, y: 0 }, LifePoint { x: 1, y: 2 }];
            let board = LifeBoard::new(points.clone());
            assert_eq!(board.live_points.len(), 2);

            let board_points = board.live_points;
            assert_eq!(points[0], board_points[0])
        }
    }
}
