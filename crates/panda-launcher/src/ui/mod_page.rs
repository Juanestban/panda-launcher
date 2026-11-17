//! One mod's page inside the launcher: details, description, and every Fabric build for the selected version.

// `view!` only takes `move ||` closures in `if` and `for`, which clippy calls redundant here.
#![allow(clippy::redundant_closure)]

use super::format::{compact, date};
use super::markdown::{self, Block};
use super::mods::{InstallButtonProps, ModIconProps, NoticeProps};
use super::state::{self, AppState, core};
use crate::i18n::{t, tr};
use crate::icons;
use panda_core::mods::{self, ModVersion, Project};
use zgui::prelude::*;
use zgui_ui::prelude::*;
use zgui_ui_icons::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
  About,
  Versions,
}

#[component]
pub fn ModPage(state: AppState, id: String) -> impl IntoView {
  let project = RwSignal::new(None::<Result<Project, String>>);
  let builds = RwSignal::new(None::<Result<Vec<ModVersion>, String>>);
  {
    let id = id.clone();
    spawn(async move {
      let r = background(async move { mods::project(&core().http, &id).await }).await;
      let _ = project.try_set(Some(r.map_err(|e| e.to_string())));
    });
  }
  // Builds follow the version picked on the Play page.
  let watching = zgui::reactive::RenderEffect::new(move |_| {
    let (id, game) = (id.clone(), state.version.get());
    builds.set(None);
    spawn(async move {
      let r = background(async move { mods::versions(&core().http, &id, &game).await }).await;
      let _ = builds.try_set(Some(r.map_err(|e| e.to_string())));
    });
  });
  on_cleanup_local(move || drop(watching));

  view! {
    row {
      Button(variant = ButtonVariant::Ghost, size = ButtonSize::Sm, on:click = move |_| state.mod_open.set(None)) {
        label {{t("mods-back")}}
      }
    }
    if move || project.get().is_none() {
      column(class = "list") {
        Skeleton(class = "skeleton-card")
        Skeleton(class = "skeleton-row")
      }
    }
    if move || matches!(project.get(), Some(Err(_))) {
      label(class = "error") {{move || match project.get() { Some(Err(e)) => e, _ => String::new() }}}
    }
    for p in move || project.get().and_then(Result::ok).into_iter().collect::<Vec<_>>(), key = |p: &Project| p.id.clone() {
      Details(state = state, project = p, builds = builds)
    }
  }
}

#[component]
fn Details(state: AppState, project: Project, builds: RwSignal<Option<Result<Vec<ModVersion>, String>>>) -> impl IntoView {
  let tab = RwSignal::new(Tab::About);
  let blocks = StoredValue::new(markdown::parse(&project.body).into_iter().enumerate().collect::<Vec<(usize, Block)>>());
  let categories = project.categories.clone();
  let links: Vec<(String, String)> = [
    (t("mods-link-source"), project.source_url.clone()),
    (t("mods-link-wiki"), project.wiki_url.clone()),
    (t("mods-link-issues"), project.issues_url.clone()),
    (t("mods-link-discord"), project.discord_url.clone()),
    (t("mods-link-modrinth"), Some(format!("https://modrinth.com/mod/{}", project.slug))),
  ]
  .into_iter()
  .filter_map(|(name, url)| Some((name, url.filter(|u| u.starts_with("https://"))?)))
  .collect();
  // Custom licences arrive as "LicenseRef-Polyform-Shield-1.0.0" with an empty name.
  let license = project
    .license
    .as_ref()
    .map(|l| if l.name.is_empty() || l.name == "Custom" { l.id.trim_start_matches("LicenseRef-").replace('-', " ") } else { l.name.clone() })
    .unwrap_or_default();
  let pid = project.id.clone();

  view! {
    ScrollArea(class = "scroll") {
      column(class = "stack") {
        row(class = "detail-head") {
          ModIcon(project_id = project.id.clone(), url = project.icon_url.clone(), size = "lg")
          column(class = "detail-head__text") {
            label(class = "heading-4") {{project.title.clone()}}
            label(class = "body-3 muted") {{project.description.clone()}}
            row(class = "chips") {
              for c in move || categories.clone(), key = |c: &String| c.clone() {
                Badge(variant = BadgeVariant::Outline) {{c.clone()}}
              }
            }
          }
          InstallButton(state = state, project = pid.clone())
        }
        Notice(state = state)

        row(class = "stats") {
          column(class = "stat") {
            label(class = "stat__value") {{compact(project.downloads)}}
            label(class = "caption-4 muted") {{t("mods-stat-downloads")}}
          }
          column(class = "stat") {
            label(class = "stat__value") {{compact(project.followers)}}
            label(class = "caption-4 muted") {{t("mods-stat-followers")}}
          }
          column(class = "stat") {
            label(class = "stat__value") {{date(&project.updated).to_owned()}}
            label(class = "caption-4 muted") {{t("mods-stat-updated")}}
          }
          column(class = "stat") {
            label(class = "stat__value") {{if license.is_empty() { "—".to_owned() } else { license.clone() }}}
            label(class = "caption-4 muted") {{t("mods-stat-license")}}
          }
        }

        row(class = "links") {
          for link in move || links.clone(), key = |l: &(String, String)| l.1.clone() {
            Button(variant = ButtonVariant::Outline, size = ButtonSize::Sm, on:click = { let url = link.1.clone(); move |_| state::open_url(&url) }) {
              label {{link.0.clone()}}
            }
          }
        }

        row(class = "tabs", a11y:role = Role::TabList) {
          TabButton(tab = tab, value = Tab::About, label = t("mods-tab-about"))
          TabButton(tab = tab, value = Tab::Versions, label = t("mods-tab-versions"))
        }

        if move || tab.get() == Tab::About {
          column(class = "md") {
            for block in move || blocks.get_value(), key = |b: &(usize, Block)| b.0 {
              label(class = block.1.class()) {{block.1.text()}}
            }
          }
        }
        if move || tab.get() == Tab::Versions {
          Versions(state = state, builds = builds)
        }
      }
    }
  }
}

