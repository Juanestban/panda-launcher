//! Fabric loader on top of a vanilla version, and per-version profiles (ADR 0005, 0006).

use crate::download::{self, FileSpec};
use crate::paths::Dirs;
use crate::version::{Arg, Artifact, Library, LibraryDownloads, Version};
use crate::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const META: &str = "https://meta.fabricmc.net/v2";
const PROFILE_FILE: &str = "panda-profile.json";

/// `1.21.10` → `1.21.10-fabric`, the profile folder name.
pub fn profile_name(game: &str) -> String {
  format!("{game}-fabric")
}

#[derive(Debug, Deserialize)]
struct GameVersion {
  version: String,
  stable: bool,
}

/// Release versions Fabric supports, newest first (for the version picker).
pub async fn game_versions(http: &Client) -> Result<Vec<String>> {
  let all: Vec<GameVersion> = download::get_json(http, &format!("{META}/versions/game")).await?;
  Ok(all.into_iter().filter(|v| v.stable).map(|v| v.version).collect())
}

#[derive(Debug, Deserialize)]
struct LoaderEntry {
  loader: Loader,
}
#[derive(Debug, Deserialize)]
struct Loader {
  version: String,
  stable: bool,
}

async fn latest_loader(http: &Client, game: &str) -> Result<String> {
  let response = http.get(format!("{META}/versions/loader/{game}")).send().await?;
  // Fabric answers 400 or an empty list for versions it does not support.
  if response.status().is_client_error() {
    return Err(Error::NoFabric(game.into()));
  }
  let loaders: Vec<LoaderEntry> = response.error_for_status()?.json().await?;
  loaders.into_iter().find(|l| l.loader.stable).map(|l| l.loader.version).ok_or_else(|| Error::NoFabric(game.into()))
}

/// Fabric's launcher profile: a partial version JSON that inherits from vanilla.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricProfile {
  pub id: String,
  pub inherits_from: String,
  pub main_class: String,
  #[serde(default)]
  pub arguments: Option<FabricArguments>,
  pub libraries: Vec<MavenLibrary>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FabricArguments {
  #[serde(default)]
  pub game: Vec<String>,
  #[serde(default)]
  pub jvm: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavenLibrary {
  /// `group:artifact:version[:classifier]`
  pub name: String,
  /// Repository base, e.g. `https://maven.fabricmc.net/`.
  pub url: String,
  pub sha1: Option<String>,
  pub size: Option<u64>,
}

/// What a profile folder runs, so it can start offline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileInfo {
  pub minecraft: String,
  pub fabric_loader: String,
}

/// `net.fabricmc:fabric-loader:0.20.1` → `net/fabricmc/fabric-loader/0.20.1/fabric-loader-0.20.1.jar`.
pub fn maven_path(name: &str) -> Option<String> {
  let mut parts = name.split(':');
  let (group, artifact, version) = (parts.next()?, parts.next()?, parts.next()?);
  let classifier = parts.next().map(|c| format!("-{c}")).unwrap_or_default();
  Some(format!("{}/{artifact}/{version}/{artifact}-{version}{classifier}.jar", group.replace('.', "/")))
}

fn fabric_json(dirs: &Dirs, id: &str) -> std::path::PathBuf {
  dirs.version(id).join(format!("{id}.json"))
}

/// `group:artifact:version[:classifier]` without the version, to spot the same library twice.
fn without_version(name: &str) -> String {
  let parts: Vec<&str> = name.split(':').collect();
  [&parts[..parts.len().min(2)], parts.get(3..).unwrap_or_default()].concat().join(":")
}

/// Vanilla plus Fabric: Fabric libraries first, Fabric main class, both argument lists.
/// The `id` stays vanilla so the client jar and natives paths keep working.
pub fn merge(mut vanilla: Version, fabric: &FabricProfile) -> Version {
  let libs: Vec<Library> = fabric
    .libraries
    .iter()
    .filter_map(|l| {
      let path = maven_path(&l.name)?;
      let artifact =
        Artifact { url: format!("{}{path}", l.url), path: Some(path), sha1: l.sha1.clone().unwrap_or_default(), size: l.size.unwrap_or_default() };
      let downloads = LibraryDownloads { artifact: Some(artifact), classifiers: Default::default() };
      Some(Library { name: l.name.clone(), downloads: Some(downloads), rules: vec![], natives: None })
    })
    .collect();
  // Like the official launcher, the child profile wins: vanilla's ASM 9.6 next to Fabric's 9.10 breaks Mixin.
  let fabric_names: std::collections::HashSet<String> = libs.iter().map(|l| without_version(&l.name)).collect();
  vanilla.libraries.retain(|l| !fabric_names.contains(&without_version(&l.name)));
  vanilla.libraries = libs.into_iter().chain(vanilla.libraries).collect();
  vanilla.main_class = fabric.main_class.clone();
  if let (Some(args), Some(extra)) = (vanilla.arguments.as_mut(), &fabric.arguments) {
    args.jvm.extend(extra.jvm.iter().cloned().map(Arg::Plain));
    args.game.extend(extra.game.iter().cloned().map(Arg::Plain));
  }
  vanilla
}

