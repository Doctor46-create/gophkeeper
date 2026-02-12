use clap::{Parser, Subcommand};

use crate::core::{GopherApp, Secret};
use anyhow::Result;

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
    self.inner.login(login, pass).await
  }

  pub async fn sync(&self) -> Result<Vec<Secret>> {
    self.inner.sync_and_decrypt().await
  }

  pub async fn add_secret(&self, sec_type: String, data: String) -> Result<()> {
    self.inner.add_secret(sec_type, data).await
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
    #[arg(short = 't', long, default_value = "text")]
    type_val: String,
    #[arg(short, long)]
    data: String,
  },
  Sync,
  Delete {
    #[arg(short, long)]
    id: String,
  },
}
