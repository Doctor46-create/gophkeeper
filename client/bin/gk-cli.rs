use anyhow::Result;
use clap::Parser;
use gophkeeper_client::cli::CliApp;
use gophkeeper_client::cli::app::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  let mut app = CliApp::new(cli.address);

  match cli.command {
    Commands::Register { login, pass } => {
      app.register(login, pass).await?;
    }
    Commands::Login { login, pass } => {
      app.login(login, pass).await?;
    }
    Commands::Add {
      type_val,
      data,
      pass,
    } => {
      app.add_secret(type_val, data, pass).await?;
    }
    Commands::Sync { pass } => {
      let secrets = app.sync_and_decrypt(pass).await?;
      println!("--- Your Secrets ---");
      for s in secrets {
        println!("ID: {} | Type: {} | Data: {}", s.id, s.secret_type, s.data);
      }
    }
    Commands::Delete { id } => {
      app.delete_secret(id).await?;
    }
  }

  Ok(())
}
