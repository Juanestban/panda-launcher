//! Modrinth mods for a Fabric profile: search, install with dependencies, uninstall (ADR 0007).

use crate::download::download_verified;
use crate::fabric::profile_name;
use crate::paths::Dirs;
use crate::{Error, Result};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;
use std::time::Duration;

const API: &str = "https://api.modrinth.com/v2";
const RECORD: &str = "panda-mods.json";
const PAGE: usize = 20;

/// GET with the Modrinth rate limit (300 req/min): on 429 wait for `X-Ratelimit-Reset`, at most 3 times.
async fn get<T: DeserializeOwned>(http: &Client, url: &str, query: &[(&str, String)]) -> Result<T> {
  for _ in 0..3 {
    let r = http.get(url).query(query).send().await?;
    if r.status() != StatusCode::TOO_MANY_REQUESTS {
      return Ok(r.error_for_status()?.json().await?);
    }
    let wait = r.headers().get("x-ratelimit-reset").and_then(|v| v.to_str().ok()?.parse().ok()).unwrap_or(5);
    tokio::time::sleep(Duration::from_secs(wait)).await;
  }
  Err(Error::RateLimited)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchPage {
  pub hits: Vec<SearchHit>,
  pub total_hits: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SearchHit {
  pub project_id: String,
  pub slug: String,
  pub title: String,
  pub description: String,
  pub author: String,
  pub downloads: u64,
  pub icon_url: Option<String>,
}

/// Fabric mods that have a build for `game`, 20 per page.
pub async fn search(http: &Client, game: &str, query: &str, page: usize) -> Result<SearchPage> {
  let facets = serde_json::json!([[format!("versions:{game}")], ["categories:fabric"], ["project_type:mod"]]).to_string();
  let q = [("query", query.into()), ("facets", facets), ("limit", PAGE.to_string()), ("offset", (page * PAGE).to_string())];
  get(http, &format!("{API}/search"), &q).await
}

/// One published build of a mod.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ModVersion {
  pub id: String,
  pub project_id: String,
  /// Display name, e.g. "Sodium 0.6.0 for Fabric".
  pub name: String,
  pub version_number: String,
  /// `release`, `beta` or `alpha`.
  pub version_type: String,
  pub date_published: String,
  pub downloads: u64,
  #[serde(default)]
  pub game_versions: Vec<String>,
  files: Vec<ModFile>,
  dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ModFile {
  url: String,
  filename: String,
  primary: bool,
  hashes: Hashes,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Hashes {
  sha512: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Dependency {
  project_id: Option<String>,
  version_id: Option<String>,
  dependency_type: String,
}

/// Everything the mod page shows. Optional fields are often missing on Modrinth.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
  pub id: String,
  pub slug: String,
  pub title: String,
  pub description: String,
  /// Long description, Markdown with some HTML.
  #[serde(default)]
  pub body: String,
  pub icon_url: Option<String>,
  pub downloads: u64,
  pub followers: u64,
  #[serde(default)]
  pub categories: Vec<String>,
  pub license: Option<License>,
  pub source_url: Option<String>,
  pub issues_url: Option<String>,
  pub wiki_url: Option<String>,
  pub discord_url: Option<String>,
  pub updated: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct License {
  pub id: String,
  pub name: String,
}

/// A project by id or slug.
pub async fn project(http: &Client, id: &str) -> Result<Project> {
  get(http, &format!("{API}/project/{id}"), &[]).await
}

async fn title(http: &Client, project: &str) -> String {
  self::project(http, project).await.map_or_else(|_| project.into(), |p| p.title)
}

/// Fabric builds of `project` for `game`, newest first.
pub async fn versions(http: &Client, project: &str, game: &str) -> Result<Vec<ModVersion>> {
  let q = [("loaders", r#"["fabric"]"#.into()), ("game_versions", format!(r#"["{game}"]"#))];
  let mut versions: Vec<ModVersion> = get(http, &format!("{API}/project/{project}/version"), &q).await?;
  // ISO-8601 timestamps sort as text.
  versions.sort_by(|a, b| b.date_published.cmp(&a.date_published));
  Ok(versions)
}

/// Newest version of `project` built for Fabric on `game`.
async fn latest_version(http: &Client, project: &str, game: &str) -> Result<ModVersion> {
  match versions(http, project, game).await?.into_iter().next() {
    Some(v) => Ok(v),
    None => Err(Error::NoCompatibleMod(title(http, project).await, game.into())),
  }
}

/// The project's icon, downloaded once into `cache/icons`. Modrinth serves PNG, JPEG, WebP or GIF.
pub async fn icon(http: &Client, dirs: &Dirs, project_id: &str, url: &str) -> Result<PathBuf> {
  let ext = url.rsplit('.').next().filter(|e| ["png", "jpg", "jpeg", "webp", "gif"].contains(e)).unwrap_or("png");
  // The id is Modrinth's base62; refuse anything else so it can't walk out of the cache folder.
  if project_id.is_empty() || !project_id.chars().all(|c| c.is_ascii_alphanumeric()) {
    return Err(Error::ModNotInstalled(project_id.into()));
  }
  let path = dirs.root.join("cache").join("icons").join(format!("{project_id}.{ext}"));
  if path.exists() {
    return Ok(path);
  }
  let bytes = http.get(url).send().await?.error_for_status()?.bytes().await?;
  tokio::fs::create_dir_all(path.parent().expect("has a parent")).await?;
  let tmp = path.with_extension("part");
  tokio::fs::write(&tmp, &bytes).await?;
  tokio::fs::rename(&tmp, &path).await?;
  Ok(path)
}

/// One mod in a profile's `panda-mods.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstalledMod {
  pub project_id: String,
  pub version_id: String,
  pub title: String,
  pub file: String,
  pub sha512: String,
  /// Projects that need this one. Empty and not `explicit` means it can go.
  pub required_by: Vec<String>,
  /// Installed by the player, not only as a dependency.
  pub explicit: bool,
  /// Projects this version declares it cannot run with.
  pub incompatible: Vec<String>,
  /// Shown in the installed list; absent in records written before icons existed.
  #[serde(default)]
  pub icon_url: Option<String>,
  #[serde(default)]
  pub version_number: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ModList {
  pub mods: Vec<InstalledMod>,
}

impl ModList {
  fn get_mut(&mut self, project: &str) -> Option<&mut InstalledMod> {
    self.mods.iter_mut().find(|m| m.project_id == project)
  }
}

fn mods_dir(dirs: &Dirs, game: &str) -> PathBuf {
  dirs.profile(&profile_name(game)).join("mods")
}

fn record_path(dirs: &Dirs, game: &str) -> PathBuf {
  dirs.profile(&profile_name(game)).join(RECORD)
}

/// The mods installed in a profile.
pub fn list(dirs: &Dirs, game: &str) -> Result<ModList> {
  match std::fs::read(record_path(dirs, game)) {
    Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(ModList::default()),
    Err(e) => Err(e.into()),
  }
}

fn save(dirs: &Dirs, game: &str, list: &ModList) -> Result<()> {
  std::fs::create_dir_all(mods_dir(dirs, game))?;
  std::fs::write(record_path(dirs, game), serde_json::to_vec_pretty(list)?)?;
  Ok(())
}

/// Install `project` (id or slug) and its required dependencies into the `game` profile: the newest build,
/// or `version` when the player picked one (which replaces an installed build of the same mod).
/// Returns the titles of the mods that were added.
pub async fn install(http: &Client, dirs: &Dirs, game: &str, project: &str, version: Option<&str>) -> Result<Vec<String>> {
  let mut list = list(dirs, game)?;
  // (project, needed by, pinned version)
  let mut queue = VecDeque::from([(project.to_string(), None::<String>, version.map(str::to_owned))]);
  let mut planned: Vec<ModVersion> = Vec::new();
  let mut seen = HashSet::new();
  // Who needed the build being replaced, so they still need the new one.
  let mut replaced_required_by = Vec::new();

  while let Some((project, needed_by, pinned)) = queue.pop_front() {
    let is_root = planned.is_empty() && seen.is_empty();
    let version = match pinned {
      Some(id) => get::<ModVersion>(http, &format!("{API}/version/{id}"), &[]).await?,
      None => latest_version(http, &project, game).await?,
    };
    let id = version.project_id.clone();
    if is_root && let Some(pos) = list.mods.iter().position(|m| m.project_id == id && m.version_id != version.id) {
      let old = list.mods.remove(pos);
      remove_file(dirs, game, &old.file)?;
      replaced_required_by = old.required_by;
    }
    if let Some(existing) = list.get_mut(&id) {
      match needed_by {
        Some(parent) if !existing.required_by.contains(&parent) => existing.required_by.push(parent),
        None => existing.explicit = true,
        _ => {}
      }
      continue;
    }
    if !seen.insert(id.clone()) {
      continue;
    }
    for dep in &version.dependencies {
      match (dep.dependency_type.as_str(), &dep.project_id) {
        ("required", Some(p)) => queue.push_back((p.clone(), Some(id.clone()), dep.version_id.clone())),
        ("required", None) => {
          if let Some(v) = &dep.version_id {
            queue.push_back((String::new(), Some(id.clone()), Some(v.clone())));
          }
        }
        _ => {}
      }
    }
    planned.push(version);
  }

  // Refuse conflicts in both directions: new vs installed, and installed vs new.
  let mut present: Vec<(String, Vec<String>)> = list.mods.iter().map(|m| (m.project_id.clone(), m.incompatible.clone())).collect();
  present.extend(planned.iter().map(|v| (v.project_id.clone(), incompatible_of(v))));
  if let Some((a, b)) = find_conflict(&present) {
    return Err(Error::IncompatibleMods(title(http, &a).await, title(http, &b).await));
  }

  let roots: HashSet<String> = planned.first().map(|v| v.project_id.clone()).into_iter().collect();
  let mut added = Vec::new();
  for version in &planned {
    let file =
      version.files.iter().find(|f| f.primary).or(version.files.first()).ok_or(Error::NoCompatibleMod(version.project_id.clone(), game.into()))?;
    download_verified::<Sha512>(http, &file.url, &mods_dir(dirs, game).join(&file.filename), &file.hashes.sha512, |_, _| {}).await?;
    let mut required_by: Vec<String> = planned
      .iter()
      .filter(|p| p.dependencies.iter().any(|d| d.dependency_type == "required" && d.project_id.as_deref() == Some(&version.project_id)))
      .map(|p| p.project_id.clone())
      .collect();
    if roots.contains(&version.project_id) {
      required_by.extend(replaced_required_by.iter().cloned());
    }
    let project = self::project(http, &version.project_id).await.ok();
    let title = project.as_ref().map_or_else(|| version.project_id.clone(), |p| p.title.clone());
    list.mods.push(InstalledMod {
      project_id: version.project_id.clone(),
      version_id: version.id.clone(),
      title: title.clone(),
      file: file.filename.clone(),
      sha512: file.hashes.sha512.clone(),
      required_by,
      explicit: roots.contains(&version.project_id),
      incompatible: incompatible_of(version),
      icon_url: project.and_then(|p| p.icon_url),
      version_number: version.version_number.clone(),
    });
    added.push(title);
  }
  save(dirs, game, &list)?;
  Ok(added)
}

fn remove_file(dirs: &Dirs, game: &str, file: &str) -> Result<()> {
  match std::fs::remove_file(mods_dir(dirs, game).join(file)) {
    Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.into()),
    _ => Ok(()),
  }
}

/// First pair `(a, b)` where mod `a` declares `b` incompatible.
fn find_conflict(present: &[(String, Vec<String>)]) -> Option<(String, String)> {
  present.iter().find_map(|(a, conflicts)| present.iter().find(|(b, _)| conflicts.contains(b)).map(|(b, _)| (a.clone(), b.clone())))
}

fn incompatible_of(v: &ModVersion) -> Vec<String> {
  v.dependencies.iter().filter(|d| d.dependency_type == "incompatible").filter_map(|d| d.project_id.clone()).collect()
}

/// Remove a mod, plus dependencies nothing else needs any more. Returns the removed titles.
/// Without `force`, refuses while other installed mods require it.
pub fn uninstall(dirs: &Dirs, game: &str, project: &str, force: bool) -> Result<Vec<String>> {
  let mut list = list(dirs, game)?;
  let target = list.mods.iter().find(|m| m.project_id == project).ok_or_else(|| Error::ModNotInstalled(project.into()))?;
  if !force && !target.required_by.is_empty() {
    let needers = list.mods.iter().filter(|m| target.required_by.contains(&m.project_id)).map(|m| m.title.clone()).collect();
    return Err(Error::ModStillRequired(target.title.clone(), needers));
  }

  let mut removed = Vec::new();
  let mut pending = vec![project.to_string()];
  while let Some(id) = pending.pop() {
    let Some(pos) = list.mods.iter().position(|m| m.project_id == id) else { continue };
    let m = list.mods.remove(pos);
    remove_file(dirs, game, &m.file)?;
    removed.push(m.title);
    for other in &mut list.mods {
      other.required_by.retain(|p| p != &id);
      if !other.explicit && other.required_by.is_empty() {
        pending.push(other.project_id.clone());
      }
    }
  }
  save(dirs, game, &list)?;
  Ok(removed)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn temp_dirs(name: &str) -> Dirs {
    let root = std::env::temp_dir().join(format!("panda-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    Dirs { root }
  }

  fn installed(id: &str, required_by: &[&str], explicit: bool) -> InstalledMod {
    InstalledMod {
      project_id: id.into(),
      version_id: format!("v-{id}"),
      title: id.to_uppercase(),
      file: format!("{id}.jar"),
      sha512: String::new(),
      required_by: required_by.iter().map(|s| s.to_string()).collect(),
      explicit,
      incompatible: vec![],
      icon_url: None,
      version_number: "1.0".into(),
    }
  }

  #[test]
  fn uninstall_removes_orphaned_dependencies_only() {
    let dirs = temp_dirs("uninstall");
    let start = ModList {
      mods: vec![
        installed("menu", &[], true),
        installed("api", &["menu", "other"], false),
        installed("placeholder", &["menu"], false),
        installed("other", &[], true),
      ],
    };
    save(&dirs, "1.21.10", &start).unwrap();
    for m in &start.mods {
      std::fs::write(mods_dir(&dirs, "1.21.10").join(&m.file), "").unwrap();
    }

    let err = uninstall(&dirs, "1.21.10", "api", false).unwrap_err();
    assert!(matches!(err, Error::ModStillRequired(_, ref by) if by.len() == 2));

    let mut removed = uninstall(&dirs, "1.21.10", "menu", false).unwrap();
    removed.sort();
    assert_eq!(removed, ["MENU", "PLACEHOLDER"]);
    let left: Vec<_> = list(&dirs, "1.21.10").unwrap().mods.into_iter().map(|m| (m.project_id, m.required_by)).collect();
    assert_eq!(left, [("api".to_string(), vec!["other".to_string()]), ("other".to_string(), vec![])]);
    assert!(!mods_dir(&dirs, "1.21.10").join("menu.jar").exists());
    assert!(mods_dir(&dirs, "1.21.10").join("api.jar").exists());
    std::fs::remove_dir_all(&dirs.root).unwrap();
  }

  #[test]
  fn conflicts_are_found_in_either_direction() {
    let mods = |list: &[(&str, &[&str])]| -> Vec<(String, Vec<String>)> {
      list.iter().map(|(id, c)| (id.to_string(), c.iter().map(|s| s.to_string()).collect())).collect()
    };
    assert_eq!(find_conflict(&mods(&[("sodium", &[]), ("optifabric", &["sodium"])])), Some(("optifabric".into(), "sodium".into())));
    assert_eq!(find_conflict(&mods(&[("a", &["z"]), ("b", &[])])), None);
  }

  /// Installs Mod Menu (needs Fabric API + Placeholder API) from Modrinth, then removes it.
  #[tokio::test]
  #[ignore]
  async fn installs_and_removes_a_real_mod() {
    let dirs = temp_dirs("modrinth");
    let http = crate::net::client();
    let hits = search(&http, "1.21.10", "mod menu", 0).await.unwrap();
    let menu = hits.hits.iter().find(|h| h.slug == "modmenu").expect("Mod Menu in search");

    let added = install(&http, &dirs, "1.21.10", &menu.project_id, None).await.unwrap();
    println!("added: {added:?}");
    let mods = list(&dirs, "1.21.10").unwrap().mods;
    assert!(mods.len() >= 2, "dependencies were installed");
    assert!(mods.iter().all(|m| mods_dir(&dirs, "1.21.10").join(&m.file).exists()));
    assert!(mods.iter().filter(|m| !m.explicit).all(|m| m.required_by.contains(&menu.project_id)));

    // Installing again changes nothing.
    assert!(install(&http, &dirs, "1.21.10", "modmenu", None).await.unwrap().is_empty());

    // Picking an older build replaces the installed one and keeps the dependencies.
    let builds = versions(&http, &menu.project_id, "1.21.10").await.unwrap();
    assert!(builds.windows(2).all(|w| w[0].date_published >= w[1].date_published), "newest first");
    if let Some(older) = builds.get(1) {
      install(&http, &dirs, "1.21.10", &menu.project_id, Some(&older.id)).await.unwrap();
      let now = list(&dirs, "1.21.10").unwrap().mods;
      let entry = now.iter().find(|m| m.project_id == menu.project_id).unwrap();
      assert_eq!(entry.version_id, older.id);
      assert!(entry.explicit && entry.icon_url.is_some());
      assert_eq!(now.len(), mods.len());
      assert_eq!(std::fs::read_dir(mods_dir(&dirs, "1.21.10")).unwrap().count(), mods.len(), "old jar removed");
    }
    let page = project(&http, "modmenu").await.unwrap();
    assert!(!page.body.is_empty());
    let icon_path = icon(&http, &dirs, &page.id, page.icon_url.as_deref().unwrap()).await.unwrap();
    assert!(icon_path.metadata().unwrap().len() > 100);

    let removed = uninstall(&dirs, "1.21.10", &menu.project_id, false).unwrap();
    assert_eq!(removed.len(), mods.len(), "orphaned dependencies removed too");
    assert!(list(&dirs, "1.21.10").unwrap().mods.is_empty());
    std::fs::remove_dir_all(&dirs.root).unwrap();
  }
}
