//! Launcher settings, kept in `settings.json` at the data root.

use crate::Result;
use crate::paths::Dirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
  /// `"es"` or `"en"`; `None` follows the OS language.
  pub language: Option<String>,
  /// Java heap for the game (`-Xmx`).
  pub memory_mb: u32,
  /// Check GitHub Releases on start and hourly (ADR 0010).
  pub auto_update: bool,
}

impl Default for Settings {
  fn default() -> Self {
    Self { language: None, memory_mb: 4096, auto_update: true }
  }
}

fn path(dirs: &Dirs) -> std::path::PathBuf {
  dirs.root.join("settings.json")
}

/// The saved settings; defaults when the file is missing or unreadable, so a bad file never blocks the launcher.
pub fn load(dirs: &Dirs) -> Settings {
  std::fs::read(path(dirs)).ok().and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
}

/// Writes to a temporary file first so a crash mid-write keeps the old settings.
pub fn save(dirs: &Dirs, settings: &Settings) -> Result<()> {
  let path = path(dirs);
  std::fs::create_dir_all(&dirs.root)?;
  let tmp = path.with_extension("json.part");
  std::fs::write(&tmp, serde_json::to_vec_pretty(settings)?)?;
  std::fs::rename(tmp, path)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn round_trip_and_defaults() {
    let dirs = Dirs { root: std::env::temp_dir().join(format!("panda-test-settings-{}", std::process::id())) };
    let _ = std::fs::remove_dir_all(&dirs.root);
    assert_eq!(load(&dirs), Settings::default());

    let s = Settings { language: Some("en".into()), memory_mb: 8192, auto_update: false };
    save(&dirs, &s).unwrap();
    assert_eq!(load(&dirs), s);

    // Fields added later fall back to defaults; garbage falls back to all defaults.
    std::fs::write(path(&dirs), r#"{"memory_mb": 2048}"#).unwrap();
    assert_eq!(load(&dirs), Settings { memory_mb: 2048, ..Settings::default() });
    std::fs::write(path(&dirs), "not json").unwrap();
    assert_eq!(load(&dirs), Settings::default());
    let _ = std::fs::remove_dir_all(&dirs.root);
  }
}
