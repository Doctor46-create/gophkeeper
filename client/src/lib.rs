pub mod core;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "tui")]
pub mod tui;

pub use core::{GopherApp, Secret};

#[cfg(feature = "cli")]
pub use cli::CliApp;

#[cfg(feature = "tui")]
pub use tui::{TuiApp, run_tui};
