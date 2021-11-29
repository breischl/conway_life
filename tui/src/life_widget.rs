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
    active_style: Style,
    default_style: Style,
}

impl LifeWidgetState {
    pub fn new() -> LifeWidgetState {
        LifeWidgetState {
            screen_offset: ConsolePoint::new(0, 0),
            active_style: Style::default().bg(Color::LightYellow).fg(Color::Green),
            default_style: Style::default().bg(Color::Black).fg(Color::Green),
        }
    }

    pub fn screen_offset(mut self, offset: ConsolePoint) -> LifeWidgetState {
        self.screen_offset = offset;
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
        const LIVE_CELL: &str = "•"; //█
        const DEAD_CELL: &str = " ";
        let state = self.state;
        let offset = &state.screen_offset;
        let width = area.width as i64;
        let height = area.height as i64;
        let center_y = offset.y + (area.height as i64 / 2);
        let center_x = offset.x + (area.width as i64 / 2);

        for yi in offset.y..(offset.y + height) {
            let mut spans: Vec<Span> = Vec::with_capacity(area.width as usize);
            for xi in offset.x..(offset.x + width) {
                let span_style = if xi == center_x && yi == center_y {
                    state.active_style
                } else {
                    state.default_style
                };

                if self.board.is_live(xi as i64, yi as i64) {
                    spans.push(Span::styled(LIVE_CELL, span_style));
                } else {
                    spans.push(Span::styled(DEAD_CELL, span_style));
                }
            }

            buf.set_spans(area.x, area.y + yi, &Spans::from(spans), area.width);
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
