//! Spanish and English strings with Fluent (ADR 0011).

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
  Es,
  En,
}

impl Lang {
  /// Spanish when the OS locale is Spanish, English otherwise.
  pub fn system() -> Self {
    match sys_locale::get_locale() {
      Some(l) if l.to_lowercase().starts_with("es") => Lang::Es,
      _ => Lang::En,
    }
  }

  fn source(self) -> &'static str {
    match self {
      Lang::Es => include_str!("../locales/es.ftl"),
      Lang::En => include_str!("../locales/en.ftl"),
    }
  }

  fn tag(self) -> &'static str {
    match self {
      Lang::Es => "es",
      Lang::En => "en",
    }
  }
}

fn bundle(lang: Lang) -> FluentBundle<FluentResource> {
  let mut b = FluentBundle::new(vec![lang.tag().parse().expect("valid language tag")]);
  // No Unicode isolation marks: they show up as boxes in a native text renderer.
  b.set_use_isolating(false);
  let resource = FluentResource::try_new(lang.source().to_owned()).expect("locale file parses");
  b.add_resource(resource).expect("no duplicate keys");
  b
}

thread_local! {
  static BUNDLE: RefCell<FluentBundle<FluentResource>> = RefCell::new(bundle(Lang::system()));
}

/// Switch language; the UI rebuilds its tree afterwards.
#[allow(dead_code)] // wired to the Settings screen in the next increment
pub fn set_lang(lang: Lang) {
  BUNDLE.with(|b| *b.borrow_mut() = bundle(lang));
}

/// The string for `key`.
pub fn t(key: &str) -> String {
  tr(key, &[])
}

/// The string for `key` with `{ $name }` placeholders filled.
pub fn tr(key: &str, args: &[(&str, String)]) -> String {
  BUNDLE.with(|b| {
    let b = b.borrow();
    let Some(pattern) = b.get_message(key).and_then(|m| m.value()) else {
      return key.to_owned();
    };
    let mut fluent_args = FluentArgs::new();
    for (name, value) in args {
      fluent_args.set(*name, value.clone());
    }
    b.format_pattern(pattern, Some(&fluent_args), &mut Vec::new()).into_owned()
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn keys(lang: Lang) -> Vec<&'static str> {
    let mut keys: Vec<_> = lang.source().lines().filter_map(|l| l.split_once(" = ").map(|(k, _)| k)).collect();
    keys.sort();
    keys
  }

  #[test]
  fn both_languages_have_the_same_keys() {
    assert_eq!(keys(Lang::Es), keys(Lang::En));
    bundle(Lang::Es);
    bundle(Lang::En);
  }

  #[test]
  fn placeholders_are_filled() {
    set_lang(Lang::En);
    assert_eq!(tr("play-files", &[("done", "3".into()), ("total", "10".into())]), "3 of 10 files");
    set_lang(Lang::Es);
    assert_eq!(t("nav-play"), "Jugar");
    assert_eq!(t("missing-key"), "missing-key");
  }
}
