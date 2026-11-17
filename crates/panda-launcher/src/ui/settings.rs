//! Settings: language, game memory, automatic update checks.

use super::state::{AppState, core};
use crate::i18n::{self, t, tr};
use panda_core::settings::{self, Settings};
use zgui::prelude::*;
use zgui_ui::prelude::*;

const MEMORY_GB: [u32; 7] = [2, 3, 4, 6, 8, 12, 16];

#[component]
pub fn SettingsPage(state: AppState) -> impl IntoView {
  let error = RwSignal::new(None::<String>);
  // Saves right away: there is no Save button to forget.
  let change = move |f: &dyn Fn(&mut Settings)| {
    let mut next = state.settings.get_untracked();
    f(&mut next);
    error.set(settings::save(&core().dirs, &next).err().map(|e| tr("settings-save-error", &[("error", e.to_string())])));
    // Language first: setting the signal rebuilds the window, which must read the new strings.
    i18n::set_lang(i18n::Lang::from_setting(next.language.as_deref()));
    state.settings.set(next);
  };

  let language =
    Binding::controlled(Signal::derive_local(move || state.settings.get().language.unwrap_or_else(|| "system".into())), move |v: String| {
      change(&|s| s.language = (v != "system").then(|| v.clone()))
    });
  let memory = Binding::controlled(Signal::derive_local(move || (state.settings.get().memory_mb / 1024).to_string()), move |v: String| {
    if let Ok(gb) = v.parse::<u32>() {
      change(&|s| s.memory_mb = gb * 1024)
    }
  });
  let auto_update =
    Binding::controlled(Signal::derive_local(move || state.settings.get().auto_update), move |on: bool| change(&|s| s.auto_update = on));

  view! {
    column(class = "page-header") {
      label(class = "heading-4") {{t("settings-title")}}
      label(class = "body-6 muted") {{t("settings-subtitle")}}
    }
    column(class = "panel settings") {
      column(class = "field") {
        label(class = "field__label") {{t("settings-language")}}
        NativeSelect(value = language, a11y:label = t("settings-language")) {
          NativeSelectOption(value = "system") {{t("settings-language-system")}}
          NativeSelectOption(value = "es") {"Español"}
          NativeSelectOption(value = "en") {"English"}
        }
      }
      column(class = "field") {
        label(class = "field__label") {{t("settings-memory")}}
        NativeSelect(value = memory, a11y:label = t("settings-memory")) {
          for gb in move || MEMORY_GB, key = |gb: &u32| *gb {
            NativeSelectOption(value = gb.to_string()) {{format!("{gb} GB")}}
          }
        }
        label(class = "muted") {{t("settings-memory-hint")}}
      }
      row(class = "settings__toggle") {
        column(class = "field settings__toggle-text") {
          label {{t("settings-auto-update")}}
          label(class = "muted") {{t("settings-auto-update-hint")}}
        }
        Switch(checked = auto_update, a11y:label = t("settings-auto-update"))
      }
      if move || error.get().is_some() {
        label(class = "error") {{move || error.get().unwrap_or_default()}}
      }
    }
  }
}
