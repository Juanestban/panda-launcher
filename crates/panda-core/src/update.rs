//! Self-update from this repository's GitHub Releases (ADR 0010), based on the archi-games updater.
//!
//! A release is offered when it has the installer for this copy and `SHA256SUMS.txt`, and every download
//! is checked against that SHA-256. Like archi-games, this needs no keys: trust comes from GitHub over HTTPS.
//! Optional hardening: a build made with `PANDA_UPDATE_PUBLIC_KEY` also requires `<installer>.sig`, an
//! ed25519 signature over the digest, which proves the file came from our CI and not from whoever
//! controls the GitHub account.

use crate::download::download_verified;
use crate::net::Client;
use crate::{Error, Result};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::Deserialize;
use sha2::Sha256;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const REPO: &str = "juanestban/panda-launcher";
/// Passed to the new version after updating: it waits for the old one to exit.
pub const AFTER_UPDATE_ARG: &str = "--after-update";
const SUMS: &str = "SHA256SUMS.txt";
/// Hex ed25519 public key, optionally injected by the release build; then releases must be signed.
const PUBLIC_KEY: Option<&str> = option_env!("PANDA_UPDATE_PUBLIC_KEY");

/// CI exports an unset secret as an empty string, which means "no key" too.
fn public_key() -> Option<&'static str> {
  PUBLIC_KEY.filter(|k| !k.trim().is_empty())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u64, u64, u64);

impl Version {
  pub fn current() -> Self {
    Self::parse(env!("CARGO_PKG_VERSION")).expect("the crate version is x.y.z")
  }

  /// `x.y.z` or `vx.y.z`. Pre-releases (`-beta`…) are never offered.
  pub fn parse(text: &str) -> Option<Self> {
    let mut parts = text.strip_prefix('v').unwrap_or(text).split('.').map(|p| p.parse::<u64>().ok());
    let version = Self(parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(version)
  }
}

impl fmt::Display for Version {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}.{}.{}", self.0, self.1, self.2)
  }
}

/// How this copy was installed, which decides the asset to fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Install {
  /// Per-user MSI in `%LOCALAPPDATA%\Programs\Panda Launcher`: updated by a newer MSI.
  Msi,
  /// A standalone exe anywhere else: replaced in place.
  Portable,
}

impl Install {
  /// `None` where we ship no build (macOS, Linux): nothing to update to.
  pub fn detect() -> Option<Self> {
    if !cfg!(windows) {
      return None;
    }
    let msi_dir = std::env::var_os("LOCALAPPDATA").map(|d| Path::new(&d).join("Programs").join("Panda Launcher"));
    let exe = std::env::current_exe().ok()?;
    match msi_dir {
      Some(dir) if exe.to_string_lossy().to_lowercase().starts_with(&dir.to_string_lossy().to_lowercase()) => Some(Self::Msi),
      _ => Some(Self::Portable),
    }
  }

  fn suffix(self) -> &'static str {
    match self {
      Self::Msi => "-x64.msi",
      Self::Portable => "-x64-portable.exe",
    }
  }
}

/// Whether this build can update itself at all.
pub fn enabled() -> bool {
  Install::detect().is_some()
}

#[derive(Deserialize)]
struct GhRelease {
  tag_name: String,
  html_url: String,
  #[serde(default)]
  body: Option<String>,
  #[serde(default)]
  assets: Vec<GhAsset>,
}

#[derive(Deserialize)]
struct GhAsset {
  name: String,
  browser_download_url: String,
}

/// A newer release with everything needed to install it safely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
  pub version: Version,
  pub notes: String,
  /// The release's web page ("What's new").
  pub page: String,
  install: Install,
  asset_name: String,
  asset_url: String,
  sums_url: String,
  sig_url: Option<String>,
}

/// `signed`: this build has a public key, so a release without the installer's `.sig` is not offered.
fn pick(release: GhRelease, current: Version, install: Install, signed: bool) -> Option<Update> {
  let version = Version::parse(&release.tag_name).filter(|v| *v > current)?;
  let url = |pred: &dyn Fn(&str) -> bool| release.assets.iter().find(|a| pred(&a.name)).map(|a| (a.name.clone(), a.browser_download_url.clone()));
  let (asset_name, asset_url) = url(&|n| n.ends_with(install.suffix()))?;
  let (_, sums_url) = url(&|n| n == SUMS)?;
  let sig_url = url(&|n| n == format!("{asset_name}.sig")).map(|(_, u)| u);
  if signed && sig_url.is_none() {
    return None;
  }
  Some(Update { version, notes: release.body.unwrap_or_default(), page: release.html_url, install, asset_name, asset_url, sums_url, sig_url })
}

