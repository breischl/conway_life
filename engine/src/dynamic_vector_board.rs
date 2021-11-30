use std::fmt::Display;
use std::cmp::min;
use std::cmp::max;
use super::life_board::LifeBoard;

pub struct DynamicVectorLifeBoard {
    grid: Vec<Vec<bool>>,

    /// The set of logical board squares that are currently allocated
    /// ie, what area of the board can be used without resizing `grid`
    board_extent: Rectangle,

    /// Rectangle containing all the live squares on the board
    /// In board coordinates
    live_extent: Rectangle,
}

struct Rectangle {
    x_min: i64,
    y_min: i64,
    width: i64,
    height: i64,
}

type BoardIndex = i64;
type GridIndex = usize;

impl Rectangle {
    fn contains_point(&self, x:BoardIndex, y:BoardIndex) -> bool{
        return x >= self.x_min 
        && x < (self.x_min + self.width)
         && y >= self.y_min 
         && y < (self.y_min + self.height);
    }

    fn expand_to_include(&mut self, x:BoardIndex, y:BoardIndex) {
        if self.is_empty() {
            self.x_min = x;
            self.y_min = y;
            self.width = 1;
            self.height = 1;
        } else {
            let x_max = max(self.x_min + self.width - 1, x);
            let y_max = max(self.y_min + self.height - 1, y);
            
            self.x_min = min(self.x_min, x);
            self.width = x_max - self.x_min + 1;
            self.y_min = min(self.y_min, y);
            self.height = y_max - self.y_min + 1;
        }
    }

    fn empty() -> Rectangle{
        Rectangle{
            x_min: 0,
            width: 0,
            y_min: 0,
            height: 0
        }
    }

    fn is_empty(&self) -> bool{
        self.width == 0 && self.height == 0
    }
}

impl Display for Rectangle{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "(minX:{}, minY:{}, width:{}, height:{})", self.x_min, self.y_min, self.width, self.height)
    }
}

impl DynamicVectorLifeBoard {
    fn is_live_num(&self, x: i64, y: i64) -> u8 {
        if self.is_live(x, y) {
            1
        } else {
            0
        }
    }

    fn convert_coordinates(&self, x: BoardIndex, y: BoardIndex) -> (GridIndex, GridIndex) {
        ((x - self.board_extent.x_min) as GridIndex, (y - self.board_extent.y_min) as GridIndex)
    }

    fn create_empty_grid(x_size: usize, y_size: usize) -> Vec<Vec<bool>>{
        let mut new_y_vec: Vec<bool> = Vec::with_capacity(y_size);
        new_y_vec.resize(y_size, false);

        let mut new_x_vec: Vec<Vec<bool>> = Vec::with_capacity(x_size);
        new_x_vec.resize(x_size, new_y_vec);
        new_x_vec
    }

    #[allow(dead_code)]
    fn get_live_count(&self) -> u64 {
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

    pub fn empty() -> DynamicVectorLifeBoard {
        DynamicVectorLifeBoard{
            grid : vec![],
            board_extent : Rectangle::empty(),
            live_extent: Rectangle::empty()
        }
    }

    fn is_live_unchecked(&self, xu: usize, yu: usize) -> u8 {
        self.grid
            .get(xu)
            .and_then(|row| row.get(yu))
            .map(|b| if *b { 1} else {0})
            .unwrap()
    }
}

impl LifeBoard for DynamicVectorLifeBoard{
    /// Count the live neighbors of this cell, not counting the cell itself
    fn count_live_neighbors(&self, x: i64, y: i64) -> u8 {
        // let (xu, yu) = self.convert_coordinates(x, y);
        // let is_on_edge = xu == 0 || yu == 0 || xu == (self.board_extent.width as usize - 1) || yu == (self.board_extent.height as usize - 1);

        // if is_on_edge {
            self.is_live_num(x - 1, y - 1) 
            + self.is_live_num(x - 1, y)
            + self.is_live_num(x - 1, y + 1)
            + self.is_live_num(x, y - 1)
            //Note we skipped counting at (x, y) here, because that's the cell itself
            + self.is_live_num(x, y + 1)
            + self.is_live_num(x + 1, y - 1)
            + self.is_live_num(x + 1, y)
            + self.is_live_num(x + 1, y + 1)
        // } else {
        //     self.is_live_unchecked(xu - 1, yu - 1) 
        //     + self.is_live_unchecked(xu - 1, yu)
        //     + self.is_live_unchecked(xu - 1, yu + 1)
        //     + self.is_live_unchecked(xu, yu - 1)
        //     //Note we skipped counting at (x, y) here, because that's the cell itself
        //     + self.is_live_unchecked(xu, yu + 1)
        //     + self.is_live_unchecked(xu + 1, yu - 1)
        //     + self.is_live_unchecked(xu + 1, yu)
        //     + self.is_live_unchecked(xu + 1, yu + 1)
        // }
    }

