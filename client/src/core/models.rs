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

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRequest {
  pub secrets: Vec<Secret>,
}
