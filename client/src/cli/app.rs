use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::core::GopherApp;
use crate::core::models::{DecryptedSecret, SecretPayload};

pub struct CliApp {
  pub inner: GopherApp,
}

impl CliApp {
  pub fn new(base_url: String) -> Self {
    Self {
      inner: GopherApp::new(base_url),
    }
  }

  pub async fn register(&self, login: String, pass: String) -> Result<()> {
    self.inner.register(login, pass).await
  }

  pub async fn login(&mut self, login: String, pass: String) -> Result<()> {
    self.inner.login(login.clone(), pass.clone()).await?;
    self.inner.set_current_user(login);
    self.inner.set_master_password(pass);
    Ok(())
  }

  pub async fn add_secret(&self, payload: SecretPayload) -> Result<()> {
    self.inner.add_secret(payload).await
  }

  pub async fn sync(&self) -> Result<Vec<DecryptedSecret>> {
    self.inner.sync_and_decrypt().await
  }

  pub async fn logout(&mut self) -> Result<()> {
    self.inner.logout().await
  }

  pub async fn delete_secret(&self, id: String) -> Result<()> {
    self.inner.delete_secret(id).await
  }
}

#[derive(Parser)]
#[command(author, version, about = "GopherKeeper CLI Client")]
pub struct Cli {
  #[arg(short, long, default_value = "http://localhost:8080")]
  pub address: String,

  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Register {
    #[arg(short, long)]
    login: String,
    #[arg(short, long)]
    pass: String,
  },
  Login {
    #[arg(short, long)]
    login: String,
    #[arg(short, long)]
    pass: String,
  },
  Add {
    #[command(subcommand)]
    secret_type: SecretTypeCommands,
  },
  Sync,
  Delete {
    #[arg(short, long)]
    id: String,
  },
  Logout,
}

#[derive(Subcommand)]
pub enum SecretTypeCommands {
  Password(PasswordArgs),
  Note(NoteArgs),
  Card(CardArgs),
}

#[derive(clap::Args)]
pub struct PasswordArgs {
  #[arg(short, long)]
  pub title: String,
  #[arg(short, long)]
  pub login: String,
  #[arg(short, long)]
  pub password: String,
  #[arg(short, long)]
  pub url: Option<String>,
}

#[derive(clap::Args)]
pub struct NoteArgs {
  #[arg(short, long)]
  pub title: String,
  #[arg(short, long)]
  pub content: String,
}

#[derive(clap::Args)]
pub struct CardArgs {
  #[arg(short, long)]
  pub title: String,
  #[arg(short, long)]
  pub holder: String,
  #[arg(short, long)]
  pub number: String,
  #[arg(short, long)]
  pub expiry: String,
  #[arg(short, long)]
  pub cvv: String,
}
