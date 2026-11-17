//! Shared state and the actions that call panda-core off the UI thread.

use crate::i18n::tr;
use panda_core::account::{self, Resume};
use panda_core::launch::{Launch, Player};
use panda_core::net::Client;
use panda_core::paths::Dirs;
use panda_core::settings::{self, Settings};
use panda_core::update;
use panda_core::{auth, fabric, java, version};
use std::sync::OnceLock;
use std::time::Duration;
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

#[derive(Clone, Debug, PartialEq)]
pub enum UpdateState {
  Idle,
  Checking,
  UpToDate,
  Available(update::Update),
  /// Fraction downloaded, when the size is known.
  Downloading(Option<f64>),
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
  pub update: RwSignal<UpdateState>,
  /// The mod whose page is open inside the Mods tab.
  pub mod_open: RwSignal<Option<String>>,
  /// Bumped after any install or removal, so lists re-read `panda-mods.json`.
  pub mods_rev: RwSignal<u32>,
  /// Project being installed, so only its buttons show work.
  pub mods_busy: RwSignal<Option<String>>,
  /// The last install or removal outcome.
  pub mods_notice: RwSignal<Option<Result<String, String>>>,
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
      update: RwSignal::new(UpdateState::Idle),
      mod_open: RwSignal::new(None),
      mods_rev: RwSignal::new(0),
      mods_busy: RwSignal::new(None),
      mods_notice: RwSignal::new(None),
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

async fn sleep(d: Duration) {
  background(tokio::time::sleep(d)).await;
}

/// ADR 0010: 3 s after start, then hourly (5 min after a failure). Automatic failures stay silent.
pub fn start_update_checks(state: AppState) {
  if !update::enabled() {
    return;
  }
  spawn(async move {
    let mut wait = Duration::from_secs(3);
    loop {
      sleep(wait).await;
      wait = Duration::from_secs(3600);
      let busy = matches!(state.update.get_untracked(), UpdateState::Available(_) | UpdateState::Downloading(_));
      if busy || !state.settings.get_untracked().auto_update {
        continue;
      }
      match background(async { update::check(&core().http).await }).await {
        Ok(Some(u)) => state.update.set(UpdateState::Available(u)),
        Ok(None) => {}
        Err(_) => wait = Duration::from_secs(300),
      }
    }
  });
}

/// The "Check for updates" button in About; unlike the automatic check it reports every outcome.
pub fn check_updates(state: AppState) {
  state.update.set(UpdateState::Checking);
  spawn(async move {
    state.update.set(match background(async { update::check(&core().http).await }).await {
      Ok(Some(u)) => UpdateState::Available(u),
      Ok(None) => UpdateState::UpToDate,
      Err(e) => UpdateState::Failed(message(e)),
    });
  });
}

/// Download, verify, hand over to the installer and quit. Only ever runs on a click.
pub fn install_update(state: AppState) {
  let UpdateState::Available(u) = state.update.get_untracked() else { return };
  state.update.set(UpdateState::Downloading(None));
  let (ui, signal) = (ui(), state.update);
  let progress = move |done: u64, total: u64| {
    let fraction = (total > 0).then(|| done as f64 / total as f64);
    ui.post(move || signal.set(UpdateState::Downloading(fraction)));
  };
  spawn(async move {
    let result = background(async move {
      let path = update::download(&core().http, &u, progress).await?;
      update::apply(&u, &path)
    })
    .await;
    match result {
      // The installer or the new exe takes over; settings are already on disk.
      Ok(()) => std::process::exit(0),
      Err(e) => state.update.set(UpdateState::Failed(message(e))),
    }
  });
}

/// Mods installed in the selected version's profile; re-read when it or `mods_rev` changes.
pub fn installed_mods(state: AppState) -> Vec<panda_core::mods::InstalledMod> {
  state.mods_rev.track();
  panda_core::mods::list(&core().dirs, &state.version.get()).map(|l| l.mods).unwrap_or_default()
}

/// Install a mod (its newest build, or `version`) with its dependencies into the selected profile.
pub fn install_mod(state: AppState, project: String, version: Option<String>) {
  let game = state.version.get_untracked();
  state.mods_busy.set(Some(project.clone()));
  spawn(async move {
    let r = background(async move { panda_core::mods::install(&core().http, &core().dirs, &game, &project, version.as_deref()).await }).await;
    state.mods_notice.set(Some(r.map(|names| tr("mods-installed-ok", &[("names", names.join(", "))])).map_err(message)));
    state.mods_busy.set(None);
    state.mods_rev.update(|n| *n += 1);
  });
}

pub fn remove_mod(state: AppState, project: &str) {
  let r = panda_core::mods::uninstall(&core().dirs, &state.version.get_untracked(), project, false);
  state.mods_notice.set(Some(r.map(|names| tr("mods-removed-ok", &[("names", names.join(", "))])).map_err(message)));
  state.mods_rev.update(|n| *n += 1);
}

/// The panda mark as a file: `image` elements load from paths. Rewritten each start so a new build's art wins.
pub fn logo_path() -> Option<String> {
  static LOGO: OnceLock<Option<String>> = OnceLock::new();
  LOGO
    .get_or_init(|| {
      let path = core().dirs.root.join("cache").join("panda.png");
      std::fs::create_dir_all(path.parent()?).ok()?;
      std::fs::write(&path, include_bytes!("../../assets/icon/panda-512.png")).ok()?;
      Some(path.to_string_lossy().into_owned())
    })
    .clone()
}
