use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateArgs {
  pub auth_url: String,
  pub callback_scheme: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResult {
  pub success: bool,
  pub token: Option<String>,
  pub error: Option<String>,
}