/// The newest release, if it is newer than this copy and installable. `Ok(None)` when updates are off.
pub async fn check(http: &Client) -> Result<Option<Update>> {
  let Some(install) = Install::detect() else { return Ok(None) };
  let response =
    http.get(format!("https://api.github.com/repos/{REPO}/releases/latest")).header("Accept", "application/vnd.github+json").send().await?;
  // No release published yet.
  if response.status() == reqwest::StatusCode::NOT_FOUND {
    return Ok(None);
  }
  let release: GhRelease = response.error_for_status()?.json().await?;
  Ok(pick(release, Version::current(), install, public_key().is_some()))
}

/// The hex SHA-256 listed for `name` in a `sha256sum`-style file (`<hex>  <name>`).
fn expected_sum(sums: &str, name: &str) -> Option<String> {
  sums.lines().find_map(|line| {
    let (hash, file) = line.trim().split_once(char::is_whitespace)?;
    (file.trim().trim_start_matches('*') == name && hash.len() == 64).then(|| hash.to_ascii_lowercase())
  })
}

fn unhex<const N: usize>(text: &str) -> Option<[u8; N]> {
  let text = text.trim();
  if text.len() != N * 2 || !text.is_ascii() {
    return None;
  }
  let mut out = [0; N];
  for (i, byte) in out.iter_mut().enumerate() {
    *byte = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).ok()?;
  }
  Some(out)
}

/// Checks that `signature` is our key's signature over the SHA-256 digest `digest_hex`.
fn verify(public_key_hex: &str, digest_hex: &str, signature: &[u8]) -> Result<()> {
  let key = unhex::<32>(public_key_hex).and_then(|k| VerifyingKey::from_bytes(&k).ok()).ok_or(Error::BadSignature)?;
  let digest = unhex::<32>(digest_hex).ok_or(Error::BadSignature)?;
  let signature = Signature::from_slice(signature).map_err(|_| Error::BadSignature)?;
  key.verify_strict(&digest, &signature).map_err(|_| Error::BadSignature)
}

/// Checks the signature (signed builds), then downloads and hash-checks the installer. Nothing is left behind on failure.
pub async fn download(http: &Client, update: &Update, progress: impl Fn(u64, u64)) -> Result<PathBuf> {
  let sums = http.get(&update.sums_url).send().await?.error_for_status()?.text().await?;
  let expected = expected_sum(&sums, &update.asset_name).ok_or_else(|| Error::Checksum(update.asset_name.clone()))?;
  if let Some(key) = public_key() {
    let sig_url = update.sig_url.as_deref().ok_or(Error::BadSignature)?;
    let signature = http.get(sig_url).send().await?.error_for_status()?.bytes().await?;
    // Before downloading: a hash that is not ours makes the 20 MB pointless.
    verify(key, &expected, &signature)?;
  }
  let path = match update.install {
    Install::Msi => std::env::temp_dir().join(&update.asset_name),
    // Next to the running exe, so the swap is a rename on the same volume.
    Install::Portable => std::env::current_exe()?.with_extension("exe.new"),
  };
  download_verified::<Sha256>(http, &update.asset_url, &path, &expected, progress).await?;
  Ok(path)
}

/// Starts the new version. The caller must quit right away so its files can be replaced.
pub fn apply(update: &Update, downloaded: &Path) -> Result<()> {
  match update.install {
    // Per-user MSI: no administrator. LAUNCHAPP makes it start the new version when done.
    Install::Msi => {
      Command::new("msiexec").arg("/i").arg(downloaded).args(["/passive", "LAUNCHAPP=1"]).spawn()?;
    }
    // Windows lets a running exe be renamed, not overwritten: move it aside, put the new one in place, start it.
    Install::Portable => {
      let exe = std::env::current_exe()?;
      let old = exe.with_extension("exe.old");
      let _ = std::fs::remove_file(&old);
      std::fs::rename(&exe, &old)?;
      if let Err(e) = std::fs::rename(downloaded, &exe) {
        let _ = std::fs::rename(&old, &exe);
        return Err(e.into());
      }
      Command::new(&exe).arg(AFTER_UPDATE_ARG).spawn()?;
    }
  }
  Ok(())
}

