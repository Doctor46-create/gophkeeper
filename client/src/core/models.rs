use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
  pub login: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
  pub id: String,
  pub user_login: String,
  #[serde(rename = "type")]
  pub secret_type: String,
  pub data: String,
  pub created_at: Option<String>,
  pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum SecretPayload {
  Password {
    title: String,
    login: String,
    password: String,
    url: Option<String>,
  },
  Note {
    title: String,
    content: String,
  },
  Card {
    title: String,
    holder: String,
    number: String,
    expiry: String,
    cvv: String,
  },
}

#[derive(Debug, Clone)]
pub struct DecryptedSecret {
  pub id: String,
  pub secret_type: String,
  pub payload: SecretPayload,
  pub created_at: Option<String>,
  pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRequest {
  pub secrets: Vec<Secret>,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
  pub login: String,
  exp: usize,
}
