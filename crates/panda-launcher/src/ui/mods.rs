//! Mods: browse Modrinth for the selected version, open a mod's page, manage what is installed.

// `view!` only takes `move ||` closures in `if` and `for`, which clippy calls redundant here.
#![allow(clippy::redundant_closure)]

use super::format::compact;
use super::mod_page::ModPageProps;
use super::state::{self, AppState, core};
use crate::i18n::{t, tr};
use crate::icons;
use panda_core::mods::{self, InstalledMod, SearchHit};
use zgui::prelude::*;
use zgui_ui::prelude::*;
use zgui_ui_icons::prelude::*;

#[component]
pub fn ModsPage(state: AppState) -> impl IntoView {
  view! {
    // Keyed on the open id so each mod page is built fresh.
    for id in move || state.mod_open.get().into_iter().collect::<Vec<_>>(), key = |id: &String| id.clone() {
      ModPage(state = state, id = id)
    }
    if move || state.mod_open.get().is_none() {
      Browse(state = state)
    }
  }
}

/// Search results for one version and query; "Load more" appends the next page.
#[derive(Clone, Debug, PartialEq)]
struct Found {
  game: String,
  query: String,
  hits: Vec<SearchHit>,
  total: usize,
}

#[component]
fn Browse(state: AppState) -> impl IntoView {
  let query = RwSignal::new_local(String::new());
  let found = RwSignal::new(None::<Result<Found, String>>);
  let loading = RwSignal::new(false);

  let search = move |more: bool| {
    let game = state.version.get_untracked();
    if game.is_empty() {
      return;
    }
    let previous = found.get_untracked().and_then(Result::ok).filter(|f| more && f.game == game);
    let text = previous.as_ref().map_or_else(|| query.get_untracked(), |f| f.query.clone());
    let page = previous.as_ref().map_or(0, |f| f.hits.len() / 20);
    loading.set(true);
    spawn(async move {
      let r = background({
        let (game, text) = (game.clone(), text.clone());
        async move { mods::search(&core().http, &game, &text, page).await }
      })
      .await;
      found.set(Some(match r {
        Ok(p) => {
          let mut hits = previous.map(|f| f.hits).unwrap_or_default();
          hits.extend(p.hits);
          Ok(Found { game, query: text, hits, total: p.total_hits })
        }
        Err(e) => Err(e.to_string()),
      }));
      loading.set(false);
    });
  };
  // Popular mods for the selected version until the player searches; again whenever the version changes.
  let current = move || found.get().and_then(Result::ok).filter(|f| f.game == state.version.get());
  let watching = zgui::reactive::RenderEffect::new(move |_| {
    state.version.track();
    search(false);
  });
  on_cleanup_local(move || drop(watching));
  let installed = move || state::installed_mods(state);

  view! {
    column(class = "page-header") {
      label(class = "eyebrow") {{move || tr("mods-eyebrow", &[("version", state.version.get())])}}
      label(class = "heading-4") {{t("mods-title")}}
      label(class = "body-6 muted") {{t("mods-subtitle")}}
    }
    ScrollArea(class = "scroll") {
      column(class = "stack") {
        row(class = "search-row") {
          Input(class = "search-row__input", value = query, placeholder = t("mods-search-placeholder"),
                a11y:label = t("mods-search-placeholder"))
          Button(disabled = Signal::derive_local(move || loading.get()), on:click = move |_| search(false)) {
            Icon(icon = icons::SEARCH, size = IconSize::Sm)
            label {{t("mods-search")}}
          }
        }
        Notice(state = state)

        if move || !installed().is_empty() {
          column(class = "list") {
            row(class = "section-head") {
              label(class = "heading-6 section-head__title") {{t("mods-installed")}}
              label(class = "caption-4 muted") {{move || installed().len().to_string()}}
            }
            for item in move || installed(), key = |i: &InstalledMod| i.project_id.clone() {
              InstalledRow(state = state, item = item)
            }
          }
        }

        column(class = "list") {
          row(class = "section-head") {
            label(class = "heading-6 section-head__title") {
              {move || if current().is_some_and(|f| !f.query.is_empty()) { t("mods-results") } else { t("mods-popular") }}
            }
            label(class = "caption-4 muted") {{move || current().map(|f| tr("mods-count", &[("count", compact(f.total as u64))])).unwrap_or_default()}}
          }
          if move || matches!(found.get(), Some(Err(_))) {
            label(class = "error") {{move || match found.get() { Some(Err(e)) => e, _ => String::new() }}}
          }
          if move || current().is_some_and(|f| f.hits.is_empty()) {
            Empty {
              EmptyHeader {
                EmptyTitle {{t("mods-no-results")}}
                EmptyDescription {{t("mods-no-results-hint")}}
              }
            }
          }
          row(class = "grid") {
            for hit in move || current().map(|f| f.hits).unwrap_or_default(), key = |h: &SearchHit| h.project_id.clone() {
              ModCard(state = state, hit = hit)
            }
            if move || loading.get() {
              Skeleton(class = "skeleton-card")
              Skeleton(class = "skeleton-card")
              Skeleton(class = "skeleton-card")
            }
          }
          if move || current().is_some_and(|f| f.hits.len() < f.total) && !loading.get() {
            row {
              Button(variant = ButtonVariant::Secondary, on:click = move |_| search(true)) {
                label {{t("mods-load-more")}}
              }
            }
          }
        }
      }
    }
  }
}

