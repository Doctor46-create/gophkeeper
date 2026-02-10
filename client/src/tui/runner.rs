use crate::tui::app::TuiApp;
use crate::tui::events;
use crate::tui::ui;
use crossterm::{
  ExecutableCommand,
  event::{self, Event as CEvent},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;

pub fn run_tui(mut app: TuiApp) -> io::Result<()> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  stdout.execute(EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  terminal.clear()?;

  while !app.should_quit {
    terminal.draw(|f| ui::draw(f, &app))?;

    if event::poll(Duration::from_millis(100))? {
      if let CEvent::Key(key) = event::read()? {
        events::handle_key(&mut app, key);
      }
    }

    app.clear_notifications();
  }

  disable_raw_mode()?;
  io::stdout().execute(LeaveAlternateScreen)?;

  Ok(())
}
