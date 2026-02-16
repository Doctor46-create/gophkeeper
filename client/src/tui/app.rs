use crate::core::GopherApp;
use crate::core::models::{DecryptedSecret, SecretPayload};

use std::time::{Duration, Instant};
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
pub enum LoginStep {
  Username,
  Password,
  ConfirmPassword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddKind {
  Password,
  Note,
  Card,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddField {
  Kind,
  Title,
  Field1,
  Field2,
  Field3,
  Field4,
}

pub struct TuiApp {
  pub screen: Screen,
  pub input_mode: InputMode,

  pub api: GopherApp,
  pub rt: Runtime,

  pub secrets: Vec<DecryptedSecret>,
  pub selected: usize,

  pub username: String,
  pub password: String,
  pub confirm_password: String,
  pub login_step: LoginStep,

  pub add_kind: AddKind,
  pub add_field: AddField,

  pub title: String,
  pub field1: String,
  pub field2: String,
  pub field3: String,
  pub field4: String,

  pub detail_selected: usize,

  pub notification: Option<(String, Instant)>,

  pub should_quit: bool,
}

impl TuiApp {
  pub fn new() -> std::io::Result<Self> {
    let mut api = GopherApp::new("http://localhost:8080".into());
    let rt = Runtime::new()?;

    let auto_login = rt.block_on(api.try_auto_login()).is_ok();

    Ok(Self {
      screen: if auto_login {
        Screen::MasterPassword
      } else {
        Screen::Login
      },
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
      add_kind: AddKind::Password,
      add_field: AddField::Title,
      title: String::new(),
      field1: String::new(),
      field2: String::new(),
      field3: String::new(),
      field4: String::new(),
      detail_selected: 0,
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

  fn notify(&mut self, msg: impl Into<String>, seconds: u64) {
    self.notification = Some((msg.into(), Instant::now() + Duration::from_secs(seconds)));
  }

  pub fn clear_notifications(&mut self) {
    if let Some((_, until)) = &self.notification
      && Instant::now() > *until {
        self.notification = None;
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
      self.notify("Passwords do not match", 3);
      return;
    }

    match self.rt.block_on(
      self
        .api
        .register(self.username.clone(), self.password.clone()),
    ) {
      Ok(_) => {
        self.notify("Registered successfully", 2);
        self.login();
      }
      Err(e) => self.notify(format!("Register failed: {e}"), 3),
    }
  }

  pub fn logout(&mut self) {
    let _ = self.rt.block_on(self.api.logout());
    self.secrets.clear();
    self.screen = Screen::Login;
    self.notify("Logged out", 2);
  }

  pub fn sync_secrets(&mut self) {
    match self.rt.block_on(self.api.sync_and_decrypt()) {
      Ok(data) => {
        self.secrets = data;
        self.selected = 0;
        self.screen = Screen::Secrets;
        self.notify("Secrets synced", 2);
      }
      Err(e) => self.notify(format!("Sync failed: {e}"), 3),
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
      self.notify("Secret deleted", 2);
    }
  }

  pub fn enter_add_secret(&mut self) {
    self.screen = Screen::AddSecret;
    self.reset_add_fields();
  }

  fn reset_add_fields(&mut self) {
    self.title.clear();
    self.field1.clear();
    self.field2.clear();
    self.field3.clear();
    self.field4.clear();
    self.add_field = AddField::Title;
  }

  pub fn add_secret(&mut self) {
    if self.title.is_empty() {
      self.notify("Title required", 3);
      return;
    }

    let payload = match self.add_kind {
      AddKind::Password => SecretPayload::Password {
        title: self.title.clone(),
        login: self.field1.clone(),
        password: self.field2.clone(),
        url: if self.field3.is_empty() {
          None
        } else {
          Some(self.field3.clone())
        },
      },
      AddKind::Note => SecretPayload::Note {
        title: self.title.clone(),
        content: self.field1.clone(),
      },
      AddKind::Card => SecretPayload::Card {
        title: self.title.clone(),
        holder: self.field1.clone(),
        number: self.field2.clone(),
        expiry: self.field3.clone(),
        cvv: self.field4.clone(),
      },
    };

    match self.rt.block_on(self.api.add_secret(payload)) {
      Ok(_) => {
        self.screen = Screen::Menu;
        self.input_mode = InputMode::Normal;
        self.notify("Secret added", 2);
      }
      Err(e) => self.notify(format!("Add failed: {e}"), 3),
    }
  }

  pub fn next(&mut self) {
    if !self.secrets.is_empty() {
      self.selected = (self.selected + 1).min(self.secrets.len() - 1);
      self.detail_selected = 0;
    }
  }

  pub fn prev(&mut self) {
    if !self.secrets.is_empty() {
      self.selected = self.selected.saturating_sub(1);
      self.detail_selected = 0;
    }
  }

  pub fn push_char(&mut self, c: char) {
    match self.screen {
      Screen::Login | Screen::Register => match self.login_step {
        LoginStep::Username => self.username.push(c),
        LoginStep::Password => self.password.push(c),
        LoginStep::ConfirmPassword => self.confirm_password.push(c),
      },
      Screen::MasterPassword => self.password.push(c),
      Screen::AddSecret => match self.add_field {
        AddField::Kind => {}
        AddField::Title => self.title.push(c),
        AddField::Field1 => self.field1.push(c),
        AddField::Field2 => self.field2.push(c),
        AddField::Field3 => self.field3.push(c),
        AddField::Field4 => self.field4.push(c),
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
      Screen::AddSecret => match self.add_field {
        AddField::Kind => {}
        AddField::Title => {
          self.title.pop();
        }
        AddField::Field1 => {
          self.field1.pop();
        }
        AddField::Field2 => {
          self.field2.pop();
        }
        AddField::Field3 => {
          self.field3.pop();
        }
        AddField::Field4 => {
          self.field4.pop();
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

  pub fn toggle_field(&mut self) {
    match self.screen {
      Screen::AddSecret => {
        let field_order: Vec<AddField> = match self.add_kind {
          AddKind::Password => vec![
            AddField::Kind,
            AddField::Title,
            AddField::Field1,
            AddField::Field2,
            AddField::Field3,
          ],
          AddKind::Note => vec![AddField::Kind, AddField::Title, AddField::Field1],
          AddKind::Card => vec![
            AddField::Kind,
            AddField::Title,
            AddField::Field1,
            AddField::Field2,
            AddField::Field3,
            AddField::Field4,
          ],
        };

        if let Some(pos) = field_order.iter().position(|f| *f == self.add_field) {
          let next_index = (pos + 1) % field_order.len();
          self.add_field = field_order[next_index];
        }
      }

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

  pub fn current_secret_fields(&self) -> Vec<(String, String)> {
    if let Some(secret) = self.secrets.get(self.selected) {
      match &secret.payload {
        SecretPayload::Password {
          title,
          login,
          password,
          url,
        } => {
          let mut fields = vec![
            ("Title".into(), title.clone()),
            ("Login".into(), login.clone()),
            ("Password".into(), password.clone()),
          ];
          if let Some(u) = url {
            fields.push(("URL".into(), u.clone()));
          }
          fields
        }
        SecretPayload::Note { title, content } => {
          vec![
            ("Title".into(), title.clone()),
            ("Content".into(), content.clone()),
          ]
        }
        SecretPayload::Card {
          title,
          holder,
          number,
          expiry,
          cvv,
        } => {
          vec![
            ("Title".into(), title.clone()),
            ("Holder".into(), holder.clone()),
            ("Number".into(), number.clone()),
            ("Expiry".into(), expiry.clone()),
            ("CVV".into(), cvv.clone()),
          ]
        }
      }
    } else {
      vec![]
    }
  }
}