/// The outcome of the last install or removal, on whichever mods screen is open.
#[component]
pub fn Notice(state: AppState) -> impl IntoView {
  let notice = state.mods_notice;
  view! {
    if move || matches!(notice.get(), Some(Ok(_))) {
      Alert {
        AlertDescription {{move || match notice.get() { Some(Ok(m)) => m, _ => String::new() }}}
      }
    }
    if move || matches!(notice.get(), Some(Err(_))) {
      Alert(variant = AlertVariant::Destructive) {
        AlertDescription {{move || match notice.get() { Some(Err(m)) => m, _ => String::new() }}}
      }
    }
  }
}

/// The mod's icon from Modrinth, cached on disk; a puzzle piece until it arrives or when there is none.
#[component]
pub fn ModIcon(project_id: String, url: Option<String>, #[prop(into)] size: String) -> impl IntoView {
  let path = RwSignal::new(None::<String>);
  if let Some(url) = url {
    spawn(async move {
      if let Ok(p) = background(async move { mods::icon(&core().http, &core().dirs, &project_id, &url).await }).await {
        // The card may be gone by now; then there is nothing to update.
        let _ = path.try_set(Some(p.to_string_lossy().into_owned()));
      }
    });
  }
  view! {
    box(class = format!("mod-icon mod-icon--{size}")) {
      if move || path.get().is_some() {
        image(class = "mod-icon__img", src = move || path.get(), alt = "")
      } else {
        Icon(icon = icons::PUZZLE, size = IconSize::Sm)
      }
    }
  }
}

#[component]
fn ModCard(state: AppState, hit: SearchHit) -> impl IntoView {
  let id = StoredValue::new(hit.project_id.clone());
  let open = move || state.mod_open.set(Some(id.get_value()));
  view! {
    control(class = "mod-card", tabindex = Focus::Sequential, a11y:label = hit.title.clone(), on:click = move |_| open()) {
      row(class = "mod-card__head") {
        ModIcon(project_id = hit.project_id.clone(), url = hit.icon_url.clone(), size = "md")
        column(class = "mod-card__title") {
          label(class = "title-3") {{hit.title.clone()}}
          label(class = "caption-5 muted") {{tr("mods-by", &[("author", hit.author.clone())])}}
        }
      }
      label(class = "body-6 mod-card__desc") {{hit.description.clone()}}
      row(class = "mod-card__foot") {
        row(class = "mod-card__meta") {
          Icon(icon = icons::DOWNLOAD, size = IconSize::Sm)
          label(class = "caption-4") {{compact(hit.downloads)}}
        }
        InstallButton(state = state, project = hit.project_id.clone())
      }
    }
  }
}

/// "Install", "Working…" or an "Installed" badge for the newest build.
#[component]
pub fn InstallButton(state: AppState, project: String) -> impl IntoView {
  let id = StoredValue::new(project);
  let installed = move || id.with_value(|p| state::installed_mods(state).iter().any(|m| m.project_id == *p));
  let working = move || state.mods_busy.get().is_some_and(|b| id.with_value(|p| b == *p));
  view! {
    if move || installed() {
      Badge(variant = BadgeVariant::Secondary) {{t("mods-installed-badge")}}
    } else {
      Button(
        size = ButtonSize::Sm,
        disabled = Signal::derive_local(move || state.mods_busy.get().is_some()),
        on:click = move |e| {
          e.stop_propagation();
          state::install_mod(state, id.get_value(), None);
        }
      ) {
        label {{move || if working() { t("mods-working") } else { t("mods-install") }}}
      }
    }
  }
}

#[component]
fn InstalledRow(state: AppState, item: InstalledMod) -> impl IntoView {
  let meta = if item.explicit { item.version_number.clone() } else { format!("{} · {}", t("mods-dependency"), item.version_number) };
  let id = StoredValue::new(item.project_id.clone());
  view! {
    control(class = "mod-row", tabindex = Focus::Sequential, a11y:label = item.title.clone(),
            on:click = move |_| state.mod_open.set(Some(id.get_value()))) {
      ModIcon(project_id = item.project_id.clone(), url = item.icon_url.clone(), size = "sm")
      column(class = "mod-row__text") {
        label(class = "title-4") {{item.title.clone()}}
        label(class = "caption-5 muted") {{meta}}
      }
      Button(
        variant = ButtonVariant::Ghost,
        size = ButtonSize::Sm,
        a11y:label = format!("{} {}", t("mods-remove"), item.title),
        on:click = move |e| {
          e.stop_propagation();
          id.with_value(|p| state::remove_mod(state, p));
        }
      ) {
        Icon(icon = icons::TRASH, size = IconSize::Sm)
        label {{t("mods-remove")}}
      }
    }
  }
}
