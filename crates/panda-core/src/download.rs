//! Hash-verified, parallel downloads written atomically (temp name, then rename).

use crate::{Error, Result};
use futures_util::{StreamExt, stream};
use reqwest::Client;
use serde::de::DeserializeOwned;
use sha1::{Digest, Sha1};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

const PARALLEL: usize = 16;

#[derive(Debug, Clone)]
pub struct FileSpec {
  pub url: String,
  pub path: PathBuf,
  pub sha1: String,
  /// Unknown for a few metadata files; then any existing file counts as present.
  pub size: Option<u64>,
}

/// Download one file unless it is already there.
// shortcut: an existing file is trusted when its size matches (hashing ~4000 assets each launch is slow); add a "repair" action that re-hashes when users report corruption.
pub async fn fetch(http: &Client, f: &FileSpec) -> Result<()> {
  if let Ok(meta) = tokio::fs::metadata(&f.path).await
    && f.size.is_none_or(|s| s == meta.len())
  {
    return Ok(());
  }
  download_verified::<Sha1>(http, &f.url, &f.path, &f.sha1, |_, _| {}).await
}

/// Stream `url` to `path`, hashing with `D` (SHA-1 for Mojang, SHA-512 for Modrinth); a mismatch leaves nothing behind.
/// `progress(bytes, total)` follows the body; `total` is 0 when the server does not say.
pub async fn download_verified<D: Digest>(http: &Client, url: &str, path: &Path, expected_hex: &str, progress: impl Fn(u64, u64)) -> Result<()> {
  if let Some(dir) = path.parent() {
    tokio::fs::create_dir_all(dir).await?;
  }
  let mut tmp = path.to_path_buf().into_os_string();
  tmp.push(".part");
  let mut response = http.get(url).send().await?.error_for_status()?;
  let mut file = tokio::fs::File::create(&tmp).await?;
  let mut hash = D::new();
  let (total, mut done) = (response.content_length().unwrap_or(0), 0);
  while let Some(chunk) = response.chunk().await? {
    hash.update(&chunk);
    file.write_all(&chunk).await?;
    done += chunk.len() as u64;
    progress(done, total);
  }
  file.flush().await?;
  drop(file);
  if hex(&hash.finalize()) != expected_hex.to_ascii_lowercase() {
    let _ = tokio::fs::remove_file(&tmp).await;
    return Err(Error::Checksum(url.into()));
  }
  tokio::fs::rename(&tmp, path).await?;
  Ok(())
}

/// Download everything, 16 at a time; `progress(done, total)` after each file.
pub async fn fetch_all(http: &Client, files: &[FileSpec], progress: impl Fn(usize, usize)) -> Result<()> {
  // Assets share hashes, so the same path can appear twice; two parallel writers would race on the `.part` file.
  let mut seen = std::collections::HashSet::new();
  let unique: Vec<&FileSpec> = files.iter().filter(|f| seen.insert(&f.path)).collect();
  // Collected first: a closure inside the stream's type makes the whole future non-`Send` (rustc #102211).
  let futures: Vec<_> = unique.iter().map(|f| fetch(http, f)).collect();
  let mut pending = stream::iter(futures).buffer_unordered(PARALLEL);
  let mut done = 0;
  while let Some(result) = pending.next().await {
    result?;
    done += 1;
    progress(done, unique.len());
  }
  Ok(())
}

/// Download (if needed) and parse a JSON file.
pub async fn json<T: DeserializeOwned>(http: &Client, f: &FileSpec) -> Result<T> {
  fetch(http, f).await?;
  Ok(serde_json::from_slice(&tokio::fs::read(&f.path).await?)?)
}

/// Parse a small JSON document straight from the network.
pub async fn get_json<T: DeserializeOwned>(http: &Client, url: &str) -> Result<T> {
  Ok(http.get(url).send().await?.error_for_status()?.json().await?)
}

pub(crate) fn hex(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
  #[test]
  fn sha1_hex_matches_known_value() {
    use sha1::{Digest, Sha1};
    assert_eq!(super::hex(&Sha1::digest(b"abc")), "a9993e364706816aba3e25717850c26c9cd0d89d");
  }
}
