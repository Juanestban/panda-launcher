// No console window behind the launcher on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod i18n;
mod icons;
mod style;
mod ui;

use std::sync::Arc;
use zgui::prelude::*;

const GEIST: &[u8] = include_bytes!("../assets/fonts/Geist-Variable.ttf");
const GEIST_MONO: &[u8] = include_bytes!("../assets/fonts/GeistMono-Variable.ttf");

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // reqwest needs a tokio runtime; zgui routes `background` work onto it.
  let _tokio = zgui::tokio::install()?;

  // Only our own faces, so the launcher looks the same on every machine.
  let fonts = Fonts::shipped_only();
  fonts.register(Arc::new(GEIST), Some("Geist"))?;
  fonts.register(Arc::new(GEIST_MONO), Some("Geist Mono"))?;

  app()
    .with_application_id("dev.panda.Launcher")
    .with_title("Panda Launcher")
    .with_size(1100.0, 720.0)
    .with_min_size(900.0, 600.0)
    .with_fonts(fonts)
    .with_stylesheet(style::SHEET)
    .run(|| view! { ui::App() })?;
  Ok(())
}
