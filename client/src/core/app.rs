use crate::core::client::GopherClient;
use crate::core::crypto;
use crate::core::models::Secret;
use anyhow::{Context, Result};

pub struct GopherApp {
  api: GopherClient,
  token: Option<String>,
}

impl GopherApp {
  pub fn new(base_url: String) -> Self {
    Self {
      api: GopherClient::new(base_url),
      token: None,
    }
  }

  pub async fn register(&self, login: String, pass: String) -> Result<()> {
    self.api.register(&login, &pass).await
  }

  pub async fn login(&mut self, login: String, pass: String) -> Result<()> {
    let token = self.api.login(&login, &pass).await?;
    self.token = Some(token);
    Ok(())
  }

  pub async fn sync_and_decrypt(&self, master_pass: String) -> Result<Vec<Secret>> {
    let token = self
      .token
      .as_ref()
      .context("You are not logged in. Use 'login'-command first.")?;

    let secrets = self.api.get_secrets(token).await?;

    let mut decrypted_secrets = Vec::new();
    for mut sec in secrets {
      sec.data = crypto::decrypt_string(&sec.data, &master_pass)?;
      decrypted_secrets.push(sec);
    }

    Ok(decrypted_secrets)
  }

  pub async fn add_secret(
    &mut self,
    sec_type: String,
    data: String,
    master_pass: String,
  ) -> Result<()> {
    let token = self.token.as_ref().context("Not logged in")?;

    let encrypted_blob = crypto::encrypt_string(&data, &master_pass)?;

    let secret = Secret {
      id: crypto::generate_id(&data),
      user_login: "".to_string(),
      secret_type: sec_type,
      data: encrypted_blob,
      created_at: None,
      updated_at: None,
    };

    self.api.add_secret(token, vec![secret]).await?;
    Ok(())
  }

  pub async fn delete_secret(&self, id: String) -> Result<()> {
    let token = self.token.as_ref().context("Not logged in")?;
    self.api.delete_secret(token, &id).await?;
    Ok(())
  }
}
