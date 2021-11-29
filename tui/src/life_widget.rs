use crate::ConsolePoint;
use engine::life_board::LifeBoard;
use std::borrow::{Borrow, BorrowMut};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Style;
use tui::symbols::Marker;
use tui::text::{Span, Spans};
use tui::widgets::Widget;

pub struct LifeWidgetState {
    pub screen_offset: ConsolePoint,
    pub center_point: ConsolePoint,
    active_style: Style,
    default_style: Style,
}

impl LifeWidgetState {
    pub fn new() -> LifeWidgetState {
        LifeWidgetState {
            screen_offset: ConsolePoint::new(0, 0),
            center_point: ConsolePoint::new(0, 0),
            active_style: Style::default().bg(Color::LightYellow).fg(Color::Green),
            default_style: Style::default().bg(Color::Black).fg(Color::Green),
        }
    }
}

pub struct LifeWidget<'a> {
    board: Box<&'a dyn LifeBoard>,
    state: &'a mut LifeWidgetState,
}

impl<'a> LifeWidget<'a> {
    pub fn new(board: Box<&'a dyn LifeBoard>, state: &'a mut LifeWidgetState) -> LifeWidget<'a> {
        LifeWidget { board, state }
    }
}

impl<'a> Widget for LifeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        const LIVE_CELL: &str = "•"; //█
        const DEAD_CELL: &str = " ";
        let mut state = self.state;
        let offset = &state.screen_offset;
        let center_y = area.height / 2;
        let center_x = area.width / 2;

        //Update center point, need by main game loop to manually toggle active/inactive squares
        state.center_point =
            ConsolePoint::new(center_x as i64 + offset.x, center_y as i64 + offset.y);

        for screen_y_idx in 0..area.height {
            let mut spans: Vec<Span> = Vec::with_capacity(area.width as usize);
            for screen_x_idx in 0..area.width {
                let span_style = if screen_x_idx == center_x && screen_y_idx == center_y {
                    state.active_style
                } else {
                    state.default_style
                };

                let board_x = screen_x_idx as i64 + offset.x;
                let board_y = screen_y_idx as i64 + offset.y;
                if self.board.is_live(board_x, board_y) {
                    spans.push(Span::styled(LIVE_CELL, span_style));
                } else {
                    spans.push(Span::styled(DEAD_CELL, span_style));
                }
            }

            buf.set_spans(
                area.x,
                area.y + screen_y_idx,
                &Spans::from(spans),
                area.width,
            );
        }
    }
}
