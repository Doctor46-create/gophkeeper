use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use tokio::fs;
use tokio::io::AsyncReadExt;

use crate::core::models::{AuthRequest, Secret, SyncRequest, Token};

pub struct GopherClient {
  base_url: String,
  client: Client,
}

impl GopherClient {
  pub fn new(base_url: String) -> Self {
    Self {
      base_url,
      client: Client::new(),
    }
  }

  async fn save_token(token: &str) -> Result<()> {
    let home = home::home_dir().context("Missing HOME directory")?;
    let token_path = home.join(".goph_token");
    fs::write(&token_path, token).await?;
    Ok(())
  }

  pub async fn load_token() -> Result<String> {
    let home = home::home_dir().context("Missing HOME directory")?;
    let token_path = home.join(".goph_token");

    let mut file = fs::File::open(&token_path)
      .await
      .context("Token file not found. Please login first.")?;

    let mut token_content = String::new();
    file.read_to_string(&mut token_content).await?;

    Ok(token_content.trim().to_string())
  }

  pub async fn delete_token() -> Result<()> {
    let home = home::home_dir().context("Missing HOME directory")?;
    let token_path = home.join(".goph_token");

    if token_path.exists() {
      fs::remove_file(token_path).await?;
    }

    Ok(())
  }

  pub async fn register(&self, login: &str, password: &str) -> Result<()> {
    let payload = AuthRequest {
      login: login.to_string(),
      password: password.to_string(),
    };
    let url = format!("{}/api/register", self.base_url);

    let response = self.client.post(&url).json(&payload).send().await?;

    if response.status().is_success() {
      return Ok(());
    }

    let status = response.status().as_u16();
    Err(anyhow!("Registration failed with status {}", status))
  }

  pub async fn login(&self, login: &str, password: &str) -> Result<String> {
    let payload = AuthRequest {
      login: login.to_string(),
      password: password.to_string(),
    };
    let url = format!("{}/api/login", self.base_url);

    let response = self.client.post(&url).json(&payload).send().await?;

    if !response.status().is_success() {
      return Err(anyhow!("Login failed"));
    }

    let token_obj: Token = response.json().await?;
    let jwt = token_obj.token;

    Self::save_token(&jwt).await?;

    Ok(jwt)
  }

  pub async fn get_secrets(&self, token: &str) -> Result<Vec<Secret>> {
    let url = format!("{}/api/data", self.base_url);

    let response = self
      .client
      .get(&url)
      .header("Authorization", format!("Bearer {}", token))
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(anyhow!("Failed to fetch secrets"));
    }

    let secrets: Vec<Secret> = response.json().await?;
    Ok(secrets)
  }

  pub async fn add_secret(&self, token: &str, secrets: Vec<Secret>) -> Result<()> {
    let url = format!("{}/api/data", self.base_url);
    let payload = SyncRequest { secrets };

    let response = self
      .client
      .post(&url)
      .header("Authorization", format!("Bearer {}", token))
      .json(&payload)
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(anyhow!("Sync failed"));
    }

    Ok(())
  }

  pub async fn delete_secret(&self, token: &str, id: &str) -> Result<()> {
    let url = format!("{}/api/data?id={}", self.base_url, id);

    let response = self
      .client
      .delete(&url)
      .header("Authorization", format!("Bearer {}", token))
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(anyhow!("Delete failed"));
    }

    Ok(())
  }
}
