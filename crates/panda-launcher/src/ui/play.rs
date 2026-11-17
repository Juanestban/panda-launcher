//! Play: sign in, pick a version, press Play.

use super::state::{self, Account, AppState, PlayState};
use crate::i18n::{t, tr};
use crate::icons;
use zgui::prelude::*;
use zgui_ui::prelude::*;
use zgui_ui_icons::prelude::*;

#[component]
pub fn PlayPage(state: AppState) -> impl IntoView {
  view! {
    column(class = "page-header") {
      label(class = "eyebrow") {{t("play-eyebrow")}}
      label(class = "heading-4") {{t("play-title")}}
      label(class = "body-6 muted") {{t("play-subtitle")}}
    }
    if move || matches!(state.account.get(), Account::SignedIn { .. }) {
      PlayPanel(state = state)
    } else {
      SignInPanel(state = state)
    }
    if move || state.account_error.get().is_some() {
      Alert(variant = AlertVariant::Destructive) {
        AlertDescription {{move || state.account_error.get().unwrap_or_default()}}
      }
    }
  }
}

#[component]
fn SignInPanel(state: AppState) -> impl IntoView {
  let pending = RwSignal::new_local(None::<Task>);
  let code = move || match state.account.get() {
    Account::Code { code, uri } => Some((code, uri)),
    _ => None,
  };
  view! {
    row(class = "hero") {
    column(class = "hero__text") {
      if move || code().is_some() {
        label(class = "eyebrow") {{t("account-code-title")}}
        label(class = "code", a11y:live = zgui::vocab::Live::Polite) {{move || code().map(|c| c.0).unwrap_or_default()}}
        label(class = "muted") {{move || tr("account-code-hint", &[("uri", code().map(|c| c.1).unwrap_or_default())])}}
        row(class = "search-row") {
          Button(on:click = move |_| if let Some((_, uri)) = code() { state::open_url(&uri) }) {{t("account-open-link")}}
          Button(variant = ButtonVariant::Ghost, on:click = move |_| {
            if let Some(task) = pending.get_untracked() { task.cancel() }
            state.account.set(Account::SignedOut);
          }) {{t("account-cancel")}}
        }
        Skeleton(class = "skeleton-row")
      } else {
        label(class = "eyebrow") {{t("play-welcome")}}
        label(class = "heading-5") {{t("play-sign-in-title")}}
        label(class = "body-6 muted") {{t("play-sign-in-first")}}
        row(class = "hero__controls") {
          Button(
            size = ButtonSize::Lg,
            disabled = Signal::derive_local(move || state.account.get() == Account::Checking),
            on:click = move |_| pending.set(Some(state::sign_in(state)))
          ) {{t("account-sign-in")}}
        }
      }
    }
      image(class = "hero__art", src = state::logo_path(), alt = "")
    }
  }
}

#[component]
fn PlayPanel(state: AppState) -> impl IntoView {
  let busy = move || matches!(state.play.get(), PlayState::Working { .. } | PlayState::Running);
  let mods = move || state::installed_mods(state).len();
  view! {
    row(class = "hero") {
     column(class = "hero__text") {
      label(class = "eyebrow") {{t("play-ready")}}
      label(class = "heading-4") {{move || format!("Minecraft {}", state.version.get())}}
      label(class = "body-6 muted") {{move || tr("play-hero-mods", &[("count", mods().to_string())])}}
      row(class = "hero__controls") {
        column(class = "field") {
          label(class = "field__label") {{t("play-version")}}
          VersionSelect(state = state, disabled = Signal::derive_local(busy))
        }
        Button(size = ButtonSize::Lg, disabled = Signal::derive_local(busy), on:click = move |_| state::play(state)) {
          Icon(icon = icons::PLAY, size = IconSize::Sm)
          label {{t("play-button")}}
        }
      }
     }
      image(class = "hero__art", src = state::logo_path(), alt = "")
    }
    if move || state.versions_failed.get() {
      label(class = "error") {{t("play-versions-error")}}
    }
    PlayStatus(state = state)
    row(class = "stats") {
      column(class = "stat") {
        label(class = "stat__value") {{move || mods().to_string()}}
        label(class = "caption-4 muted") {{t("play-stat-mods")}}
      }
      column(class = "stat") {
        label(class = "stat__value") {{move || format!("{} GB", state.settings.get().memory_mb / 1024)}}
        label(class = "caption-4 muted") {{t("play-stat-memory")}}
      }
      column(class = "stat") {
        label(class = "stat__value") {"Fabric"}
        label(class = "caption-4 muted") {{t("play-stat-loader")}}
      }
    }
  }
}

#[component]
fn PlayStatus(state: AppState) -> impl IntoView {
  let working = move || match state.play.get() {
    PlayState::Working { step, done, total } => Some((step, done, total)),
    _ => None,
  };
  view! {
    column(class = "play-status", a11y:live = zgui::vocab::Live::Polite) {
      if move || working().is_some() {
        label {{move || working().map(|w| w.0).unwrap_or_default()}}
        Progress(value = Signal::derive_local(move || working().filter(|w| w.2 > 0).map(|w| w.1 as f64 * 100.0 / w.2 as f64)))
        label(class = "muted") {{move || working().filter(|w| w.2 > 0).map(|w| tr("play-files", &[("done", w.1.to_string()), ("total", w.2.to_string())])).unwrap_or_default()}}
      } else {
        if move || state.play.get() == PlayState::Running {
          label(class = "success title-4") {{t("play-running")}}
        } else {
          label(class = "error") {{move || match state.play.get() { PlayState::Failed(e) => e, _ => String::new() }}}
        }
      }
    }
  }
}

/// The Minecraft version picker. Play and Mods share it: both work on the selected version's profile.
#[component]
pub fn VersionSelect(state: AppState, disabled: Signal<bool, LocalStorage>) -> impl IntoView {
  view! {
    if move || state.versions.get().is_empty() && !state.versions_failed.get() {
      Skeleton(class = "skeleton-select")
    } else {
      NativeSelect(value = Binding::controlled(Signal::derive_local(move || state.version.get()), move |v| state.version.set(v)), disabled = disabled, a11y:label = t("play-version")) {
        for v in move || state.versions.get(), key = |v: &String| v.clone() {
          NativeSelectOption(value = v.clone()) {{v.clone()}}
        }
      }
    }
  }
}