    fn set_liveness(&mut self, x: i64, y: i64, is_live:bool) {
        self.live_extent.expand_to_include(x, y);
        
         if !self.board_extent.contains_point(x, y) {
            //Note we're expanding the board extent a fair bit here, to hopefully avoid too many resizes when setting up the initial board conditions
            const PADDING : i64 = 10;
            let new_board_extent = Rectangle{
                x_min: self.live_extent.x_min - PADDING,
                width: self.live_extent.width + (2 * PADDING),
                y_min: self.live_extent.y_min - PADDING,
                height: self.live_extent.height + (2 * PADDING)
            };

            let mut new_grid = DynamicVectorLifeBoard::create_empty_grid(new_board_extent.width as usize, new_board_extent.height as usize);
            
            let le = &self.live_extent;

            //Copy old grid values to new grid
            for xi in le.x_min..(le.x_min + le.width) {
                let new_grid_x = (xi - new_board_extent.x_min) as usize;
                let column = new_grid.get_mut(new_grid_x).unwrap();

                for yi in le.y_min..(le.y_min + le.height) {
                    if self.is_live(xi, yi) {
                        let new_grid_y = (yi - new_board_extent.y_min) as usize;
                        column[new_grid_y] = true;
                    }
                    
                }
            }

            self.grid = new_grid;
            self.board_extent = new_board_extent;

            //Not necessary - live extent didn't change because it was in board coordinates already
            //self.live_extent = ?
        }

        let (xu, yu) = self.convert_coordinates(x, y);
        self.grid.get_mut(xu).unwrap()[yu] = is_live;
    }
    
    fn is_live(&self, x: i64, y: i64) -> bool {
        //We need to check that we don't index off the underlying vectors, which would really be the self.board_extent rectangle
        //But we may as well check against self.live_extent since it's always a smaller bound, and will let us skip the actual grid lookup in some cases
        if !self.live_extent.contains_point(x, y) {
            false
        } else {
            let (xu, yu) = self.convert_coordinates(x, y);
            self.grid
                .get(xu)
                .and_then(|row| row.get(yu))
                .map(|b| *b)
                .unwrap_or(false)
        }
    }

