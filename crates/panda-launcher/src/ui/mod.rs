//! The launcher window: sidebar navigation, account box, and one page at a time.

mod about;
mod format;
mod markdown;
mod mod_page;
mod mods;
mod play;
mod settings;
mod state;

use crate::i18n::{t, tr};
use crate::icons;
use about::AboutPageProps;
use mods::ModsPageProps;
use play::PlayPageProps;
use settings::SettingsPageProps;
use state::{Account, AppState, Page, UpdateState};
use zgui::prelude::*;
use zgui_ui::prelude::*;
use zgui_ui_icons::prelude::*;
use zgui_ui_tokens::prelude::*;

#[component]
pub fn App() -> impl IntoView {
  let state = AppState::new();
  state::start(state);
  state::start_update_checks(state);
  crate::i18n::set_lang(crate::i18n::Lang::from_setting(state.settings.get_untracked().language.as_deref()));
  view! {
    ThemeProvider(dark = crate::style::theme(), scheme = ColorScheme::Dark) {
      // Strings are read once when a view is built, so a language change rebuilds the window under a new key.
      for _lang in move || [state.settings.with(|s| s.language.clone())], key = |l: &Option<String>| l.clone() {
        Shell(state = state)
      }
    }
  }
}

#[component]
fn Shell(state: AppState) -> impl IntoView {
  view! {
    row(class = "shell") {
      Sidebar(state = state)
      column(class = "main") {
        UpdateBanner(state = state)
        if move || state.page.get() == Page::Play {
          PlayPage(state = state)
        }
        if move || state.page.get() == Page::Mods {
          ModsPage(state = state)
        }
        if move || state.page.get() == Page::Settings {
          SettingsPage(state = state)
        }
        if move || state.page.get() == Page::About {
          AboutPage(state = state)
        }
      }
    }
  }
}

#[component]
fn Sidebar(state: AppState) -> impl IntoView {
  view! {
    column(class = "sidebar", a11y:role = Role::Navigation, a11y:label = "Panda Launcher") {
      row(class = "brand") {
        image(class = "brand__logo", src = state::logo_path(), alt = "")
        column(class = "brand__text") {
          label(class = "brand__name") {"Panda Launcher"}
          label(class = "caption-5 muted") {"Minecraft · Fabric"}
        }
      }
      label(class = "nav-label") {{t("nav-section")}}
      NavItem(state = state, page = Page::Play, icon = icons::PLAY, label = t("nav-play"))
      NavItem(state = state, page = Page::Mods, icon = icons::PUZZLE, label = t("nav-mods"))
      NavItem(state = state, page = Page::Settings, icon = icons::GEAR, label = t("nav-settings"))
      NavItem(state = state, page = Page::About, icon = icons::INFO, label = t("nav-about"))
      spacer()
      AccountBox(state = state)
    }
  }
}

#[component]
fn NavItem(state: AppState, page: Page, icon: IconData, #[prop(into)] label: String) -> impl IntoView {
  view! {
    control(
      class = "nav-item",
      class:is-active = move || state.page.get() == page,
      tabindex = Focus::Sequential,
      a11y:label = label.clone(),
      on:click = move |_| {
        // The Mods entry always leads back to the browser, even from a mod's page.
        state.mod_open.set(None);
        state.page.set(page);
      }
    ) {
      Icon(class = "icon", icon = icon, size = IconSize::Md)
      label {{label}}
    }
  }
}

#[component]
fn AccountBox(state: AppState) -> impl IntoView {
  let signed_in_name = move || match state.account.get() {
    Account::SignedIn { name, .. } => Some(name),
    _ => None,
  };
  let offline = move || matches!(state.account.get(), Account::SignedIn { offline: true, .. });
  view! {
    column(class = "account") {
      if move || signed_in_name().is_some() {
        row(class = "account__row") {
          row(class = "account__avatar") { label {{move || signed_in_name().and_then(|n| n.chars().next()).map(|c| c.to_uppercase().to_string()).unwrap_or_default()}} }
          column(class = "account__text") {
            label(class = "title-4") {{move || signed_in_name().unwrap_or_default()}}
            label(class = "account__status", class:is-offline = offline) {
              {move || if offline() { t("account-offline") } else { t("account-online") }}
            }
          }
        }
        Button(variant = ButtonVariant::Ghost, size = ButtonSize::Sm, on:click = move |_| state::sign_out(state)) {
          Icon(icon = icons::SIGN_OUT, size = IconSize::Sm)
          label {{t("account-sign-out")}}
        }
      } else {
        if move || state.account.get() == Account::Checking {
          Skeleton(class = "skeleton-row")
        } else {
          row(class = "account__row") {
            row(class = "account__avatar") { label {"?"} }
            column(class = "account__text") {
              label(class = "title-4") {{t("account-guest")}}
              label(class = "account__status is-out") {{t("account-signed-out")}}
            }
          }
        }
      }
    }
  }
}

/// Offers a found update; nothing downloads until the player clicks (ADR 0010).
#[component]
fn UpdateBanner(state: AppState) -> impl IntoView {
  let available = move || match state.update.get() {
    UpdateState::Available(u) => Some(u),
    _ => None,
  };
  let downloading = move || match state.update.get() {
    UpdateState::Downloading(f) => Some(f),
    _ => None,
  };
  view! {
    if move || available().is_some() {
      row(class = "update-banner", a11y:live = zgui::vocab::Live::Polite) {
        label(class = "update-banner__text") {
          {move || tr("update-available", &[("version", available().map(|u| u.version.to_string()).unwrap_or_default())])}
        }
        Button(variant = ButtonVariant::Ghost, size = ButtonSize::Sm,
               on:click = move |_| if let Some(u) = available() { state::open_url(&u.page) }) {
          label {{t("update-whats-new")}}
        }
        Button(size = ButtonSize::Sm, on:click = move |_| state::install_update(state)) {
          Icon(icon = icons::DOWNLOAD, size = IconSize::Sm)
          label {{t("update-install")}}
        }
      }
    }
    if move || downloading().is_some() {
      column(class = "update-banner", a11y:live = zgui::vocab::Live::Polite) {
        label {{t("update-downloading")}}
        Progress(value = Signal::derive_local(move || downloading().flatten().map(|f| f * 100.0)))
      }
    }
  }
}
