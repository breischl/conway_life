mod board {
    use crate::engine::LifePoint;
    use std::fmt;

    #[derive(Clone, Debug)]
    pub struct LifeBoard {
        live_points: Vec<LifePoint>,
    }
    impl LifeBoard {
        pub fn new(state: &Vec<LifePoint>) -> LifeBoard {
            LifeBoard {
                live_points: vec![],
            }
        }

        pub fn step_one(&self) -> LifeBoard {
            LifeBoard {
                live_points: vec![],
            }
        }

        pub fn step_many(&self, numSteps: u32) -> LifeBoard {
            if numSteps == 0 {
                //We didn't change anything, return the same state
                self.clone()
            } else {
                let mut board = self.step_one();
                for _stepNum in 0..(numSteps - 1) {
                    board = board.step_one();
                }
                board
            }
        }
    }
}
