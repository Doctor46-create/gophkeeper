use anyhow::Result;
use clap::Parser;
use rpassword::prompt_password;
use tokio::runtime::Runtime;

use gophkeeper_client::cli::CliApp;
use gophkeeper_client::cli::app::Commands;

use gophkeeper_client::cli::app::Cli;

fn main() -> Result<()> {
  let cli = Cli::parse();
  let rt = Runtime::new()?;

  rt.block_on(run(cli))
}

async fn run(cli: Cli) -> Result<()> {
  let mut app = CliApp::new(cli.address.clone());

  if app.inner.try_auto_login().await.is_ok() {
    println!("Auto login successful.");
  }

  match cli.command {
    Commands::Register { login, pass } => {
      app.register(login, pass).await?;
      println!("Registered successfully.");
    }

    Commands::Login { login, pass } => {
      app.login(login, pass).await?;
      println!("Login successful.");
    }

    Commands::Sync => {
      ensure_master_password(&mut app)?;
      let secrets = app.sync().await?;
      println!("{:#?}", secrets);
    }

    Commands::Add { type_val, data } => {
      ensure_master_password(&mut app)?;
      app.add_secret(type_val, data).await?;
      println!("Secret added.");
    }

    Commands::Delete { id } => {
      app.delete_secret(id).await?;
      println!("Secret deleted.");
    }
  }

  Ok(())
}

fn ensure_master_password(app: &mut CliApp) -> Result<()> {
  if app.inner.master_password_is_none() {
    let pass = prompt_password("Enter master password: ")?;
    app.inner.set_master_password(pass);
  }
  Ok(())
}
