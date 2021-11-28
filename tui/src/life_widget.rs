use crate::ConsolePoint;
use engine::life_board::LifeBoard;
use std::borrow::{Borrow, BorrowMut};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Style;
use tui::symbols::Marker;
use tui::widgets::Widget;

pub struct LifeWidgetState {
    screen_offset: ConsolePoint,
    active_cell: Option<ConsolePoint>,
    active_style: Style,
    default_style: Style,
}

impl LifeWidgetState {
    pub fn new() -> LifeWidgetState {
        LifeWidgetState {
            screen_offset: ConsolePoint::new(0, 0),
            active_cell: None,
            active_style: Style::default().bg(Color::LightYellow).fg(Color::Green),
            default_style: Style::default().bg(Color::Black).fg(Color::Green),
        }
    }

    pub fn screen_offset(mut self, offset: ConsolePoint) -> LifeWidgetState {
        self.screen_offset = offset;
        self
    }

    pub fn active_cell(mut self, cell: ConsolePoint) -> LifeWidgetState {
        self.active_cell = Some(cell);
        self
    }
}

pub struct LifeWidget<'a> {
    board: Box<&'a dyn LifeBoard>,
    state: &'a LifeWidgetState,
}

impl<'a> LifeWidget<'a> {
    pub fn new(board: Box<&'a dyn LifeBoard>, state: &'a LifeWidgetState) -> LifeWidget<'a> {
        LifeWidget { board, state }
    }
}

impl<'a> Widget for LifeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        const LIVE_CELL: char = '•'; //█
        const DEAD_CELL: char = ' ';
        let state = self.state;
        let offset = &state.screen_offset;

        for yi in offset.y..(offset.y + area.height) {
            let mut str = String::with_capacity(area.width as usize);
            for xi in offset.x..(offset.x + area.width) {
                if self.board.is_live(xi as i64, yi as i64) {
                    str.push(LIVE_CELL);
                } else {
                    str.push(DEAD_CELL);
                }
            }
            buf.set_string(area.x, area.y + yi, str, Style::default());
        }
        // let bufAreaString = format!(
        //     "dimensions=(x={}, y={}, width={}, height={})",
        //     area.x, area.y, area.width, area.height
        // );
        // let strlen = bufAreaString.len() as u16;
        // buf.set_string(area.x, area.y, bufAreaString.clone(), Style::default());
        // buf.set_string(
        //     area.x + area.width - strlen,
        //     area.y + area.height - 1,
        //     bufAreaString,
        //     Style::default(),
        // );
        // buf.set_spans(x: u16, y: u16, spans: &Spans<'a>, width: u16)
    }
}
