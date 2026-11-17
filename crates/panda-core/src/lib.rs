//! Panda Launcher core: everything except the UI (ADR 0001).

pub mod account;
pub mod auth;
pub mod download;
pub mod java;
pub mod launch;
pub mod net;
pub mod paths;
pub mod version;

/// Errors from installing and launching (auth has its own [`auth::AuthError`]).
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Http(#[from] reqwest::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("invalid metadata: {0}")]
  Json(#[from] serde_json::Error),
  #[error("corrupt download (checksum mismatch): {0}")]
  Checksum(String),
  #[error("cannot unpack natives: {0}")]
  Zip(#[from] zip::result::ZipError),
  #[error("unknown Minecraft version {0}")]
  UnknownVersion(String),
  #[error("Minecraft {0} is too old for this launcher (Fabric needs 1.14+)")]
  UnsupportedVersion(String),
  #[error("Mojang has no Java runtime {0} for this platform")]
  NoJavaRuntime(String),
}

pub type Result<T> = std::result::Result<T, Error>;
