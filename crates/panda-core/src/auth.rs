//! Microsoft → Xbox Live → XSTS → Minecraft sign-in with an ownership check (ADR 0003).

use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use std::time::Duration;

const MSA: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0";
const SCOPE: &str = "XboxLive.signin offline_access";
const DEVICE_GRANT: &str = "urn:ietf:params:oauth:grant-type:device_code";
const MC: &str = "https://api.minecraftservices.com";

/// Our own Azure app, injected at build time. Never another launcher's ID.
pub const CLIENT_ID: Option<&str> = option_env!("PANDA_MSA_CLIENT_ID");

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
  #[error("this build has no Microsoft client ID (set PANDA_MSA_CLIENT_ID at build time)")]
  NoClientId,
  #[error("the sign-in code expired, start again")]
  Expired,
  #[error("sign-in was declined")]
  Declined,
  #[error("this Microsoft account has no Xbox profile yet; create one at xbox.com")]
  NoXboxAccount,
  #[error("Xbox Live is not available in this account's country")]
  RegionBlocked,
  #[error("this account needs adult verification on xbox.com")]
  AdultVerification,
  #[error("this is a child account; an adult must add it to a Microsoft family")]
  ChildAccount,
  #[error("Xbox rejected the sign-in (XErr {0})")]
  Xsts(u64),
  #[error("this launcher is not approved by Mojang for sign-in yet")]
  AppNotApproved,
  #[error("this account does not own Minecraft: Java Edition")]
  NotOwned,
  #[error("this account has no Minecraft profile yet; pick a name on minecraft.net")]
  NoProfile,
  #[error("Microsoft sign-in error: {0}")]
  Oauth(String),
  #[error(transparent)]
  Http(#[from] reqwest::Error),
  #[error("keychain error: {0}")]
  Keychain(#[from] keyring::Error),
}

impl AuthError {
  /// No network: the only case where a verified account may play offline (ADR 0004).
  pub fn is_offline(&self) -> bool {
    matches!(self, AuthError::Http(e) if e.is_connect() || e.is_timeout())
  }
}

type Result<T> = std::result::Result<T, AuthError>;

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCode {
  pub device_code: String,
  /// Show this to the player.
  pub user_code: String,
  /// Where the player types the code (microsoft.com/link).
  pub verification_uri: String,
  pub expires_in: u64,
  pub interval: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MsaToken {
  pub access_token: String,
  pub refresh_token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
  /// UUID without dashes.
  pub id: String,
  pub name: String,
}

/// A signed-in player who owns the game.
#[derive(Debug, Clone)]
pub struct Session {
  pub profile: Profile,
  pub refresh_token: String,
  /// Minecraft access token (~24 h). Never log or persist it.
  pub access_token: String,
}

fn client_id() -> Result<&'static str> {
  // CI exports an unset secret as an empty string.
  CLIENT_ID.filter(|id| !id.trim().is_empty()).ok_or(AuthError::NoClientId)
}

/// Step 1: ask Microsoft for a code the player enters at `verification_uri`.
pub async fn device_code(http: &Client) -> Result<DeviceCode> {
  let r = http.post(format!("{MSA}/devicecode")).form(&[("client_id", client_id()?), ("scope", SCOPE)]).send().await?;
  oauth(r).await
}

/// Step 2: wait until the player finishes in the browser.
pub async fn poll(http: &Client, code: &DeviceCode) -> Result<MsaToken> {
  let mut interval = code.interval.max(1);
  loop {
    tokio::time::sleep(Duration::from_secs(interval)).await;
    let form = [("grant_type", DEVICE_GRANT), ("device_code", &code.device_code)];
    match token(http, &form).await {
      Err(AuthError::Oauth(e)) if e == "authorization_pending" => {}
      Err(AuthError::Oauth(e)) if e == "slow_down" => interval += 5,
      done => return done,
    }
  }
}

/// Swap a stored refresh token for a fresh one.
pub async fn refresh(http: &Client, refresh_token: &str) -> Result<MsaToken> {
  let form = [("grant_type", "refresh_token"), ("refresh_token", refresh_token), ("scope", SCOPE)];
  token(http, &form).await
}

async fn token(http: &Client, form: &[(&str, &str)]) -> Result<MsaToken> {
  let mut all = vec![("client_id", client_id()?)];
  all.extend_from_slice(form);
  let r = http.post(format!("{MSA}/token")).form(&all).send().await?;
  oauth(r).await
}

async fn oauth<T: DeserializeOwned>(r: Response) -> Result<T> {
  if r.status().is_success() {
    return Ok(r.json().await?);
  }
  #[derive(Deserialize)]
  struct Err {
    error: String,
  }
  let e: Err = r.json().await?;
  Err(match e.error.as_str() {
    "expired_token" => AuthError::Expired,
    "authorization_declined" | "access_denied" => AuthError::Declined,
    _ => AuthError::Oauth(e.error),
  })
}

/// Steps 3–7: turn a Microsoft token into a Minecraft session, refusing non-owners.
pub async fn sign_in(http: &Client, msa: MsaToken) -> Result<Session> {
  let (xbl, uhs) = xbox_live(http, &msa.access_token).await?;
  let xsts = xsts(http, &xbl).await?;
  let access_token = login_with_xbox(http, &uhs, &xsts).await?;
  if !owns_java(http, &access_token).await? {
    return Err(AuthError::NotOwned);
  }
  let profile = profile(http, &access_token).await?;
  Ok(Session { profile, refresh_token: msa.refresh_token, access_token })
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct XboxToken {
  token: String,
  display_claims: Claims,
}
#[derive(Deserialize)]
struct Claims {
  xui: Vec<Xui>,
}
#[derive(Deserialize)]
struct Xui {
  uhs: String,
}

async fn xbox_live(http: &Client, msa_access: &str) -> Result<(String, String)> {
  let body = json!({
      "Properties": {
          "AuthMethod": "RPS",
          "SiteName": "user.auth.xboxlive.com",
          "RpsTicket": format!("d={msa_access}"),
      },
      "RelyingParty": "http://auth.xboxlive.com",
      "TokenType": "JWT",
  });
  let r = http.post("https://user.auth.xboxlive.com/user/authenticate").json(&body).send().await?.error_for_status()?;
  let t: XboxToken = r.json().await?;
  let uhs = t.display_claims.xui.into_iter().next().map(|x| x.uhs);
  Ok((t.token, uhs.ok_or(AuthError::Oauth("Xbox token without user hash".into()))?))
}

async fn xsts(http: &Client, xbl: &str) -> Result<String> {
  let body = json!({
      "Properties": { "SandboxId": "RETAIL", "UserTokens": [xbl] },
      "RelyingParty": "rp://api.minecraftservices.com/",
      "TokenType": "JWT",
  });
  let r = http.post("https://xsts.auth.xboxlive.com/xsts/authorize").json(&body).send().await?;
  if r.status() == StatusCode::UNAUTHORIZED {
    #[derive(Deserialize)]
    struct XErr {
      #[serde(rename = "XErr")]
      code: u64,
    }
    return Err(xerr(r.json::<XErr>().await?.code));
  }
  Ok(r.error_for_status()?.json::<XboxToken>().await?.token)
}

/// Known XSTS refusals, so the player sees what to fix instead of a number.
fn xerr(code: u64) -> AuthError {
  match code {
    2148916233 => AuthError::NoXboxAccount,
    2148916235 => AuthError::RegionBlocked,
    2148916236 | 2148916237 => AuthError::AdultVerification,
    2148916238 => AuthError::ChildAccount,
    other => AuthError::Xsts(other),
  }
}

async fn login_with_xbox(http: &Client, uhs: &str, xsts: &str) -> Result<String> {
  let r =
    http.post(format!("{MC}/authentication/login_with_xbox")).json(&json!({ "identityToken": format!("XBL3.0 x={uhs};{xsts}") })).send().await?;
  // Mojang answers 403 until our Azure app is approved (ROADMAP Phase 0).
  if r.status() == StatusCode::FORBIDDEN {
    return Err(AuthError::AppNotApproved);
  }
  #[derive(Deserialize)]
  struct McToken {
    access_token: String,
  }
  Ok(r.error_for_status()?.json::<McToken>().await?.access_token)
}

#[derive(Deserialize)]
struct Entitlements {
  items: Vec<Item>,
}
#[derive(Deserialize)]
struct Item {
  name: String,
}

async fn owns_java(http: &Client, mc: &str) -> Result<bool> {
  let e: Entitlements = http.get(format!("{MC}/entitlements/mcstore")).bearer_auth(mc).send().await?.error_for_status()?.json().await?;
  Ok(has_java(&e))
}

/// Game Pass shows up as `game_minecraft` too.
fn has_java(e: &Entitlements) -> bool {
  e.items.iter().any(|i| i.name == "game_minecraft" || i.name == "product_minecraft")
}

async fn profile(http: &Client, mc: &str) -> Result<Profile> {
  let r = http.get(format!("{MC}/minecraft/profile")).bearer_auth(mc).send().await?;
  if r.status() == StatusCode::NOT_FOUND {
    return Err(AuthError::NoProfile);
  }
  Ok(r.error_for_status()?.json().await?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn xerr_codes_map_to_actionable_errors() {
    assert!(matches!(xerr(2148916233), AuthError::NoXboxAccount));
    assert!(matches!(xerr(2148916238), AuthError::ChildAccount));
    assert!(matches!(xerr(1), AuthError::Xsts(1)));
  }

  #[test]
  fn ownership_needs_a_java_entitlement() {
    let parse = |s| serde_json::from_str::<Entitlements>(s).unwrap();
    assert!(has_java(&parse(r#"{"items":[{"name":"product_minecraft","signature":"x"}]}"#)));
    assert!(!has_java(&parse(r#"{"items":[{"name":"product_minecraft_bedrock"}]}"#)));
    assert!(!has_java(&parse(r#"{"items":[]}"#)));
  }

  #[test]
  fn xbox_token_parses_user_hash() {
    let t: XboxToken = serde_json::from_str(r#"{"IssueInstant":"x","Token":"tok","DisplayClaims":{"xui":[{"uhs":"123"}]}}"#).unwrap();
    assert_eq!((t.token.as_str(), t.display_claims.xui[0].uhs.as_str()), ("tok", "123"));
  }
}
