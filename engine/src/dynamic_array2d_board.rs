use super::life_board::LifeBoard;
use std::cmp::max;
use std::cmp::min;
use std::fmt::Display;
use std::ops::Range;

pub struct DynamicArray2dLifeBoard {
    grid: ArrayGrid,
    /// The set of logical board squares that are currently allocated
    /// ie, what area of the board can be used without resizing `grid`
    /// The physical `grid` will be the same size, but is 0-indexed
    board_extent: Rectangle,

    /// Rectangle containing all the live squares on the board
    /// In board coordinates
    live_extent: Rectangle,
}

impl DynamicArray2dLifeBoard {
    fn is_live_num(&self, x: i64, y: i64) -> u8 {
        if self.is_live(x, y) {
            1
        } else {
            0
        }
    }

    fn get_live_count(&self) -> u64 {
        //From inspection of Array2d source, row major order appears to be the "natural" iteration order
        //https://github.com/HarrisonMc555/array2d/blob/master/src/lib.rs#L814-L816
        self.grid.iter().map(|b| *b as u64).sum()
    }

    pub fn empty() -> DynamicArray2dLifeBoard {
        DynamicArray2dLifeBoard {
            grid: ArrayGrid::empty(),
            board_extent: Rectangle::empty(),
            live_extent: Rectangle::empty(),
        }
    }

    fn is_live_unchecked(&self, xu: usize, yu: usize) -> u8 {
        self.grid.get(xu, yu)
    }
}

impl LifeBoard for DynamicArray2dLifeBoard {
    /// Count the live neighbors of this cell, not counting the cell itself
    fn count_live_neighbors(&self, x: BoardIndex, y: BoardIndex) -> u8 {
        let (xu, yu) = self.board_extent.to_grid_point(x, y);
        let is_on_edge = xu <= 0
            || yu <= 0
            || xu >= (self.board_extent.width as usize - 1)
            || yu >= (self.board_extent.height as usize - 1);

        if is_on_edge {
            self.is_live_num(x - 1, y - 1)
                + self.is_live_num(x - 1, y)
                + self.is_live_num(x - 1, y + 1)
                + self.is_live_num(x, y - 1)
                + self.is_live_num(x, y + 1)
                + self.is_live_num(x + 1, y - 1)
                + self.is_live_num(x + 1, y)
                + self.is_live_num(x + 1, y + 1)
        } else {
            self.is_live_unchecked(xu - 1, yu - 1)
                + self.is_live_unchecked(xu - 1, yu)
                + self.is_live_unchecked(xu - 1, yu + 1)
                + self.is_live_unchecked(xu, yu - 1)
                + self.is_live_unchecked(xu, yu + 1)
                + self.is_live_unchecked(xu + 1, yu - 1)
                + self.is_live_unchecked(xu + 1, yu)
                + self.is_live_unchecked(xu + 1, yu + 1)
        }
    }

    fn set_liveness(&mut self, x: BoardIndex, y: BoardIndex, is_live: bool) {
        //Check if we have space in the current grid, and if not expand it
        if !self.board_extent.contains_point(x, y) {
            let mut new_board_extent = self.board_extent.clone();
            new_board_extent.expand_to_include(x, y);

            let mut new_grid = ArrayGrid::create(
                new_board_extent.width as usize,
                new_board_extent.height as usize,
            );
            //Copy old grid values to new grid
            for xi in self.live_extent.x_range() {
                let xu = new_board_extent.to_grid_x(xi);

                for yi in self.live_extent.y_range() {
                    if self.is_live(xi, yi) {
                        new_grid.set(xu, new_board_extent.to_grid_y(yi), 1);
                    }
                }
            }

            self.grid = new_grid;
            self.board_extent = new_board_extent;
        }

        self.live_extent.expand_to_include(x, y);
        let (xu, yu) = self.board_extent.to_grid_point(x, y);
        let live: u8 = if is_live { 1 } else { 0 };
        self.grid.set(xu, yu, live);
    }
    fn is_live(&self, x: BoardIndex, y: BoardIndex) -> bool {
        self.live_extent.contains_point(x, y)
            && self.is_live_unchecked(
                self.board_extent.to_grid_x(x),
                self.board_extent.to_grid_y(y),
            ) > 0
    }

    fn step_one(&mut self) {
        if self.board_extent.is_empty() {
            return;
        }
        //We'll make the new board one larger than the existing live_extent in every direction so we can't possibly grow off the sides
        //This does not grow unbounded because we're basing off live_extent, not board_extent
        let new_board_extent = Rectangle {
            x_min: self.live_extent.x_min - 1,
            width: self.live_extent.width + 2,
            y_min: self.live_extent.y_min - 1,
            height: self.live_extent.height + 2,
        };

        let mut new_grid = ArrayGrid::create(
            new_board_extent.width as usize,
            new_board_extent.height as usize,
        );

        let mut new_live_extent = Rectangle::empty();
        for xi in new_board_extent.x_range() {
            let xu = new_board_extent.to_grid_x(xi);
            for yi in new_board_extent.y_range() {
                let count = self.count_live_neighbors(xi, yi);
                let live = count == 3 || (count == 2 && self.is_live(xi, yi));
                if live {
                    new_grid.set(xu, new_board_extent.to_grid_y(yi), 1);
                    new_live_extent.expand_to_include(xi, yi);
                }
            }
        }

        self.grid = new_grid;
        self.live_extent = new_live_extent;
        self.board_extent = new_board_extent;
    }

