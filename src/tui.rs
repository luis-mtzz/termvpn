use crate::app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::*, widgets::Paragraph};
use std::io::{self, Stdout, stdout};
use std::time::Duration;

type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, _app: &App) {
    let greeting = Paragraph::new("Hello, termvpn! Press 'q' to quit.")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    frame.render_widget(greeting, frame.area());
}

pub fn run(terminal: &mut Tui, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.should_quit = true;
                }
            }
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
