use gophkeeper_client::tui::{TuiApp, run_tui};

fn main() -> std::io::Result<()> {
  let app = TuiApp::new()?;
  run_tui(app)
}
