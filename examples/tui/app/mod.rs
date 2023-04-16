use anyhow::Ok;
use chrono::{Duration, Local};
use crossterm::event::{self, KeyCode};
use std::path::{Path, PathBuf};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, Paragraph},
    Frame, Terminal,
};

pub struct App {
    file: PathBuf,
}

impl App {
    pub fn new<P>(file: &P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            file: PathBuf::from(file.as_ref()),
        }
    }

    pub fn run<B: Backend>(
        terminal: &mut Terminal<B>,
        app: App,
        tick_rate: Duration,
    ) -> anyhow::Result<()> {
        let mut last_tick = Local::now();
        let mut continue_ = true;
        while continue_ {
            terminal.draw(|f| ui(f, &app))?;
            let time_out = tick_rate
                .checked_sub(&(Local::now() - last_tick))
                .unwrap_or_else(Duration::zero);

            if crossterm::event::poll(std::time::Duration::from_millis(
                time_out.num_milliseconds() as u64,
            ))? {
                if let event::Event::Key(key) = event::read()? {
                    continue_ = app.handle_keyevent(key);
                }
            }
            if tick_rate <= Local::now() - last_tick {
                last_tick = Local::now();
            }
        }

        Ok(())
    }

    fn handle_keyevent(&self, key: event::KeyEvent) -> bool {
        !matches!(key.code, KeyCode::Esc)
    }

    pub fn file_name(&self) -> String {
        self.file.display().to_string()
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let style = Style::default();
    let msg = vec![
        Span::raw(app.file_name()),
        Span::raw(Local::now().format(" %H:%M:%S").to_string()),
    ];
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let file_name = Paragraph::new(text);

    f.render_widget(file_name, chunks[0]);

    let atoms = List::new(Vec::new()).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Messages"),
    );

    f.render_widget(atoms, chunks[1]);
}