/// Add the latest stable Fabric loader to an installed vanilla version and create its profile folder.
pub async fn install(http: &Client, dirs: &Dirs, vanilla: Version, progress: impl Fn(usize, usize)) -> Result<Version> {
  let game = vanilla.id.clone();
  let loader = latest_loader(http, &game).await?;
  let fabric: FabricProfile = download::get_json(http, &format!("{META}/versions/loader/{game}/{loader}/profile/json")).await?;

  let mut files = Vec::new();
  for l in &fabric.libraries {
    let Some(path) = maven_path(&l.name) else { continue };
    let url = format!("{}{path}", l.url);
    // The loader and intermediary jars come without a hash in the profile; Maven publishes one next to the jar.
    let sha1 = match &l.sha1 {
      Some(s) => s.clone(),
      None => http.get(format!("{url}.sha1")).send().await?.error_for_status()?.text().await?.trim().to_string(),
    };
    files.push(FileSpec { url, path: dirs.libraries().join(path), sha1, size: l.size });
  }
  download::fetch_all(http, &files, progress).await?;

  let json = fabric_json(dirs, &fabric.id);
  std::fs::create_dir_all(json.parent().unwrap_or(&dirs.root))?;
  std::fs::write(&json, serde_json::to_vec_pretty(&fabric)?)?;
  let profile = dirs.profile(&profile_name(&game));
  std::fs::create_dir_all(profile.join("mods"))?;
  let info = ProfileInfo { minecraft: game, fabric_loader: loader };
  std::fs::write(profile.join(PROFILE_FILE), serde_json::to_vec_pretty(&info)?)?;
  Ok(merge(vanilla, &fabric))
}

/// The merged version of an installed profile, without network (offline play).
pub fn load(dirs: &Dirs, game: &str) -> Result<Version> {
  let info: ProfileInfo = serde_json::from_slice(&std::fs::read(dirs.profile(&profile_name(game)).join(PROFILE_FILE))?)?;
  let id = format!("fabric-loader-{}-{}", info.fabric_loader, info.minecraft);
  let fabric: FabricProfile = serde_json::from_slice(&std::fs::read(fabric_json(dirs, &id))?)?;
  Ok(merge(Version::load(dirs, game)?, &fabric))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maven_names_become_repository_paths() {
    assert_eq!(maven_path("net.fabricmc:fabric-loader:0.20.1").unwrap(), "net/fabricmc/fabric-loader/0.20.1/fabric-loader-0.20.1.jar");
    assert_eq!(maven_path("org.lwjgl:lwjgl:3.3.3:natives-windows").unwrap(), "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows.jar");
    assert!(maven_path("broken").is_none());
  }

  #[test]
  fn merge_puts_fabric_first_and_keeps_vanilla_id() {
    let vanilla: Version = serde_json::from_str(
      r#"{"id":"1.21.10","type":"release","mainClass":"net.minecraft.client.main.Main",
          "arguments":{"game":["--username","x"],"jvm":["-cp","${classpath}"]},
          "assetIndex":{"id":"27","sha1":"x","size":1,"url":"u"},
          "downloads":{"client":{"sha1":"x","size":1,"url":"u"}},
          "libraries":[{"name":"com.mojang:brigadier:1","downloads":{"artifact":{"path":"b.jar","sha1":"x","size":1,"url":"u"}}},
                       {"name":"org.ow2.asm:asm:9.6","downloads":{"artifact":{"path":"a.jar","sha1":"x","size":1,"url":"u"}}}],
          "javaVersion":{"component":"java-runtime-delta","majorVersion":21}}"#,
    )
    .unwrap();
    let fabric: FabricProfile = serde_json::from_str(
      r#"{"id":"fabric-loader-0.20.1-1.21.10","inheritsFrom":"1.21.10","mainClass":"net.fabricmc.loader.impl.launch.knot.KnotClient",
          "arguments":{"game":[],"jvm":["-DFabricMcEmu= net.minecraft.client.main.Main "]},
          "libraries":[{"name":"net.fabricmc:fabric-loader:0.20.1","url":"https://maven.fabricmc.net/"},
                       {"name":"org.ow2.asm:asm:9.10.1","url":"https://maven.fabricmc.net/"}]}"#,
    )
    .unwrap();
    let merged = merge(vanilla, &fabric);
    assert_eq!(merged.id, "1.21.10");
    assert_eq!(merged.main_class, "net.fabricmc.loader.impl.launch.knot.KnotClient");
    assert_eq!(merged.libraries[0].name, "net.fabricmc:fabric-loader:0.20.1");
    let names: Vec<&str> = merged.libraries.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(names, ["net.fabricmc:fabric-loader:0.20.1", "org.ow2.asm:asm:9.10.1", "com.mojang:brigadier:1"]);
    assert_eq!(without_version("org.lwjgl:lwjgl:3.3.3:natives-windows"), "org.lwjgl:lwjgl:natives-windows");
    assert!(matches!(merged.arguments.unwrap().jvm.last(), Some(Arg::Plain(s)) if s.starts_with("-DFabricMcEmu")));
  }

  /// Needs the vanilla install from `version::tests::installs_a_real_version` in the same PANDA_TEST_DIR.
  #[tokio::test]
  #[ignore]
  async fn installs_real_fabric_profiles() {
    let dirs = Dirs { root: std::env::var_os("PANDA_TEST_DIR").expect("set PANDA_TEST_DIR").into() };
    let http = crate::net::client();
    for game in ["1.21.10", "1.18.2"] {
      let merged = install(&http, &dirs, Version::load(&dirs, game).unwrap(), |_, _| {}).await.unwrap();
      assert!(merged.classpath(&dirs).iter().all(|p| p.exists()), "{game}: classpath files missing");
      let offline = load(&dirs, game).unwrap();
      assert_eq!(offline.classpath(&dirs), merged.classpath(&dirs));
      assert!(dirs.profile(&profile_name(game)).join("mods").is_dir());
    }
    assert!(matches!(latest_loader(&http, "1.13.2").await, Err(Error::NoFabric(_))));
    assert!(game_versions(&http).await.unwrap().contains(&"1.21.10".to_string()));
  }
}
