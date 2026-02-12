use crate::tui::app::{InputMode, Screen, TuiApp};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_key(app: &mut TuiApp, key: KeyEvent) {
  if key.kind != KeyEventKind::Press {
    return;
  }

  if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
    app.should_quit = true;
    return;
  }

  if key.modifiers.contains(KeyModifiers::CONTROL) {
    match key.code {
      KeyCode::Char('l') => {
        app.screen = Screen::Login;
        app.input_mode = InputMode::Editing;
        return;
      }
      KeyCode::Char('r') => {
        app.screen = Screen::Register;
        app.input_mode = InputMode::Editing;
        return;
      }
      _ => {}
    }
  }

  match app.input_mode {
    InputMode::Normal => handle_normal(app, key),
    InputMode::Editing => handle_editing(app, key),
  }
}

fn handle_normal(app: &mut TuiApp, key: KeyEvent) {
  match key.code {
    KeyCode::Char('l') if app.screen != Screen::MasterPassword => {app.logout()},
    KeyCode::Esc => {
      app.input_mode = InputMode::Normal;

      app.screen = match app.screen {
        Screen::Secrets | Screen::AddSecret => Screen::Menu,
        Screen::Register => Screen::Register,
        Screen::Login => Screen::Login,
        Screen::Menu => Screen::Menu,
        Screen::MasterPassword => Screen::MasterPassword,
      };
    }
    KeyCode::Char('s') if app.screen != Screen::MasterPassword => {app.sync_secrets()},
    KeyCode::Char('v') if app.screen != Screen::MasterPassword => {app.view_secrets()},
    KeyCode::Char('a') if app.screen != Screen::MasterPassword => {app.enter_add_secret()},
    KeyCode::Char('c') if app.screen == Screen::Secrets => copy_to_clipboard(app),

    KeyCode::Char('d') if app.screen == Screen::Secrets => {
      app.delete_selected();
    }

    KeyCode::Up => app.prev(),
    KeyCode::Down => app.next(),

    KeyCode::Enter => app.submit(),
    KeyCode::Char('e') => app.input_mode = InputMode::Editing,
    _ => {}
  }
}

fn handle_editing(app: &mut TuiApp, key: KeyEvent) {
  match key.code {
    KeyCode::Esc => app.input_mode = InputMode::Normal,
    KeyCode::Tab => app.toggle_field(),
    KeyCode::Enter => app.submit(),
    KeyCode::Backspace => app.backspace(),
    KeyCode::Char(c) => app.push_char(c),
    _ => {}
  }
}

fn copy_to_clipboard(app: &mut TuiApp) {
  use super::clipboard::copy;

  if app.secrets.is_empty() {
    app.notify_error("No secrets to copy");
    return;
  }

  let secret = &app.secrets[app.selected];
  match copy(secret.data.clone()) {
    Ok(_) => app.notify_success("Copied to clipboard"),
    Err(e) => app.notify_error(format!("Clipboard error: {}", e)),
  }
}