/// Removes what a portable update left behind.
pub fn clean_up() {
  if let Ok(exe) = std::env::current_exe() {
    let _ = std::fs::remove_file(exe.with_extension("exe.old"));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ed25519_dalek::{Signer, SigningKey};

  fn release(tag: &str, assets: &[&str]) -> GhRelease {
    let assets: Vec<_> =
      assets.iter().map(|n| serde_json::json!({ "name": n, "browser_download_url": format!("https://example.com/{n}") })).collect();
    serde_json::from_value(serde_json::json!({ "tag_name": tag, "html_url": "https://github.com/r", "body": "notes", "assets": assets })).unwrap()
  }

  const FULL: [&str; 5] = [
    "PandaLauncher-0.2.0-x64.msi",
    "PandaLauncher-0.2.0-x64.msi.sig",
    "PandaLauncher-0.2.0-x64-portable.exe",
    "PandaLauncher-0.2.0-x64-portable.exe.sig",
    "SHA256SUMS.txt",
  ];

  #[test]
  fn versions_compare_numerically() {
    assert!(Version::parse("v0.10.0").unwrap() > Version::parse("0.9.9").unwrap());
    assert_eq!(Version::parse("1.2.3").unwrap().to_string(), "1.2.3");
    assert_eq!(Version::parse("v1.2.3-beta.1"), None);
    assert_eq!(Version::parse("1.2"), None);
    assert_eq!(Version::parse("1.2.3.4"), None);
  }

  #[test]
  fn offers_only_newer_complete_releases() {
    let current = Version::parse("0.1.0").unwrap();
    let msi = pick(release("v0.2.0", &FULL), current, Install::Msi, true).unwrap();
    assert_eq!(msi.asset_name, "PandaLauncher-0.2.0-x64.msi");
    assert!(msi.sig_url.unwrap().ends_with("x64.msi.sig"));
    let portable = pick(release("v0.2.0", &FULL), current, Install::Portable, true).unwrap();
    assert_eq!(portable.asset_name, "PandaLauncher-0.2.0-x64-portable.exe");

    assert_eq!(pick(release("v0.1.0", &FULL), current, Install::Msi, false), None, "same version");
    assert_eq!(pick(release("v0.0.9", &FULL), current, Install::Msi, false), None, "older");
    assert_eq!(pick(release("v0.2.0", &FULL[..4]), current, Install::Msi, false), None, "no checksums");

    let unsigned = [FULL[0], FULL[4]];
    assert_eq!(pick(release("v0.2.0", &unsigned), current, Install::Msi, true), None, "signed build needs a signature");
    let msi = pick(release("v0.2.0", &unsigned), current, Install::Msi, false).expect("keyless build takes checksums only");
    assert_eq!(msi.sig_url, None);
  }

  #[test]
  fn reads_sha256sum_files() {
    let hash = "a".repeat(64);
    let sums = format!("{hash}  PandaLauncher-0.2.0-x64.msi\r\n{}  *PandaLauncher-0.2.0-x64-portable.exe\n", "B".repeat(64));
    assert_eq!(expected_sum(&sums, "PandaLauncher-0.2.0-x64.msi"), Some(hash));
    assert_eq!(expected_sum(&sums, "PandaLauncher-0.2.0-x64-portable.exe"), Some("b".repeat(64)));
    assert_eq!(expected_sum(&sums, "other.exe"), None);
  }

  #[test]
  fn accepts_only_our_signature_over_the_listed_hash() {
    let key = SigningKey::from_bytes(&[7; 32]);
    let public = crate::download::hex(key.verifying_key().as_bytes());
    let digest = [0xab_u8; 32];
    let signature = key.sign(&digest).to_bytes();
    let digest_hex = crate::download::hex(&digest);

    assert!(verify(&public, &digest_hex, &signature).is_ok());
    assert!(verify(&public, &"cd".repeat(32), &signature).is_err(), "another file");
    let other = crate::download::hex(SigningKey::from_bytes(&[8; 32]).verifying_key().as_bytes());
    assert!(verify(&other, &digest_hex, &signature).is_err(), "another key");
    assert!(verify(&public, &digest_hex, &signature[..63]).is_err(), "truncated");
  }
}