    fn step_one(&mut self) { 
        //We'll make the new board one larger than the existing live_extent in every direction
        //So we can't possibly grow off the sides
        let new_board_extent = Rectangle{
            x_min: self.live_extent.x_min - 1,
            width: self.live_extent.width + 2,
            y_min: self.live_extent.y_min - 1,
            height: self.live_extent.height + 2
        };

        let mut new_grid = DynamicVectorLifeBoard::create_empty_grid(new_board_extent.width as usize, new_board_extent.height as usize);

        let mut new_live_extent = Rectangle::empty();
        
        for xi in new_board_extent.x_min..(new_board_extent.x_min + new_board_extent.width) {
            let new_grid_x = (xi - new_board_extent.x_min) as usize;
            let column = new_grid.get_mut(new_grid_x).unwrap();

            for yi in new_board_extent.y_min..(new_board_extent.y_min + new_board_extent.height) {
                let count = self.count_live_neighbors(xi, yi);
                let live = count == 3 || (count == 2 && self.is_live(xi, yi));
                if live {
                    let new_grid_y = (yi - new_board_extent.y_min) as usize;
                    column[new_grid_y] = true;
                    new_live_extent.expand_to_include(xi, yi);
                }  
            }
        }

        self.grid = new_grid;
        self.live_extent = new_live_extent;
        self.board_extent = new_board_extent;
    }

    
    fn get_stats(&self) -> Vec<(&str, String)>{
        vec![
        ("implementation", "Dynamic vector".to_owned()),    
        ("live_cells", self.get_live_count().to_string()),
        ("board_extent", format!("{}", &self.board_extent)),
        ("live_extent", format!("{}", &self.live_extent)),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn step_one_works_empty(){
        let mut board = DynamicVectorLifeBoard::empty();
        board.step_one();
        assert_eq!(0, board.get_live_count());
    }

    #[test]
    pub fn step_one_works_blinker(){
        let mut board = DynamicVectorLifeBoard::empty();
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
        let board = DynamicVectorLifeBoard::empty();
        assert_eq!(0, board.get_live_count());
    }

    #[test]
    pub fn set_live_ensures_capacity() {
        let mut board = DynamicVectorLifeBoard::empty();
        board.set_live(0, 0);
        board.set_live(0, 1);

        assert_eq!(true, board.is_live(0, 0));
        assert_eq!(true, board.is_live(0, 1));
        assert_eq!(1, board.live_extent.width);
        assert_eq!(2, board.live_extent.height);
        assert_eq!(2, board.get_live_count());

        board.set_live(1, 2);
        assert_eq!(2, board.live_extent.width);
        assert_eq!(3, board.live_extent.height);
        assert_eq!(true, board.is_live(1, 2));
        assert_eq!(3, board.get_live_count());

        board.set_live(4, 5);
        assert_eq!(5, board.live_extent.width);
        assert_eq!(6, board.live_extent.height);
        assert_eq!(true, board.is_live(4, 5));
        assert_eq!(4, board.get_live_count());
    }

    #[test]
    pub fn set_live_ensures_capacity_in_negative_coordinates() {
        let mut board = DynamicVectorLifeBoard::empty();
        assert_eq!(false, board.is_live(0, 0));
        assert_eq!(0, board.live_extent.width);
        assert_eq!(0, board.live_extent.height);
        
        board.set_live(10, 10);
        assert_eq!(true, board.is_live(10, 10));
        assert_eq!(1, board.live_extent.width);
        assert_eq!(1, board.live_extent.height);

        board.set_live(5, 5);
        assert_eq!(true, board.is_live(5, 5));
        assert_eq!(true, board.is_live(10, 10));
        assert_eq!(6, board.live_extent.width);
        assert_eq!(6, board.live_extent.height);
    }

    #[test]
    pub fn live_extent_works_off_origin() {
        let mut board = DynamicVectorLifeBoard::empty();
        board.set_live(90, 100);
        assert_eq!(1, board.live_extent.width);
        assert_eq!(1, board.live_extent.height);
        assert_eq!(90, board.live_extent.x_min);
        assert_eq!(100, board.live_extent.y_min);
    }

    #[test]
    pub fn count_live_neighbors_works_at_borders() {
        let mut board = DynamicVectorLifeBoard::empty();
        board.set_live(0, 0);
        board.set_live(0, 1);

        let neighbors = board.count_live_neighbors(0, 0);
        assert_eq!(neighbors, 1);
    }

    #[test]
    pub fn count_live_neighbors_doesnt_count_self() {
        let mut board = DynamicVectorLifeBoard::empty();
        for xi in 0..3{
            for yi in 0..3{
                board.set_live(xi, yi);
            }
        }

        assert_eq!(9, board.get_live_count());

        let neighbors = board.count_live_neighbors(1, 1);
        assert_eq!(neighbors, 8);
    }
}
