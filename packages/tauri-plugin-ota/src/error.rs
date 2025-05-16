use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("Network error: {0}")]
  Network(String),
  #[error("Update not available")]
  NoUpdateAvailable,
  #[error("Update already in progress")]
  UpdateInProgress,
  #[error("Hash verification failed")]
  HashVerificationFailed,
  #[error("Extraction failed: {0}")]
  ExtractionFailed(String),
  #[error("Missing configuration: {0}")]
  MissingConfig(String),
  #[error("JSON error: {0}")]
  Json(String),
  #[error("Other error: {0}")]
  Other(String),
  #[cfg(mobile)]
  #[error(transparent)]
  PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::Network(err.to_string())
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Self {
    Error::Json(err.to_string())
  }
}

impl From<zip::result::ZipError> for Error {
  fn from(err: zip::result::ZipError) -> Self {
    Error::ExtractionFailed(err.to_string())
  }
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
