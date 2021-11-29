#![allow(unused_imports)]
mod life_widget;
use crossterm::event::{poll, read, Event, KeyCode};
use engine::life_board::LifeBoard;
use life_widget::{LifeWidget, LifeWidgetState};
use std::io;
use std::time::Duration;
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
    life_board.set_live(0, 0);
    life_board.set_live(0, 1);
    life_board.set_live(1, 0);
    life_board.set_live(1, 1);
    life_board.set_live(1, 2);
    life_board.set_live(3, 4);
    let mut life_widget_state = LifeWidgetState::new();
    let mut paused = true; //start in paused state
    let mut speed: u64 = 1; //frame per second

    let mut last_input_event: String = String::default();

    loop {
        draw(
            &mut terminal,
            &life_widget_state,
            &life_board,
            last_input_event.clone(),
            speed,
            paused,
        )?;

        let poll_duration = if paused {
            Duration::from_millis(100)
        } else {
            Duration::from_millis(1000 / speed as u64)
        };

        if poll(poll_duration)? {
            // It's guaranteed that the `read()` won't block when `poll()` returns `true`
            let event = read()?;
            last_input_event = format!("{:?}", event);

            match event {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('n') => {
                        if paused {
                            //Single-step only works if we're paused first
                            life_board.step_one()
                        }
                    }
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('>') | KeyCode::Char(']') => {
                        speed += 1;
                    }
                    KeyCode::Char('<') | KeyCode::Char('[') => match speed {
                        1 => paused = true,
                        _ => speed -= 1,
                    },
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

        if (!paused) {
            life_board.step_one();
        }
    }

    terminal.clear()?;
    Ok(())
}

fn draw<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    life_widget_state: &'a LifeWidgetState,
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
            "(space) -> play/pause, (> or ]) speed up, (< or [) slow down, (n)ext step, (q)uit";

        let debug_text = Spans::from(vec![Span::from(last_input_event)]);

        let controls_block = Paragraph::new(vec![
            status_spans,
            Spans::from(vec![Span::from(controls_text)]),
            debug_text,
        ])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
        f.render_widget(controls_block, chunks[1]);
    })?;

    Ok(())
}

pub struct ConsolePoint {
    x: u16,
    y: u16,
}

impl ConsolePoint {
    pub fn new(x: u16, y: u16) -> ConsolePoint {
        ConsolePoint { x, y }
    }
}
