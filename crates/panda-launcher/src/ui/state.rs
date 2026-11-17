//! Shared state and the actions that call panda-core off the UI thread.

use crate::i18n::tr;
use panda_core::account::{self, Resume};
use panda_core::launch::{Launch, Player};
use panda_core::net::Client;
use panda_core::paths::Dirs;
use panda_core::settings::{self, Settings};
use panda_core::{auth, fabric, java, version};
use std::sync::OnceLock;
use zgui::prelude::*;

/// What every action needs: one HTTP client and the data folder.
pub struct Core {
  pub http: Client,
  pub dirs: Dirs,
}

pub fn core() -> &'static Core {
  static CORE: OnceLock<Core> = OnceLock::new();
  CORE.get_or_init(|| Core { http: panda_core::net::client(), dirs: Dirs::platform() })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
  Play,
  Mods,
  Settings,
  About,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Account {
  Checking,
  SignedOut,
  /// Waiting for the player to enter `code` at `uri`.
  Code {
    code: String,
    uri: String,
  },
  SignedIn {
    name: String,
    uuid: String,
    token: String,
    offline: bool,
  },
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayState {
  Idle,
  Working { step: String, done: usize, total: usize },
  Running,
  Failed(String),
}

/// Signals the screens share. All `Send`, so background work can post to them.
#[derive(Clone, Copy)]
pub struct AppState {
  pub page: RwSignal<Page>,
  pub account: RwSignal<Account>,
  pub account_error: RwSignal<Option<String>>,
  pub versions: RwSignal<Vec<String>>,
  pub versions_failed: RwSignal<bool>,
  /// The selected Minecraft version; Play and Mods both work on its profile.
  pub version: RwSignal<String>,
  pub play: RwSignal<PlayState>,
  pub settings: RwSignal<Settings>,
}

impl AppState {
  pub fn new() -> Self {
    Self {
      page: RwSignal::new(Page::Play),
      account: RwSignal::new(Account::Checking),
      account_error: RwSignal::new(None),
      versions: RwSignal::new(Vec::new()),
      versions_failed: RwSignal::new(false),
      version: RwSignal::new(String::new()),
      play: RwSignal::new(PlayState::Idle),
      settings: RwSignal::new(settings::load(&core().dirs)),
    }
  }
}

// shortcut: core errors are shown in English (thiserror text); map them to Fluent keys when the UI copy is finalised.
fn message(e: impl std::fmt::Display) -> String {
  e.to_string()
}

/// On start: restore the saved account (ADR 0004) and load the version list.
pub fn start(state: AppState) {
  spawn(async move {
    match background(async { account::resume(&core().http).await }).await {
      Ok(Resume::Online(s)) => state.account.set(signed_in(&s.profile, s.access_token, false)),
      Ok(Resume::Offline(a)) => state.account.set(signed_in(&a.profile, "0".into(), true)),
      Ok(Resume::SignedOut) => state.account.set(Account::SignedOut),
      Err(e) => {
        state.account.set(Account::SignedOut);
        state.account_error.set(Some(message(e)));
      }
    }
  });
  spawn(async move {
    let versions = match background(async { fabric::game_versions(&core().http).await }).await {
      Ok(v) => v,
      // Offline: the profiles already on disk.
      Err(_) => installed_versions(),
    };
    state.versions_failed.set(versions.is_empty());
    if state.version.get_untracked().is_empty()
      && let Some(first) = versions.first()
    {
      state.version.set(first.clone());
    }
    state.versions.set(versions);
  });
}

fn installed_versions() -> Vec<String> {
  let Ok(entries) = std::fs::read_dir(core().dirs.root.join("profiles")) else { return Vec::new() };
  let mut versions: Vec<String> = entries.filter_map(|e| e.ok()?.file_name().to_str()?.strip_suffix("-fabric").map(str::to_owned)).collect();
  versions.sort_by(|a, b| b.cmp(a));
  versions
}

fn signed_in(profile: &auth::Profile, token: String, offline: bool) -> Account {
  Account::SignedIn { name: profile.name.clone(), uuid: profile.id.clone(), token, offline }
}

/// Device-code sign-in: show the code, wait for Microsoft, check ownership, remember the account.
pub fn sign_in(state: AppState) -> Task {
  state.account_error.set(None);
  spawn(async move {
    let code = match background(async { auth::device_code(&core().http).await }).await {
      Ok(code) => code,
      Err(e) => {
        state.account_error.set(Some(message(e)));
        return;
      }
    };
    state.account.set(Account::Code { code: code.user_code.clone(), uri: code.verification_uri.clone() });
    let result = background(async move {
      let http = &core().http;
      let session = auth::sign_in(http, auth::poll(http, &code).await?).await?;
      account::save(&session)?;
      Ok::<_, auth::AuthError>(session)
    })
    .await;
    match result {
      Ok(s) => state.account.set(signed_in(&s.profile, s.access_token, false)),
      Err(e) => {
        state.account.set(Account::SignedOut);
        state.account_error.set(Some(message(e)));
      }
    }
  })
}

pub fn sign_out(state: AppState) {
  if let Err(e) = account::sign_out() {
    state.account_error.set(Some(message(e)));
  }
  state.account.set(Account::SignedOut);
}

/// Opens a URL in the default browser.
pub fn open_url(url: &str) {
  let result = if cfg!(windows) {
    std::process::Command::new("explorer").arg(url).spawn()
  } else if cfg!(target_os = "macos") {
    std::process::Command::new("open").arg(url).spawn()
  } else {
    std::process::Command::new("xdg-open").arg(url).spawn()
  };
  let _ = result; // nothing useful to show: the code and URL stay on screen to type by hand
}

/// Install (or load offline) the selected version's Fabric profile, then start the game.
pub fn play(state: AppState) {
  let Account::SignedIn { name, uuid, token, offline } = state.account.get_untracked() else { return };
  let game = state.version.get_untracked();
  let max_memory_mb = state.settings.get_untracked().memory_mb;
  if game.is_empty() || matches!(state.play.get_untracked(), PlayState::Working { .. } | PlayState::Running) {
    return;
  }
  let ui = ui();
  let play = state.play;
  let step = {
    let ui = ui.clone();
    move |key: &'static str, args: Vec<(&'static str, String)>| {
      ui.post(move || play.set(PlayState::Working { step: tr(key, &args), done: 0, total: 0 }))
    }
  };
  let progress = move |done: usize, total: usize| {
    ui.post(move || {
      play.update(|p| {
        if let PlayState::Working { done: d, total: t, .. } = p {
          (*d, *t) = (done, total);
        }
      })
    })
  };
  play.set(PlayState::Working { step: tr("play-step-version", &[("version", game.clone())]), done: 0, total: 0 });

  spawn(async move {
    let result = background(async move {
      let c = core();
      let v = if offline {
        fabric::load(&c.dirs, &game)?
      } else {
        let manifest = version::manifest(&c.http).await?;
        let vanilla = version::install(&c.http, &c.dirs, manifest.find(&game)?, &progress).await?;
        step("play-step-fabric", vec![]);
        fabric::install(&c.http, &c.dirs, vanilla, &progress).await?
      };
      step("play-step-java", vec![]);
      let java = java::ensure(&c.http, &c.dirs, &v.java_version, &progress).await?;
      step("play-step-launch", vec![]);
      let player = Player { name, uuid, access_token: token };
      let game_dir = c.dirs.profile(&fabric::profile_name(&v.id));
      let launch = Launch { version: &v, java: &java, game_dir: &game_dir, player: &player, max_memory_mb };
      launch.spawn(&c.dirs)
    })
    .await;
    let mut child = match result {
      Ok(child) => child,
      Err(e) => return play.set(PlayState::Failed(message(e))),
    };
    play.set(PlayState::Running);
    let _ = blocking(move || child.wait()).await;
    play.set(PlayState::Idle);
  });
}
