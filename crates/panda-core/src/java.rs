//! Managed Java runtimes from Mojang's runtime manifest (ADR 0008).

use crate::download::{self, FileSpec};
use crate::paths::Dirs;
use crate::version::JavaVersion;
use crate::{Error, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const ALL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";
/// Written last, so a half-finished install is retried instead of trusted.
const DONE: &str = ".panda-complete";

#[derive(Deserialize)]
struct Runtime {
  manifest: Ref,
  version: RuntimeVersion,
}

#[derive(Deserialize)]
struct RuntimeVersion {
  name: String,
}

#[derive(Deserialize)]
struct Ref {
  sha1: String,
  size: u64,
  url: String,
}

#[derive(Deserialize)]
struct RuntimeManifest {
  files: HashMap<String, Entry>,
}

#[derive(Deserialize)]
struct Entry {
  #[serde(rename = "type")]
  kind: String,
  // Only Unix needs these (exec bit, symlinks); Windows builds never read them.
  #[serde(default)]
  #[cfg_attr(not(unix), allow(dead_code))]
  executable: bool,
  downloads: Option<EntryDownloads>,
  #[cfg_attr(not(unix), allow(dead_code))]
  target: Option<String>,
}

#[derive(Deserialize)]
struct EntryDownloads {
  raw: Ref,
}

fn platform() -> &'static str {
  match (std::env::consts::OS, std::env::consts::ARCH) {
    ("windows", "aarch64") => "windows-arm64",
    ("windows", "x86") => "windows-x86",
    ("windows", _) => "windows-x64",
    ("macos", "aarch64") => "mac-os-arm64",
    ("macos", _) => "mac-os",
    (_, "x86") => "linux-i386",
    _ => "linux",
  }
}

/// The requested component, or another one with the same major version: ARM platforms ship
/// Java 17 only as `java-runtime-gamma`, never as the `java-runtime-beta` that 1.18 asks for.
fn pick(mut runtimes: HashMap<String, Vec<Runtime>>, java: &JavaVersion) -> Option<Runtime> {
  let first = |list: Vec<Runtime>| list.into_iter().next();
  if let Some(r) = runtimes.remove(&java.component).and_then(first) {
    return Some(r);
  }
  let major = java.major_version.to_string();
  let mut same_major: Vec<_> = runtimes
    .into_iter()
    .filter_map(|(name, list)| first(list).map(|r| (name, r)))
    .filter(|(_, r)| r.version.name.split(['.', 'u']).next() == Some(major.as_str()))
    .collect();
  same_major.sort_by(|a, b| a.0.cmp(&b.0)); // deterministic choice
  same_major.into_iter().next().map(|(_, r)| r)
}

/// `javaw.exe` on Windows so no console window opens.
fn java_exe(root: &Path) -> PathBuf {
  if cfg!(windows) {
    root.join("bin/javaw.exe")
  } else if cfg!(target_os = "macos") {
    root.join("jre.bundle/Contents/Home/bin/java")
  } else {
    root.join("bin/java")
  }
}

/// The Java executable a version asks for, downloading it if needed.
pub async fn ensure(http: &Client, dirs: &Dirs, java: &JavaVersion, progress: impl Fn(usize, usize)) -> Result<PathBuf> {
  let root = dirs.java(&java.component);
  if root.join(DONE).exists() {
    return Ok(java_exe(&root));
  }

  let mut all: HashMap<String, HashMap<String, Vec<Runtime>>> = download::get_json(http, ALL).await?;
  let runtimes = all.remove(platform()).unwrap_or_default();
  let m = pick(runtimes, java).ok_or_else(|| Error::NoJavaRuntime(java.component.clone()))?.manifest;
  let manifest_file = FileSpec { url: m.url, path: root.join(".manifest.json"), sha1: m.sha1, size: Some(m.size) };
  let manifest: RuntimeManifest = download::json(http, &manifest_file).await?;

  let mut files = Vec::new();
  for (name, entry) in &manifest.files {
    let path = root.join(name);
    match (entry.kind.as_str(), &entry.downloads) {
      ("directory", _) => std::fs::create_dir_all(&path)?,
      ("file", Some(d)) => files.push(FileSpec { url: d.raw.url.clone(), path, sha1: d.raw.sha1.clone(), size: Some(d.raw.size) }),
      _ => {}
    }
  }
  download::fetch_all(http, &files, progress).await?;

  #[cfg(unix)]
  for (name, entry) in &manifest.files {
    use std::os::unix::fs::PermissionsExt;
    let path = root.join(name);
    if entry.kind == "file" && entry.executable {
      std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
    }
    // macOS runtimes use a few symlinks; Windows runtimes have none.
    if let ("link", Some(target)) = (entry.kind.as_str(), &entry.target)
      && std::fs::symlink_metadata(&path).is_err()
    {
      std::os::unix::fs::symlink(target, &path)?;
    }
  }

  std::fs::write(root.join(DONE), "")?;
  Ok(java_exe(&root))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn runtime(version: &str) -> Vec<Runtime> {
    vec![Runtime { manifest: Ref { sha1: String::new(), size: 0, url: version.into() }, version: RuntimeVersion { name: version.into() } }]
  }

  #[test]
  fn falls_back_to_same_major_version() {
    let java = |c: &str, m| JavaVersion { component: c.into(), major_version: m };
    let all = || {
      HashMap::from([
        ("java-runtime-beta".to_string(), vec![]),
        ("java-runtime-gamma".to_string(), runtime("17.0.15")),
        ("java-runtime-delta".to_string(), runtime("21.0.7")),
        ("jre-legacy".to_string(), runtime("8u51")),
      ])
    };
    assert_eq!(pick(all(), &java("java-runtime-beta", 17)).unwrap().version.name, "17.0.15");
    assert_eq!(pick(all(), &java("java-runtime-delta", 21)).unwrap().version.name, "21.0.7");
    assert_eq!(pick(all(), &java("jre-legacy", 8)).unwrap().version.name, "8u51");
    assert!(pick(all(), &java("java-runtime-alpha", 16)).is_none());
  }
}
