//! Mods: search Modrinth for the selected version, install with dependencies, remove.

use super::state::{AppState, core};
use crate::i18n::{t, tr};
use crate::icons;
use panda_core::mods::{self, InstalledMod, SearchHit};
use zgui::prelude::*;
use zgui_ui::prelude::*;
use zgui_ui_icons::prelude::*;

/// The Mods page's signals, `Copy` so every row can carry them.
#[derive(Clone, Copy)]
struct Mods {
  state: AppState,
  /// Bumped after install/remove; the installed list re-reads panda-mods.json when it or the version changes.
  reload: RwSignal<u32>,
  /// Project being installed, so only its button shows work.
  busy: RwSignal<Option<String>>,
  notice: RwSignal<Option<Result<String, String>>>,
}

impl Mods {
  fn installed(self) -> Vec<InstalledMod> {
    self.reload.track();
    mods::list(&core().dirs, &self.state.version.get()).map(|l| l.mods).unwrap_or_default()
  }

  fn is_installed(self, id: &str) -> bool {
    self.installed().iter().any(|m| m.project_id == id)
  }

  fn install(self, project: String) {
    let game = self.state.version.get_untracked();
    self.busy.set(Some(project.clone()));
    spawn(async move {
      let r = background(async move { mods::install(&core().http, &core().dirs, &game, &project).await }).await;
      self.notice.set(Some(r.map(|names| tr("mods-installed-ok", &[("names", names.join(", "))])).map_err(|e| e.to_string())));
      self.busy.set(None);
      self.reload.update(|n| *n += 1);
    });
  }

  fn remove(self, project: &str) {
    let r = mods::uninstall(&core().dirs, &self.state.version.get_untracked(), project, false);
    self.notice.set(Some(r.map(|names| tr("mods-removed-ok", &[("names", names.join(", "))])).map_err(|e| e.to_string())));
    self.reload.update(|n| *n += 1);
  }
}

#[component]
pub fn ModsPage(state: AppState) -> impl IntoView {
  let query = RwSignal::new_local(String::new());
  // Results remember which version they were searched for, so switching versions hides stale ones.
  let found = RwSignal::new(None::<(String, Result<Vec<SearchHit>, String>)>);
  let searching = RwSignal::new(false);
  let m = Mods { state, reload: RwSignal::new(0), busy: RwSignal::new(None), notice: RwSignal::new(None) };
  let notice = m.notice;
  let results = move || found.get().filter(|(v, _)| *v == state.version.get()).map(|(_, r)| r);

  let search = move || {
    let (game, text) = (state.version.get_untracked(), query.get_untracked());
    searching.set(true);
    spawn(async move {
      let hits = background({
        let game = game.clone();
        async move { mods::search(&core().http, &game, &text, 0).await }
      })
      .await;
      found.set(Some((game, hits.map(|p| p.hits).map_err(|e| e.to_string()))));
      searching.set(false);
    });
  };

  view! {
    column(class = "page-header") {
      label(class = "page-title") {{t("mods-title")}}
      label(class = "page-subtitle") {{move || tr("mods-subtitle", &[("version", state.version.get())])}}
    }
    ScrollArea(class = "mods") {
      column(class = "mods") {
        row(class = "search-row") {
          Input(class = "search-row__input", value = query, placeholder = t("mods-search-placeholder"),
                a11y:label = t("mods-search-placeholder"))
          Button(disabled = Signal::derive_local(move || searching.get()), on:click = move |_| search()) {
            Icon(icon = icons::SEARCH, size = IconSize::Sm)
            label {{t("mods-search")}}
          }
        }
        if move || notice.get().is_some() {
          label(class:success = move || matches!(notice.get(), Some(Ok(_))),
                class:error = move || matches!(notice.get(), Some(Err(_)))) {
            {move || match notice.get() { Some(Ok(m) | Err(m)) => m, None => String::new() }}
          }
        }

        if move || searching.get() {
          column(class = "list") {
            Skeleton(class = "skeleton-row")
            Skeleton(class = "skeleton-row")
            Skeleton(class = "skeleton-row")
          }
        } else {
          if move || results().is_some() {
            label(class = "section-title") {{t("mods-results")}}
            if move || matches!(results(), Some(Err(_))) {
              label(class = "error") {{move || match results() { Some(Err(e)) => e, _ => String::new() }}}
            } else {
              if move || matches!(results(), Some(Ok(ref h)) if h.is_empty()) {
                label(class = "muted") {{t("mods-no-results")}}
              } else {
                column(class = "list") {
                  for hit in move || results().and_then(Result::ok).unwrap_or_default(), key = |h: &SearchHit| h.project_id.clone() {
                    ResultRow(m = m, hit = hit)
                  }
                }
              }
            }
          }
        }

        label(class = "section-title") {{t("mods-installed")}}
        if move || m.installed().is_empty() {
          Empty {
            EmptyHeader {
              EmptyTitle {{t("mods-empty-title")}}
              EmptyDescription {{t("mods-empty-hint")}}
            }
          }
        } else {
          column(class = "list") {
            for item in move || m.installed(), key = |i: &InstalledMod| i.project_id.clone() {
              InstalledRow(m = m, item = item)
            }
          }
        }
      }
    }
  }
}

#[component]
fn ResultRow(m: Mods, hit: SearchHit) -> impl IntoView {
  // `StoredValue` is `Copy`, so the closures below can be rebuilt each time the row's branch re-renders.
  let id = StoredValue::new(hit.project_id.clone());
  let install_label = StoredValue::new(format!("{} {}", t("mods-install"), hit.title));
  let working = move || m.busy.get().is_some_and(|b| id.with_value(|i| b == *i));
  view! {
    row(class = "mod-row") {
      column(class = "mod-row__text") {
        label(class = "mod-row__title") {{hit.title.clone()}}
        label(class = "mod-row__meta") {{format!("{} · {}", tr("mods-by", &[("author", hit.author.clone())]), hit.description)}}
      }
      if move || !id.with_value(|i| m.is_installed(i)) {
        Button(
          size = ButtonSize::Sm,
          disabled = Signal::derive_local(working),
          a11y:label = install_label.get_value(),
          on:click = move |_| m.install(id.get_value())
        ) {
          Icon(icon = icons::DOWNLOAD, size = IconSize::Sm)
          label {{move || if working() { t("mods-working") } else { t("mods-install") }}}
        }
      }
    }
  }
}

#[component]
fn InstalledRow(m: Mods, item: InstalledMod) -> impl IntoView {
  let meta = if item.explicit { item.file.clone() } else { format!("{} · {}", t("mods-dependency"), item.file) };
  let id = item.project_id.clone();
  view! {
    row(class = "mod-row") {
      column(class = "mod-row__text") {
        label(class = "mod-row__title") {{item.title.clone()}}
        label(class = "mod-row__meta") {{meta}}
      }
      Button(
        variant = ButtonVariant::Ghost,
        size = ButtonSize::Sm,
        a11y:label = format!("{} {}", t("mods-remove"), item.title),
        on:click = move |_| m.remove(&id)
      ) {
        Icon(icon = icons::TRASH, size = IconSize::Sm)
        label {{t("mods-remove")}}
      }
    }
  }
}
