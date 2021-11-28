use engine::life_board::LifeBoard;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::symbols::Marker;
use tui::widgets::Widget;

pub struct LifeWidget<'a> {
    board: Box<&'a dyn LifeBoard>,
    x: u16,
    y: u16,
}

impl<'a> LifeWidget<'a> {
    const LIVE_CELL: char = '•';
    const DEAD_CELL: char = ' ';

    pub fn new(board: Box<&'a dyn LifeBoard>) -> LifeWidget<'a> {
        LifeWidget {
            board: board,
            x: 0,
            y: 0,
        }
    }

    pub fn x(mut self, x: u16) -> LifeWidget<'a> {
        self.x = x;
        self
    }

    pub fn y(mut self, y: u16) -> LifeWidget<'a> {
        self.y = y;
        self
    }
}

impl<'a> Widget for LifeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bufAreaString = format!(
            "dimensions=(x={}, y={}, width={}, height={})",
            area.x, area.y, area.width, area.height
        );

        for yi in self.y..(self.y + area.height) {
            let mut str = String::with_capacity(area.width as usize);
            for xi in self.x..(self.x + area.width) {
                if self.board.is_live(xi as i64, yi as i64) {
                    str.push(LifeWidget::LIVE_CELL);
                } else {
                    str.push(LifeWidget::DEAD_CELL);
                }
            }
            buf.set_string(area.x, area.y + yi, str, Style::default());
        }
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
