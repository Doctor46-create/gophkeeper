use crate::core::GopherApp;
use crate::core::models::Secret;
use std::time::{Duration, Instant, SystemTime};
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
  Login,
  Register,
  MasterPassword,
  Menu,
  Secrets,
  AddSecret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
  Normal,
  Editing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddStep {
  Type,
  Data,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginStep {
  Username,
  Password,
  ConfirmPassword,
}

pub struct TuiApp {
  pub screen: Screen,
  pub input_mode: InputMode,

  pub api: GopherApp,
  pub rt: Runtime,

  pub secrets: Vec<Secret>,
  pub selected: usize,

  pub username: String,
  pub password: String,
  pub confirm_password: String,
  pub login_step: LoginStep,

  pub add_type: String,
  pub add_data: String,
  pub add_step: AddStep,

  pub login_time: Option<SystemTime>,

  pub notification: Option<(String, Instant)>,
  pub should_quit: bool,
}

impl TuiApp {
  pub fn new() -> std::io::Result<Self> {
    let mut api = GopherApp::new("http://localhost:8080".into());
    let rt = Runtime::new()?;

    let auto_login = rt.block_on(api.try_auto_login()).is_ok();

    let screen = if auto_login {
      Screen::MasterPassword
    } else {
      Screen::Login
    };

    Ok(Self {
      screen,
      input_mode: if auto_login {
        InputMode::Normal
      } else {
        InputMode::Editing
      },

      api,
      rt,

      secrets: vec![],
      selected: 0,

      username: String::new(),
      password: String::new(),
      confirm_password: String::new(),
      login_step: LoginStep::Username,

      add_type: "password".into(),
      add_data: String::new(),
      add_step: AddStep::Type,

      login_time: None,

      notification: if auto_login {
        Some((
          "INFO: Auto login successful".into(),
          Instant::now() + Duration::from_secs(2),
        ))
      } else {
        None
      },

      should_quit: false,
    })
  }

  pub fn notify_success(&mut self, msg: impl Into<String>) {
    self.notification = Some((
      format!("SUCCESS: {}", msg.into()),
      Instant::now() + Duration::from_secs(2),
    ));
  }

  pub fn notify_error(&mut self, msg: impl Into<String>) {
    self.notification = Some((
      format!("ERROR: {}", msg.into()),
      Instant::now() + Duration::from_secs(3),
    ));
  }

  pub fn notify_info(&mut self, msg: impl Into<String>) {
    self.notification = Some((
      format!("INFO: {}", msg.into()),
      Instant::now() + Duration::from_secs(2),
    ));
  }

  pub fn clear_notifications(&mut self) {
    if let Some((_, until)) = &self.notification {
      if Instant::now() > *until {
        self.notification = None;
      }
    }
  }

  pub fn next(&mut self) {
    if !self.secrets.is_empty() {
      self.selected = (self.selected + 1).min(self.secrets.len() - 1);
    }
  }

  pub fn prev(&mut self) {
    if !self.secrets.is_empty() {
      self.selected = self.selected.saturating_sub(1);
    }
  }

  pub fn login(&mut self) {
    match self
      .rt
      .block_on(self.api.login(self.username.clone(), self.password.clone()))
    {
      Ok(_) => {
        self.api.set_current_user(self.username.clone());

        self.api.set_master_password(self.password.clone());

        self.screen = Screen::Menu;
        self.input_mode = InputMode::Normal;

        self.notify_success("Logged in successfully");

        self.username.clear();
        self.password.clear();
      }
      Err(e) => self.notify_error(format!("Login failed: {}", e)),
    }
  }

  pub fn register(&mut self) {
    if self.password != self.confirm_password {
      self.notify_error("Passwords do not match");
      return;
    }

    match self.rt.block_on(
      self
        .api
        .register(self.username.clone(), self.password.clone()),
    ) {
      Ok(_) => {
        self.login_step = LoginStep::Username;
        self.confirm_password.clear();
        self.notify_success("Registered successfully");
        self.login();
      }
      Err(e) => self.notify_error(format!("Register failed: {}", e)),
    }
  }

  pub fn logout(&mut self) {
    let _ = self.rt.block_on(self.api.logout());
    self.api.clear_current_user();

    self.secrets.clear();
    self.selected = 0;
    self.screen = Screen::Login;
    self.input_mode = InputMode::Editing;

    self.notify_info("Logged out");
  }

  pub fn view_secrets(&mut self) {
    if self.secrets.is_empty() {
      match self.rt.block_on(self.api.sync_and_decrypt()) {
        Ok(data) => {
          self.secrets = data;
          self.selected = 0;
          self.screen = Screen::Secrets;
          self.input_mode = InputMode::Normal;
          self.notify_success("Secrets loaded");
        }
        Err(e) => self.notify_error(format!("Failed to load secrets: {}", e)),
      }
    } else {
      self.screen = Screen::Secrets;
      self.input_mode = InputMode::Normal;
    }
  }

  pub fn sync_secrets(&mut self) {
    let current_screen = self.screen;

    match self.rt.block_on(self.api.sync_and_decrypt()) {
      Ok(data) => {
        self.secrets = data;
        self.selected = 0;
        self.input_mode = InputMode::Normal;
        self.notify_success("Secrets synced");

        if current_screen == Screen::Menu && !self.secrets.is_empty() {
          self.screen = Screen::Secrets;
        } else {
          self.screen = current_screen;
        }
      }
      Err(e) => self.notify_error(format!("Sync failed: {}", e)),
    }
  }

  pub fn delete_selected(&mut self) {
    if self.secrets.is_empty() {
      return;
    }

    let id = self.secrets[self.selected].id.clone();
    if self.rt.block_on(self.api.delete_secret(id)).is_ok() {
      self.secrets.remove(self.selected);
      self.selected = self.selected.saturating_sub(1);
      self.notify_success("Secret deleted");
    }
  }

  pub fn add_secret(&mut self) {
    if self.add_type.is_empty() || self.add_data.is_empty() {
      self.notify_error("Fields cannot be empty");
      return;
    }

    match self.rt.block_on(
      self
        .api
        .add_secret(self.add_type.clone(), self.add_data.clone()),
    ) {
      Ok(_) => {
        self.add_data.clear();
        self.screen = Screen::Menu;
        self.input_mode = InputMode::Normal;
        self.notify_success("Secret added");
      }
      Err(e) => self.notify_error(format!("Add failed: {}", e)),
    }
  }

  pub fn enter_add_secret(&mut self) {
    self.screen = Screen::AddSecret;
    self.input_mode = InputMode::Editing;
  }

  pub fn toggle_field(&mut self) {
    match self.screen {
      Screen::Login => {
        self.login_step = match self.login_step {
          LoginStep::Username => LoginStep::Password,
          LoginStep::Password => LoginStep::Username,
          LoginStep::ConfirmPassword => LoginStep::Username,
        };
      }
      Screen::Register => {
        self.login_step = match self.login_step {
          LoginStep::Username => LoginStep::Password,
          LoginStep::Password => LoginStep::ConfirmPassword,
          LoginStep::ConfirmPassword => LoginStep::Username,
        };
      }
      Screen::AddSecret => {
        self.add_step = match self.add_step {
          AddStep::Type => AddStep::Data,
          AddStep::Data => AddStep::Type,
        };
      }
      _ => {}
    }
  }

  pub fn push_char(&mut self, c: char) {
    match self.screen {
      Screen::Login | Screen::Register => match self.login_step {
        LoginStep::Username => self.username.push(c),
        LoginStep::Password => self.password.push(c),
        LoginStep::ConfirmPassword => self.confirm_password.push(c),
      },
      Screen::MasterPassword => {
        self.password.push(c);
      }
      Screen::AddSecret => match self.add_step {
        AddStep::Type => self.add_type.push(c),
        AddStep::Data => self.add_data.push(c),
      },
      _ => {}
    }
  }

  pub fn backspace(&mut self) {
    match self.screen {
      Screen::Login | Screen::Register => match self.login_step {
        LoginStep::Username => {
          self.username.pop();
        }
        LoginStep::Password => {
          self.password.pop();
        }
        LoginStep::ConfirmPassword => {
          self.confirm_password.pop();
        }
      },
      Screen::MasterPassword => {
        self.password.pop();
      }
      Screen::AddSecret => match self.add_step {
        AddStep::Type => {
          self.add_type.pop();
        }
        AddStep::Data => {
          self.add_data.pop();
        }
      },
      _ => {}
    }
  }

  pub fn submit(&mut self) {
    match self.screen {
      Screen::Login => match self.login_step {
        LoginStep::Username => self.login_step = LoginStep::Password,
        LoginStep::Password => self.login(),
        _ => {}
      },
      Screen::Register => match self.login_step {
        LoginStep::Username => self.login_step = LoginStep::Password,
        LoginStep::Password => self.login_step = LoginStep::ConfirmPassword,
        LoginStep::ConfirmPassword => self.register(),
      },

      Screen::MasterPassword => {
        self.submit_master_password();
      }
      Screen::AddSecret => self.add_secret(),
      _ => {}
    }
  }

  pub fn submit_master_password(&mut self) {
    if self.password.is_empty() {
      self.notify_error("Master password cannot be empty");
      return;
    }

    self.api.set_master_password(self.password.clone());
    self.password.clear();

    self.screen = Screen::Menu;
    self.input_mode = InputMode::Normal;

    self.notify_success("Master password set");
  }
}
