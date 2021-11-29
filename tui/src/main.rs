#![allow(unused_imports)]
mod life_widget;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use engine::life_board::{BoardPoint, LifeBoard};
use engine::pattern::Pattern;
use life_widget::{LifeWidget, LifeWidgetState};
use std::io;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Chart, Paragraph, Widget, Wrap};
use tui::{Frame, Terminal};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut life_board = engine::new_dynamic_vector_board();
    let mut life_widget_state = LifeWidgetState::new();
    let mut paused = true; //start in paused state
    let mut speed: u64 = 5; //frames per second

    let mut last_input_event: String = String::default();
    let mut next_tick = Instant::now();

    loop {
        draw(
            &mut terminal,
            &mut life_widget_state,
            &life_board,
            last_input_event.clone(),
            speed,
            paused,
        )?;

        let tick_rate = Duration::from_millis(1000 / speed as u64);
        if !paused && next_tick <= Instant::now() {
            life_board.step_one();
            next_tick = Instant::now() + tick_rate;
        }

        let timeout = next_tick
            .checked_duration_since(Instant::now())
            .unwrap_or_else(|| Duration::from_secs(5000));

        if poll(timeout)? {
            // It's guaranteed that the `read()` won't block when `poll()` returns `true`
            let event = read()?;
            last_input_event = format!("{:?}", event);

            match event {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('n') => {
                        life_board.step_one();
                        next_tick = Instant::now() + tick_rate; //delay next update
                    }
                    KeyCode::Char('p') => {
                        paused = !paused;
                        if !paused {
                            next_tick = Instant::now(); //force immediate screen update
                        }
                    }
                    KeyCode::Char('>') | KeyCode::Char(']') => {
                        speed += 1;
                        next_tick = Instant::now(); //force immediate screen update
                    }
                    KeyCode::Char('<') | KeyCode::Char('[') => {
                        match speed {
                            1 => paused = true,
                            _ => speed -= 1,
                        };
                        next_tick = Instant::now(); //force immediate screen update
                    }
                    KeyCode::Char('a') | KeyCode::Left => {
                        life_widget_state.screen_offset = life_widget_state
                            .screen_offset
                            .move_left(calc_move_offset(event))
                    }
                    KeyCode::Char('d') | KeyCode::Right => {
                        life_widget_state.screen_offset = life_widget_state
                            .screen_offset
                            .move_right(calc_move_offset(event))
                    }
                    KeyCode::Char('w') | KeyCode::Up => {
                        life_widget_state.screen_offset = life_widget_state
                            .screen_offset
                            .move_up(calc_move_offset(event))
                    }
                    KeyCode::Char('s') | KeyCode::Down => {
                        life_widget_state.screen_offset = life_widget_state
                            .screen_offset
                            .move_down(calc_move_offset(event))
                    }
                    KeyCode::Char('c') => life_board = engine::new_dynamic_vector_board(),
                    KeyCode::Char(' ') => {
                        let bp = life_widget_state.center_point.to_board_point();
                        let is_live = life_board.is_live_point(&bp);
                        life_board.set_liveness_point(&bp, !is_live);
                    }
                    KeyCode::Char('1') => life_board.draw_pattern(
                        &Pattern::ACORN(),
                        &life_widget_state.center_point.to_board_point(),
                    ),
                    KeyCode::Char('2') => life_board.draw_pattern(
                        &Pattern::BLOCK(),
                        &life_widget_state.center_point.to_board_point(),
                    ),
                    KeyCode::Char('3') => life_board.draw_pattern(
                        &Pattern::BEACON(),
                        &life_widget_state.center_point.to_board_point(),
                    ),
                    KeyCode::Char('4') => life_board.draw_pattern(
                        &Pattern::PULSAR(),
                        &life_widget_state.center_point.to_board_point(),
                    ),
                    _ => {}
                },
                // Event::Mouse(event) => last_input_event = format!("{:?}", event),
                // Event::Resize(width, height) => {
                //     last_input_event = format!("New size {}x{}", width, height)
                // }
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }

    terminal.clear()?;
    Ok(())
}

fn calc_move_offset(event: crossterm::event::KeyEvent) -> i64 {
    let alt = event.modifiers.contains(KeyModifiers::ALT);
    let ctrl = event.modifiers.contains(KeyModifiers::CONTROL);
    if ctrl && alt {
        25
    } else if alt {
        10
    } else {
        1
    }
}

fn draw<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    life_widget_state: &'a mut LifeWidgetState,
    board: &dyn LifeBoard,
    last_input_event: String,
    speed: u64,
    paused: bool,
) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(f.size());

        let main_block = Block::default()
            .title("Conway's Game of Life")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let main_block_rect = main_block.inner(chunks[0]);
        f.render_widget(main_block, chunks[0]);

        let life_widget = LifeWidget::new(Box::new(board), life_widget_state);
        f.render_widget(life_widget, main_block_rect);

        let status_spans = if paused {
            Spans::from(vec![
                Span::from("paused, will run at "),
                Span::from(speed.to_string()),
                Span::from(" frames/sec when unpaused"),
            ])
        } else {
            Spans::from(vec![
                Span::from("running at "),
                Span::from(speed.to_string()),
                Span::from(" frames/sec"),
            ])
        };

        let controls_text =
            "(p)lay/(p)ause, (n)ext step, (c)lear, (q)uit, arrows move, space toggles center square liveness, 1-4 to insert pattern at center, (> or ]) speed up, (< or [) slow down";

        //let debug_text = Spans::from(vec![Span::from(last_input_event)]);
        let stats_text = board.get_stats().iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join(", ");

        let controls_block = Paragraph::new(vec![
            status_spans,
            Spans::from(vec![Span::from(controls_text)]),
            Spans::from(vec![Span::from(stats_text)]),
            //debug_text,
        ])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
        f.render_widget(controls_block, chunks[1]);
    })?;

    Ok(())
}

pub struct ConsolePoint {
    x: i64,
    y: i64,
}

impl ConsolePoint {
    pub fn new(x: i64, y: i64) -> ConsolePoint {
        ConsolePoint { x, y }
    }

    pub fn to_board_point(&self) -> BoardPoint {
        BoardPoint::new(self.x, self.y)
    }

    pub fn move_left(&self, offset: i64) -> ConsolePoint {
        ConsolePoint {
            x: self.x - offset,
            y: self.y,
        }
    }

    pub fn move_right(&self, offset: i64) -> ConsolePoint {
        ConsolePoint {
            x: self.x + offset,
            y: self.y,
        }
    }

    pub fn move_up(&self, offset: i64) -> ConsolePoint {
        ConsolePoint {
            x: self.x,
            y: self.y - offset,
        }
    }

    pub fn move_down(&self, offset: i64) -> ConsolePoint {
        ConsolePoint {
            x: self.x,
            y: self.y + offset,
        }
    }
}
