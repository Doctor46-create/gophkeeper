use crate::core::GopherApp;
use crate::core::models::Secret;
use std::time::{Duration, Instant, SystemTime};
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
  Login,
  Register,
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
  pub master_password: String,
  pub login_step: LoginStep,

  pub add_type: String,
  pub add_data: String,
  pub add_step: AddStep,

  pub current_user: Option<String>,
  pub login_time: Option<SystemTime>,

  pub notification: Option<(String, Instant)>,
  pub should_quit: bool,
}

impl TuiApp {
  pub fn new() -> std::io::Result<Self> {
    Ok(Self {
      screen: Screen::Login,
      input_mode: InputMode::Editing,
      api: GopherApp::new("http://localhost:8080".into()),
      rt: Runtime::new()?,
      secrets: vec![],
      selected: 0,
      username: String::new(),
      password: String::new(),
      confirm_password: String::new(),
      master_password: String::new(),
      login_step: LoginStep::Username,
      add_type: "password".into(),
      add_data: String::new(),
      add_step: AddStep::Type,
      current_user: None,
      login_time: None,
      notification: None,
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

  pub fn show_success(&mut self, msg: impl Into<String>) {
    self.notification = Some((msg.into(), Instant::now() + Duration::from_secs(2)));
  }

  pub fn show_error(&mut self, msg: impl Into<String>) {
    self.notification = Some((msg.into(), Instant::now() + Duration::from_secs(3)));
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
    use tokio::runtime::Runtime;

    match self
      .rt
      .block_on(self.api.login(self.username.clone(), self.password.clone()))
    {
      Ok(_) => {
        self.current_user = Some(self.username.clone());
        self.master_password = self.password.clone();
        self.screen = Screen::Menu;
        self.input_mode = InputMode::Normal;

        self.notify_success("Logged in successfully");

        self.username.clear();
        self.password.clear();
      }
      Err(e) => self.show_error(format!("❌ Login failed: {}", e)),
    }
  }

  pub fn register(&mut self) {
    if self.password != self.confirm_password {
      self.show_error("❌ Passwords do not match");
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
      Err(e) => self.show_error(format!("Register failed: {}", e)),
    }
  }

  pub fn view_secrets(&mut self) {
    if self.secrets.is_empty() {
      match self
        .rt
        .block_on(self.api.sync_and_decrypt(self.master_password.clone()))
      {
        Ok(data) => {
          self.secrets = data;
          self.selected = 0;
          self.screen = Screen::Secrets;
          self.input_mode = InputMode::Normal;
          self.show_success("Secrets loaded");
        }
        Err(e) => self.show_error(format!("Failed to load secrets: {}", e)),
      }
    } else {
      self.screen = Screen::Secrets;
      self.input_mode = InputMode::Normal;
    }
  }

  pub fn sync_secrets(&mut self) {
    let current_screen = self.screen;
    match self
      .rt
      .block_on(self.api.sync_and_decrypt(self.master_password.clone()))
    {
      Ok(data) => {
        self.secrets = data;
        self.selected = 0;
        self.input_mode = InputMode::Normal;
        self.show_success("🔄 Secrets synced");
        if current_screen == Screen::Menu && !self.secrets.is_empty() {
          self.screen = Screen::Secrets;
        } else {
          self.screen = current_screen;
        }
      }
      Err(e) => self.show_error(format!("❌ Sync failed: {}", e)),
    }
  }

  pub fn delete_selected(&mut self) {
    if self.secrets.is_empty() {
      return;
    }

    let id = self.secrets[self.selected].id.clone();
    if self.rt.block_on(self.api.delete_secret(id.clone())).is_ok() {
      self.secrets.remove(self.selected);
      self.selected = self.selected.saturating_sub(1);
      self.show_success("🗑️ Secret deleted");
    }
  }

  pub fn add_secret(&mut self) {
    if self.add_type.is_empty() || self.add_data.is_empty() {
      self.show_error("❌ Fields cannot be empty");
      return;
    }

    match self.rt.block_on(self.api.add_secret(
      self.add_type.clone(),
      self.add_data.clone(),
      self.master_password.clone(),
    )) {
      Ok(_) => {
        self.add_data.clear();
        self.screen = Screen::Menu;
        self.input_mode = InputMode::Normal;
        self.show_success("✅ Secret added");
      }
      Err(e) => self.show_error(format!("❌ Add failed: {}", e)),
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

      Screen::AddSecret => self.add_secret(),
      _ => {}
    }
  }

  pub fn logout(&mut self) {
    self.current_user = None;
    self.master_password.clear();
    self.secrets.clear();
    self.selected = 0;
    self.screen = Screen::Login;
    self.input_mode = InputMode::Editing;
    self.notify_info("Logged out");
  }
}
