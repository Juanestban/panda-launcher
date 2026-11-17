//! Mojang version metadata: manifest, version JSON, OS rules, and installing a version.

use crate::download::{self, FileSpec};
use crate::paths::Dirs;
use crate::{Error, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

const MANIFEST: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const RESOURCES: &str = "https://resources.download.minecraft.net";

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
  pub latest: Latest,
  pub versions: Vec<ManifestEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Latest {
  pub release: String,
  pub snapshot: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestEntry {
  pub id: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub url: String,
  pub sha1: String,
}

impl Manifest {
  pub fn find(&self, id: &str) -> Result<&ManifestEntry> {
    self.versions.iter().find(|v| v.id == id).ok_or_else(|| Error::UnknownVersion(id.into()))
  }
}

pub async fn manifest(http: &Client) -> Result<Manifest> {
  download::get_json(http, MANIFEST).await
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
  pub id: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub main_class: String,
  /// Missing on versions before 1.13 (`minecraftArguments`), which Fabric does not support anyway.
  pub arguments: Option<Arguments>,
  pub asset_index: AssetIndex,
  pub downloads: Downloads,
  pub libraries: Vec<Library>,
  pub java_version: JavaVersion,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Arguments {
  pub game: Vec<Arg>,
  pub jvm: Vec<Arg>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Arg {
  Plain(String),
  Ruled { rules: Vec<Rule>, value: OneOrMany },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany {
  One(String),
  Many(Vec<String>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
  pub action: Action,
  pub os: Option<OsRule>,
  pub features: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
  Allow,
  Disallow,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OsRule {
  pub name: Option<String>,
  pub arch: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetIndex {
  pub id: String,
  pub sha1: String,
  pub size: u64,
  pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Downloads {
  pub client: Artifact,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artifact {
  pub path: Option<String>,
  pub sha1: String,
  pub size: u64,
  pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Library {
  pub name: String,
  pub downloads: Option<LibraryDownloads>,
  #[serde(default)]
  pub rules: Vec<Rule>,
  /// Pre-1.19 natives: OS name → classifier key in `downloads.classifiers`.
  pub natives: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LibraryDownloads {
  pub artifact: Option<Artifact>,
  #[serde(default)]
  pub classifiers: HashMap<String, Artifact>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
  pub component: String,
  pub major_version: u32,
}

/// Mojang's OS names.
pub fn os_name() -> &'static str {
  if cfg!(windows) {
    "windows"
  } else if cfg!(target_os = "macos") {
    "osx"
  } else {
    "linux"
  }
}

impl Rule {
  fn applies(&self) -> bool {
    // We enable no optional features (demo, custom resolution, quick play).
    if self.features.is_some() {
      return false;
    }
    let Some(os) = &self.os else { return true };
    // Mojang only writes `arch: "x86"`, meaning a 32-bit OS.
    let arch = if cfg!(target_pointer_width = "32") { "x86" } else { std::env::consts::ARCH };
    os.name.as_deref().is_none_or(|n| n == os_name()) && os.arch.as_deref().is_none_or(|a| a == arch)
  }
}

/// No rules → allowed; otherwise the last rule that applies decides (default: disallowed).
pub fn allowed(rules: &[Rule]) -> bool {
  rules.is_empty() || rules.iter().rev().find(|r| r.applies()).is_some_and(|r| r.action == Action::Allow)
}

impl Library {
  /// The pre-1.19 natives jar for this OS, if any.
  fn native(&self) -> Option<&Artifact> {
    let key = self.natives.as_ref()?.get(os_name())?.replace("${arch}", "64");
    self.downloads.as_ref()?.classifiers.get(&key)
  }
}

impl Version {
  pub fn json_path(dirs: &Dirs, id: &str) -> PathBuf {
    dirs.version(id).join(format!("{id}.json"))
  }

  pub fn client_jar(&self, dirs: &Dirs) -> PathBuf {
    dirs.version(&self.id).join(format!("{}.jar", self.id))
  }

  /// Read an already-installed version (works offline).
  pub fn load(dirs: &Dirs, id: &str) -> Result<Self> {
    Ok(serde_json::from_slice(&std::fs::read(Self::json_path(dirs, id))?)?)
  }

  /// Jars on the classpath for this OS, client jar last.
  pub fn classpath(&self, dirs: &Dirs) -> Vec<PathBuf> {
    let libs = self.libraries.iter().filter(|l| allowed(&l.rules));
    let artifacts = libs.filter_map(|l| l.downloads.as_ref()?.artifact.as_ref()?.path.as_ref());
    artifacts.map(|p| dirs.libraries().join(p)).chain([self.client_jar(dirs)]).collect()
  }
}

/// Download everything a version needs to start; `progress(done, total)` counts files.
pub async fn install(http: &Client, dirs: &Dirs, entry: &ManifestEntry, progress: impl Fn(usize, usize)) -> Result<Version> {
  let json = FileSpec { url: entry.url.clone(), path: Version::json_path(dirs, &entry.id), sha1: entry.sha1.clone(), size: None };
  let version: Version = download::json(http, &json).await?;
  if version.arguments.is_none() {
    return Err(Error::UnsupportedVersion(version.id));
  }

  let lib = |a: &Artifact| FileSpec {
    url: a.url.clone(),
    path: dirs.libraries().join(a.path.as_deref().unwrap_or_default()),
    sha1: a.sha1.clone(),
    size: Some(a.size),
  };
  let client = &version.downloads.client;
  let mut files = vec![FileSpec { url: client.url.clone(), path: version.client_jar(dirs), sha1: client.sha1.clone(), size: Some(client.size) }];
  let allowed_libs: Vec<&Library> = version.libraries.iter().filter(|l| allowed(&l.rules)).collect();
  files.extend(allowed_libs.iter().filter_map(|l| l.downloads.as_ref()?.artifact.as_ref()).map(lib));
  let natives: Vec<FileSpec> = allowed_libs.iter().filter_map(|l| l.native()).map(lib).collect();
  files.extend(natives.iter().cloned());

  let index = &version.asset_index;
  let index_file = FileSpec {
    url: index.url.clone(),
    path: dirs.assets().join("indexes").join(format!("{}.json", index.id)),
    sha1: index.sha1.clone(),
    size: Some(index.size),
  };
  #[derive(Deserialize)]
  struct Objects {
    objects: HashMap<String, Object>,
  }
  #[derive(Deserialize)]
  struct Object {
    hash: String,
    size: u64,
  }
  let objects: Objects = download::json(http, &index_file).await?;
  files.extend(objects.objects.into_values().map(|o| {
    let prefix = &o.hash[..2];
    FileSpec {
      url: format!("{RESOURCES}/{prefix}/{}", o.hash),
      path: dirs.assets().join("objects").join(prefix).join(&o.hash),
      sha1: o.hash.clone(),
      size: Some(o.size),
    }
  }));

  download::fetch_all(http, &files, progress).await?;
  for jar in &natives {
    extract_natives(&jar.path, &dirs.natives(&version.id))?;
  }
  Ok(version)
}

/// Pre-1.19 natives come as jars that must be unpacked next to each other.
fn extract_natives(jar: &std::path::Path, into: &std::path::Path) -> Result<()> {
  let mut zip = zip::ZipArchive::new(std::fs::File::open(jar)?)?;
  for i in 0..zip.len() {
    let mut entry = zip.by_index(i)?;
    // `enclosed_name` rejects `..` paths from a malicious archive.
    let Some(name) = entry.enclosed_name() else { continue };
    if entry.is_dir() || name.starts_with("META-INF") {
      continue;
    }
    let out = into.join(name);
    std::fs::create_dir_all(out.parent().unwrap_or(into))?;
    std::io::copy(&mut entry, &mut std::fs::File::create(out)?)?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn rules(json: &str) -> Vec<Rule> {
    serde_json::from_str(json).unwrap()
  }

  #[test]
  fn rules_follow_the_last_matching_entry() {
    assert!(allowed(&[]));
    let other_os = if os_name() == "osx" { "windows" } else { "osx" };
    assert!(allowed(&rules(&format!(r#"[{{"action":"allow","os":{{"name":"{}"}}}}]"#, os_name()))));
    assert!(!allowed(&rules(&format!(r#"[{{"action":"allow","os":{{"name":"{other_os}"}}}}]"#))));
    assert!(!allowed(&rules(&format!(r#"[{{"action":"allow"}},{{"action":"disallow","os":{{"name":"{}"}}}}]"#, os_name()))));
    assert!(allowed(&rules(&format!(r#"[{{"action":"allow"}},{{"action":"disallow","os":{{"name":"{other_os}"}}}}]"#))));
    assert!(!allowed(&rules(r#"[{"action":"allow","features":{"is_demo_user":true}}]"#)));
    assert!(!allowed(&rules(r#"[{"action":"allow","os":{"arch":"x86"}}]"#)));
  }

  #[test]
  fn parses_mixed_argument_list() {
    let a: Arguments = serde_json::from_str(
      r#"{"game":["--username","${auth_player_name}",{"rules":[{"action":"allow","features":{"is_demo_user":true}}],"value":"--demo"}],
          "jvm":[{"rules":[{"action":"allow","os":{"name":"osx"}}],"value":["-XstartOnFirstThread"]},"-cp","${classpath}"]}"#,
    )
    .unwrap();
    assert_eq!(a.game.len(), 3);
    assert!(matches!(&a.jvm[0], Arg::Ruled { value: OneOrMany::Many(v), .. } if v[0] == "-XstartOnFirstThread"));
  }

  /// Downloads a real version and Java (~700 MB). Run with:
  /// `PANDA_TEST_DIR=/some/dir cargo test -p panda-core -- --ignored --nocapture`
  #[tokio::test]
  #[ignore]
  async fn installs_a_real_version() {
    let dirs = Dirs { root: std::env::var_os("PANDA_TEST_DIR").expect("set PANDA_TEST_DIR").into() };
    let http = crate::net::client();
    let manifest = manifest(&http).await.unwrap();
    for id in ["1.21.10", "1.18.2"] {
      let v = install(&http, &dirs, manifest.find(id).unwrap(), |d, t| {
        if d == t {
          println!("{id}: {t} files");
        }
      })
      .await
      .unwrap();
      let java = crate::java::ensure(&http, &dirs, &v.java_version, |_, _| {}).await.unwrap();
      let out = std::process::Command::new(&java).arg("-version").output().unwrap();
      assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
      assert!(v.classpath(&dirs).iter().all(|p| p.exists()), "classpath files missing");
    }
    assert!(std::fs::read_dir(dirs.natives("1.18.2")).unwrap().next().is_some(), "1.18.2 natives not extracted");
  }
}
