//! About: version, licences and the "not official" disclaimer (ADR 0012).

use crate::i18n::{t, tr};
use zgui::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
  view! {
    column(class = "page-header") {
      label(class = "page-title") {{t("about-title")}}
      label(class = "page-subtitle") {{tr("about-version", &[("version", env!("CARGO_PKG_VERSION").into())])}}
    }
    column(class = "panel about") {
      label(class = "warning") {{t("about-disclaimer")}}
      label(class = "muted") {{t("about-licenses")}}
      label(class = "muted") {{t("about-source")}}
    }
  }
}
