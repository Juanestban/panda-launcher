//! On-disk layout (docs/architecture.md#on-disk-layout).

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Dirs {
  pub root: PathBuf,
}

impl Dirs {
  /// `%LOCALAPPDATA%\PandaLauncher` on Windows; the platform's data folder elsewhere.
  pub fn platform() -> Self {
    let env = |k| std::env::var_os(k).map(PathBuf::from);
    let base = if cfg!(windows) {
      env("LOCALAPPDATA")
    } else if cfg!(target_os = "macos") {
      env("HOME").map(|h| h.join("Library/Application Support"))
    } else {
      env("XDG_DATA_HOME").or_else(|| env("HOME").map(|h| h.join(".local/share")))
    };
    Self { root: base.unwrap_or_else(|| PathBuf::from(".")).join("PandaLauncher") }
  }

  pub fn version(&self, id: &str) -> PathBuf {
    self.root.join("shared/versions").join(id)
  }
  pub fn libraries(&self) -> PathBuf {
    self.root.join("shared/libraries")
  }
  pub fn assets(&self) -> PathBuf {
    self.root.join("shared/assets")
  }
  pub fn natives(&self, id: &str) -> PathBuf {
    self.root.join("shared/natives").join(id)
  }
  pub fn java(&self, component: &str) -> PathBuf {
    self.root.join("shared/java").join(component)
  }
  pub fn profile(&self, name: &str) -> PathBuf {
    self.root.join("profiles").join(name)
  }
  pub fn logs(&self) -> PathBuf {
    self.root.join("logs")
  }
}