#[component]
fn TabButton(tab: RwSignal<Tab>, value: Tab, #[prop(into)] label: String) -> impl IntoView {
  view! {
    control(class = "tab", class:is-active = move || tab.get() == value, tabindex = Focus::Sequential,
            a11y:role = Role::Tab, a11y:label = label.clone(), on:click = move |_| tab.set(value)) {
      label {{label.clone()}}
    }
  }
}

#[component]
fn Versions(state: AppState, builds: RwSignal<Option<Result<Vec<ModVersion>, String>>>) -> impl IntoView {
  view! {
    column(class = "list") {
      label(class = "body-6 muted") {{move || tr("mods-versions-for", &[("version", state.version.get())])}}
      if move || builds.get().is_none() {
        Skeleton(class = "skeleton-row")
        Skeleton(class = "skeleton-row")
        Skeleton(class = "skeleton-row")
      }
      if move || matches!(builds.get(), Some(Err(_))) {
        label(class = "error") {{move || match builds.get() { Some(Err(e)) => e, _ => String::new() }}}
      }
      if move || matches!(builds.get(), Some(Ok(ref v)) if v.is_empty()) {
        Empty {
          EmptyHeader {
            EmptyTitle {{t("mods-versions-empty")}}
            EmptyDescription {{t("mods-versions-empty-hint")}}
          }
        }
      }
      for v in move || builds.get().and_then(Result::ok).unwrap_or_default(), key = |v: &ModVersion| v.id.clone() {
        VersionRow(state = state, release = v)
      }
    }
  }
}

#[component]
fn VersionRow(state: AppState, release: ModVersion) -> impl IntoView {
  let ids = StoredValue::new((release.project_id.clone(), release.id.clone()));
  let installed_build = move || ids.with_value(|(p, _)| state::installed_mods(state).into_iter().find(|m| m.project_id == *p).map(|m| m.version_id));
  let is_this = move || ids.with_value(|(_, v)| installed_build().as_deref() == Some(v.as_str()));
  let working = move || ids.with_value(|(p, _)| state.mods_busy.get().as_deref() == Some(p.as_str()));
  let kind = match release.version_type.as_str() {
    "release" => t("mods-release"),
    "beta" => "Beta".to_owned(),
    _ => "Alpha".to_owned(),
  };
  let stable = release.version_type == "release";
  view! {
    row(class = "version-row", class:is-installed = is_this) {
      column(class = "version-row__text") {
        label(class = "title-4") {{release.version_number.clone()}}
        label(class = "caption-5 muted") {{format!("{} · {} · {}", release.name, date(&release.date_published), tr("mods-version-downloads", &[("count", compact(release.downloads))]))}}
      }
      Badge(variant = if stable { BadgeVariant::Secondary } else { BadgeVariant::Outline }) {{kind}}
      if move || is_this() {
        Badge(variant = BadgeVariant::Default) {{t("mods-installed-badge")}}
      } else {
        Button(
          size = ButtonSize::Sm,
          variant = if installed_build().is_some() { ButtonVariant::Secondary } else { ButtonVariant::Default },
          disabled = Signal::derive_local(move || state.mods_busy.get().is_some()),
          on:click = move |_| ids.with_value(|(p, v)| state::install_mod(state, p.clone(), Some(v.clone())))
        ) {
          Icon(icon = icons::DOWNLOAD, size = IconSize::Sm)
          label {{move || if working() { t("mods-working") } else if installed_build().is_some() { t("mods-switch") } else { t("mods-install") }}}
        }
      }
    }
  }
}
