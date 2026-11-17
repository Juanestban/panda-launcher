//! Panda Launcher core: everything except the UI (ADR 0001).

pub mod account;
pub mod auth;
pub mod download;
pub mod fabric;
pub mod java;
pub mod launch;
pub mod mods;
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
  #[error("Fabric does not support Minecraft {0}")]
  NoFabric(String),
  #[error("{0} has no Fabric build for Minecraft {1}")]
  NoCompatibleMod(String, String),
  #[error("{0} cannot be installed together with {1}")]
  IncompatibleMods(String, String),
  #[error("mod {0} is not installed")]
  ModNotInstalled(String),
  #[error("{0} is needed by {needers}", needers = .1.join(", "))]
  ModStillRequired(String, Vec<String>),
  #[error("Modrinth is busy, try again in a minute")]
  RateLimited,
  #[error("Mojang has no Java runtime {0} for this platform")]
  NoJavaRuntime(String),
}

pub type Result<T> = std::result::Result<T, Error>;
