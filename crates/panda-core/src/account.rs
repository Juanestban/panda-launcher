//! The one stored account and offline play after verified ownership (ADR 0004).

use crate::auth::{self, AuthError, Profile, Session};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const SERVICE: &str = "panda-launcher";
const USER: &str = "microsoft-account";

/// What survives a restart. Lives only in the OS keychain.
/// Kept small: Windows Credential Manager caps a secret at 2560 bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
  pub profile: Profile,
  pub refresh_token: String,
  /// Unix seconds of the last successful online ownership check.
  pub verified_at: u64,
}

pub enum Resume {
  /// Refreshed and ownership re-checked online.
  Online(Session),
  /// No network, but this account was verified online before.
  Offline(Account),
  SignedOut,
}

fn entry() -> Result<keyring::Entry, AuthError> {
  Ok(keyring::Entry::new(SERVICE, USER)?)
}

pub fn load() -> Result<Option<Account>, AuthError> {
  match entry()?.get_password() {
    // A corrupt entry is treated as signed out rather than blocking the launcher.
    Ok(json) => Ok(serde_json::from_str(&json).ok()),
    Err(keyring::Error::NoEntry) => Ok(None),
    Err(e) => Err(e.into()),
  }
}

/// Remember a session that just passed the ownership check.
pub fn save(session: &Session) -> Result<Account, AuthError> {
  let account = Account {
    profile: session.profile.clone(),
    refresh_token: session.refresh_token.clone(),
    verified_at: SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs()),
  };
  let json = serde_json::to_string(&account).expect("account serialises");
  entry()?.set_password(&json)?;
  Ok(account)
}

pub fn sign_out() -> Result<(), AuthError> {
  match entry()?.delete_credential() {
    Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
    Err(e) => Err(e.into()),
  }
}

/// On start: refresh silently; fall back to offline only when the network is down.
pub async fn resume(http: &Client) -> Result<Resume, AuthError> {
  let Some(account) = load()? else {
    return Ok(Resume::SignedOut);
  };
  let online = async {
    let msa = auth::refresh(http, &account.refresh_token).await?;
    auth::sign_in(http, msa).await
  };
  match online.await {
    Ok(session) => {
      save(&session)?;
      Ok(Resume::Online(session))
    }
    Err(e) if e.is_offline() => Ok(Resume::Offline(account)),
    // Ownership gone or the token was revoked: forget the account.
    Err(e @ (AuthError::NotOwned | AuthError::Oauth(_) | AuthError::Expired)) => {
      sign_out()?;
      Err(e)
    }
    Err(e) => Err(e),
  }
}
