//! The launcher window: sidebar navigation, account box, and one page at a time.

mod about;
mod mods;
mod play;
mod settings;
mod state;

use crate::i18n::t;
use crate::icons;
use about::AboutPageProps;
use mods::ModsPageProps;
use play::PlayPageProps;
use settings::SettingsPageProps;
use state::{Account, AppState, Page};
use zgui::prelude::*;
use zgui_ui::prelude::*;
use zgui_ui_icons::prelude::*;
use zgui_ui_tokens::prelude::*;

#[component]
pub fn App() -> impl IntoView {
  let state = AppState::new();
  state::start(state);
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
          AboutPage()
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
        text(class = "brand__dot") {""}
        label(class = "brand__name") {"Panda Launcher"}
      }
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
      on:click = move |_| state.page.set(page)
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
        label(class = "account__name") {{move || signed_in_name().unwrap_or_default()}}
        label(class = "account__status", class:is-offline = offline) {
          {move || if offline() { t("account-offline") } else { t("account-online") }}
        }
        Button(variant = ButtonVariant::Ghost, size = ButtonSize::Sm, on:click = move |_| state::sign_out(state)) {
          Icon(icon = icons::SIGN_OUT, size = IconSize::Sm)
          label {{t("account-sign-out")}}
        }
      } else {
        if move || state.account.get() == Account::Checking {
          Skeleton(class = "skeleton-row")
        } else {
          label(class = "account__status") {{t("account-signed-out")}}
        }
      }
    }
  }
}