    fn get_stats(&self) -> Vec<(&str, String)> {
        vec![
            ("implementation", "Dynamic Array2d".to_owned()),
            ("live_cells", self.get_live_count().to_string()),
            ("board_extent", format!("{}", &self.board_extent)),
            ("live_extent", format!("{}", &self.live_extent)),
        ]
    }
}

type BoardIndex = i64;
type GridIndex = usize;

#[derive(Copy, Clone, Debug)]
struct Rectangle {
    x_min: BoardIndex,
    y_min: BoardIndex,
    width: BoardIndex,
    height: BoardIndex,
}

impl Rectangle {
    fn contains_point(&self, x: BoardIndex, y: BoardIndex) -> bool {
        return x >= self.x_min && x <= self.x_max() && y >= self.y_min && y <= self.y_max();
    }

    fn expand_to_include(&mut self, x: BoardIndex, y: BoardIndex) {
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

    fn empty() -> Rectangle {
        Rectangle {
            x_min: 0,
            width: 0,
            y_min: 0,
            height: 0,
        }
    }

    fn x_max(&self) -> BoardIndex {
        self.x_min + self.width - 1
    }

    fn y_max(&self) -> BoardIndex {
        self.y_min + self.height - 1
    }

    fn is_empty(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    fn to_grid_x(&self, x: BoardIndex) -> GridIndex {
        (x - self.x_min) as GridIndex
    }

    fn to_grid_y(&self, y: BoardIndex) -> GridIndex {
        (y - self.y_min) as GridIndex
    }

    fn to_grid_point(&self, x: BoardIndex, y: BoardIndex) -> (GridIndex, GridIndex) {
        (self.to_grid_x(x), self.to_grid_y(y))
    }

    fn x_range(&self) -> Range<BoardIndex> {
        self.x_min..(self.x_min + self.width)
    }

    fn y_range(&self) -> Range<BoardIndex> {
        self.y_min..(self.y_min + self.height)
    }
}

impl Display for Rectangle {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "(minX:{}, minY:{}, width:{}, height:{})",
            self.x_min, self.y_min, self.width, self.height
        )
    }
}

struct ArrayGrid {
    col_size: usize,
    arr: Vec<u8>,
}

impl ArrayGrid {
    fn set(&mut self, xu: usize, yu: usize, val: u8) {
        let idx = self.get_index(xu, yu);
        self.arr[idx] = val;
    }

    fn get(&self, xu: usize, yu: usize) -> u8 {
        let idx = self.get_index(xu, yu);
        self.arr[idx]
    }

    fn get_index(&self, xu: usize, yu: usize) -> usize {
        yu + xu * self.col_size
    }

    fn iter(&self) -> std::slice::Iter<u8> {
        self.arr.iter()
    }

    fn empty() -> ArrayGrid {
        ArrayGrid {
            col_size: 0,
            arr: vec![],
        }
    }

    fn create(row_size: usize, col_size: usize) -> ArrayGrid {
        let size = row_size * col_size;
        let mut arr: Vec<u8> = Vec::with_capacity(size);
        arr.resize(size, 0);

        ArrayGrid { col_size, arr }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn step_one_works_empty() {
        let mut board = DynamicArray2dLifeBoard::empty();
        board.step_one();
        assert_eq!(0, board.get_live_count());
    }

    #[test]
    pub fn step_one_works_blinker() {
        let mut board = DynamicArray2dLifeBoard::empty();
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
    pub fn set_live_minus() {
        let mut board = DynamicArray2dLifeBoard::empty();
        board.set_live(5, 5);
        board.set_live(4, 5);
        assert_eq!(true, board.is_live(5, 5));
        assert_eq!(true, board.is_live(4, 5));
    }

    #[test]
    pub fn can_create_empty_board() {
        let board = DynamicArray2dLifeBoard::empty();
        assert_eq!(0, board.get_live_count());
    }

    #[test]
    pub fn set_live_ensures_capacity() {
        let mut board = DynamicArray2dLifeBoard::empty();
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
        let mut board = DynamicArray2dLifeBoard::empty();
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
        let mut board = DynamicArray2dLifeBoard::empty();
        board.set_live(90, 100);
        assert_eq!(1, board.live_extent.width);
        assert_eq!(1, board.live_extent.height);
        assert_eq!(90, board.live_extent.x_min);
        assert_eq!(100, board.live_extent.y_min);
    }

    #[test]
    pub fn count_live_neighbors_works_at_borders() {
        let mut board = DynamicArray2dLifeBoard::empty();
        board.set_live(0, 0);
        board.set_live(0, 1);

        let neighbors = board.count_live_neighbors(0, 0);
        assert_eq!(neighbors, 1);
    }

    #[test]
    pub fn count_live_neighbors_doesnt_count_self() {
        let mut board = DynamicArray2dLifeBoard::empty();
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
