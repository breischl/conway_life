mod life_widget;
use crossterm::event::{poll, read, Event, KeyCode};
use engine::life_board::LifeBoard;
use life_widget::LifeWidget;
use std::io;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, Chart, Paragraph, Widget, Wrap};
use tui::Frame;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
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

            let mut life_board = engine::new_dynamic_vector_board();
            life_board.set_live(0, 0);
            life_board.set_live(0, 1);
            life_board.set_live(1, 0);
            life_board.set_live(1, 1);
            life_board.set_live(1, 2);
            life_board.set_live(3, 4);
            let life_widget = LifeWidget::new(Box::new(&life_board)).x(0).y(0);
            f.render_widget(life_widget, main_block_rect);

            let controls_text = format!("(q)uit");
            let controls_block = Paragraph::new(controls_text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            f.render_widget(controls_block, chunks[1]);
        })?;

        if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    _ => println!("{:?}", event),
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
    Ok(())
}
