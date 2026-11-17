//! About: version, updates, licences and the "not official" disclaimer (ADR 0012).

use super::state::{self, AppState, UpdateState};
use crate::i18n::{t, tr};
use zgui::prelude::*;
use zgui_ui::prelude::*;

#[component]
pub fn AboutPage(state: AppState) -> impl IntoView {
  let status = move || match state.update.get() {
    UpdateState::Checking => t("update-checking"),
    UpdateState::UpToDate => t("update-up-to-date"),
    UpdateState::Available(u) => tr("update-available", &[("version", u.version.to_string())]),
    UpdateState::Failed(e) => e,
    UpdateState::Idle | UpdateState::Downloading(_) => String::new(),
  };
  let busy = move || matches!(state.update.get(), UpdateState::Checking | UpdateState::Downloading(_));
  let updates = panda_core::update::enabled();
  view! {
    row(class = "about__brand") {
      image(class = "about__logo", src = state::logo_path(), alt = "")
      column(class = "page-header") {
        label(class = "heading-4") {{t("about-title")}}
        label(class = "body-6 muted") {{tr("about-version", &[("version", env!("CARGO_PKG_VERSION").into())])}}
      }
    }
    column(class = "panel about") {
      if move || updates {
        row(class = "search-row") {
          Button(variant = ButtonVariant::Secondary, disabled = Signal::derive_local(busy), on:click = move |_| state::check_updates(state)) {
            label {{t("update-check")}}
          }
          label(class = "muted", class:error = move || matches!(state.update.get(), UpdateState::Failed(_)), a11y:live = zgui::vocab::Live::Polite) {
            {status}
          }
        }
      } else {
        label(class = "muted body-6") {{t("update-unavailable")}}
      }
      label(class = "warning body-6") {{t("about-disclaimer")}}
      label(class = "muted body-6") {{t("about-licenses")}}
      label(class = "muted body-6") {{t("about-source")}}
    }
  }
}
