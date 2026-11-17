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

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
struct ModVersion {
  id: String,
  project_id: String,
  date_published: String,
  files: Vec<ModFile>,
  dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Deserialize)]
struct ModFile {
  url: String,
  filename: String,
  primary: bool,
  hashes: Hashes,
}

#[derive(Debug, Clone, Deserialize)]
struct Hashes {
  sha512: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Dependency {
  project_id: Option<String>,
  version_id: Option<String>,
  dependency_type: String,
}

#[derive(Debug, Deserialize)]
struct Project {
  title: String,
}

async fn title(http: &Client, project: &str) -> String {
  get::<Project>(http, &format!("{API}/project/{project}"), &[]).await.map_or_else(|_| project.into(), |p| p.title)
}

/// Newest version of `project` built for Fabric on `game`.
async fn latest_version(http: &Client, project: &str, game: &str) -> Result<ModVersion> {
  let q = [("loaders", r#"["fabric"]"#.into()), ("game_versions", format!(r#"["{game}"]"#))];
  let versions: Vec<ModVersion> = get(http, &format!("{API}/project/{project}/version"), &q).await?;
  // ISO-8601 timestamps sort as text.
  match versions.into_iter().max_by(|a, b| a.date_published.cmp(&b.date_published)) {
    Some(v) => Ok(v),
    None => Err(Error::NoCompatibleMod(title(http, project).await, game.into())),
  }
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

/// Install `project` (id or slug) and its required dependencies into the `game` profile.
/// Returns the titles of the mods that were added.
pub async fn install(http: &Client, dirs: &Dirs, game: &str, project: &str) -> Result<Vec<String>> {
  let mut list = list(dirs, game)?;
  // (project, needed by, pinned version)
  let mut queue = VecDeque::from([(project.to_string(), None::<String>, None::<String>)]);
  let mut planned: Vec<ModVersion> = Vec::new();
  let mut seen = HashSet::new();

  while let Some((project, needed_by, pinned)) = queue.pop_front() {
    let version = match pinned {
      Some(id) => get::<ModVersion>(http, &format!("{API}/version/{id}"), &[]).await?,
      None => latest_version(http, &project, game).await?,
    };
    let id = version.project_id.clone();
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
    download_verified::<Sha512>(http, &file.url, &mods_dir(dirs, game).join(&file.filename), &file.hashes.sha512).await?;
    let title = title(http, &version.project_id).await;
    let required_by = planned
      .iter()
      .filter(|p| p.dependencies.iter().any(|d| d.dependency_type == "required" && d.project_id.as_deref() == Some(&version.project_id)))
      .map(|p| p.project_id.clone())
      .collect();
    list.mods.push(InstalledMod {
      project_id: version.project_id.clone(),
      version_id: version.id.clone(),
      title: title.clone(),
      file: file.filename.clone(),
      sha512: file.hashes.sha512.clone(),
      required_by,
      explicit: roots.contains(&version.project_id),
      incompatible: incompatible_of(version),
    });
    added.push(title);
  }
  save(dirs, game, &list)?;
  Ok(added)
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
    match std::fs::remove_file(mods_dir(dirs, game).join(&m.file)) {
      Ok(()) => {}
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
      Err(e) => return Err(e.into()),
    }
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

    let added = install(&http, &dirs, "1.21.10", &menu.project_id).await.unwrap();
    println!("added: {added:?}");
    let mods = list(&dirs, "1.21.10").unwrap().mods;
    assert!(mods.len() >= 2, "dependencies were installed");
    assert!(mods.iter().all(|m| mods_dir(&dirs, "1.21.10").join(&m.file).exists()));
    assert!(mods.iter().filter(|m| !m.explicit).all(|m| m.required_by.contains(&menu.project_id)));

    // Installing again changes nothing.
    assert!(install(&http, &dirs, "1.21.10", "modmenu").await.unwrap().is_empty());

    let removed = uninstall(&dirs, "1.21.10", &menu.project_id, false).unwrap();
    assert_eq!(removed.len(), mods.len(), "orphaned dependencies removed too");
    assert!(list(&dirs, "1.21.10").unwrap().mods.is_empty());
    std::fs::remove_dir_all(&dirs.root).unwrap();
  }
}
