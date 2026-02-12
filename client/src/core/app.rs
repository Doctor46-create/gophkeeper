use crate::core::models::Claims;
use crate::core::{client::GopherClient, crypto, models::Secret};
use anyhow::{Context, Result};

pub struct GopherApp {
  api: GopherClient,
  token: Option<String>,
  master_password: Option<String>,
  current_user: Option<String>,
}
use jsonwebtoken;

impl GopherApp {
  pub fn new(base_url: String) -> Self {
    Self {
      api: GopherClient::new(base_url),
      token: None,
      master_password: None,
      current_user: None,
    }
  }

  pub async fn register(&self, login: String, pass: String) -> Result<()> {
    self.api.register(&login, &pass).await
  }

  pub async fn login(&mut self, login: String, pass: String) -> Result<()> {
    let token = self.api.login(&login, &pass).await?;

    self.token = Some(token);
    self.master_password = Some(pass);
    self.current_user = Some(login);

    Ok(())
  }

  pub async fn try_auto_login(&mut self) -> Result<()> {
    let token = GopherClient::load_token().await?;

    let token_data = jsonwebtoken::dangerous::insecure_decode::<Claims>(&token)?;

    self.current_user = Some(token_data.claims.login);
    self.token = Some(token);
    Ok(())
  }

  pub async fn logout(&mut self) -> Result<()> {
    self.token = None;
    self.master_password = None;
    self.current_user = None;

    GopherClient::delete_token().await?;
    Ok(())
  }

  pub fn is_logged_in(&self) -> bool {
    self.token.is_some()
  }

  pub fn master_password_is_none(&self) -> bool {
    self.master_password.is_none()
  }

  pub fn set_master_password(&mut self, pass: String) {
    self.master_password = Some(pass);
  }

  pub fn get_current_user(&self) -> Option<&str> {
    self.current_user.as_deref()
  }

  pub fn set_current_user(&mut self, username: String) {
    self.current_user = Some(username)
  }

  pub fn clear_current_user(&mut self) {
    self.current_user = None
  }

  pub async fn sync_and_decrypt(&self) -> Result<Vec<Secret>> {
    let token = self.token.as_ref().context("Not logged in")?;
    let master = self
      .master_password
      .as_ref()
      .context("Master password not set")?;

    let secrets = self.api.get_secrets(token).await?;

    let mut decrypted = Vec::new();

    for mut sec in secrets {
      sec.data = crypto::decrypt_string(&sec.data, master)?;
      decrypted.push(sec);
    }

    Ok(decrypted)
  }

  pub async fn add_secret(&self, sec_type: String, data: String) -> Result<()> {
    let token = self.token.as_ref().context("Not logged in")?;
    let master = self
      .master_password
      .as_ref()
      .context("Master password not set")?;

    let encrypted = crypto::encrypt_string(&data, master)?;

    let secret = Secret {
      id: crypto::generate_id(&data),
      user_login: "".to_string(),
      secret_type: sec_type,
      data: encrypted,
      created_at: None,
      updated_at: None,
    };

    self.api.add_secret(token, vec![secret]).await?;
    Ok(())
  }

  pub async fn delete_secret(&self, id: String) -> Result<()> {
    let token = self.token.as_ref().context("Not logged in")?;
    self.api.delete_secret(token, &id).await
  }
}
