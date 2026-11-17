// No console window behind the launcher on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod i18n;
mod icons;
mod style;
mod ui;

use panda_core::update;
use std::fs::{File, TryLockError};
use std::sync::Arc;
use std::time::Duration;
use zgui::prelude::*;

const GEIST: &[u8] = include_bytes!("../assets/fonts/Geist-Variable.ttf");
const GEIST_MONO: &[u8] = include_bytes!("../assets/fonts/GeistMono-Variable.ttf");

fn main() -> Result<(), Box<dyn std::error::Error>> {
  update::clean_up();
  let after_update = std::env::args().any(|a| a == update::AFTER_UPDATE_ARG);
  // Two launchers would race on the same downloads; a second start just leaves.
  let Some(_instance) = lock_instance(after_update)? else { return Ok(()) };

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
    .with_icon(window_icon()?)
    .with_fonts(fonts)
    .with_stylesheet(style::SHEET)
    .run(|| view! { ui::App() })?;
  Ok(())
}

/// Holds `launcher.lock` for the life of the process. Right after an update the old version may still be
/// closing, so the new one waits up to 15 s for it.
fn lock_instance(after_update: bool) -> std::io::Result<Option<File>> {
  let root = panda_core::paths::Dirs::platform().root;
  std::fs::create_dir_all(&root)?;
  let file = File::create(root.join("launcher.lock"))?;
  let attempts = if after_update { 60 } else { 1 };
  for attempt in 1..=attempts {
    match file.try_lock() {
      Ok(()) => return Ok(Some(file)),
      Err(TryLockError::WouldBlock) if attempt < attempts => std::thread::sleep(Duration::from_millis(250)),
      Err(TryLockError::WouldBlock) => return Ok(None),
      // A filesystem without locks: run anyway rather than never start.
      Err(TryLockError::Error(_)) => return Ok(Some(file)),
    }
  }
  Ok(None)
}

/// The panda for the title bar and taskbar (macOS and Wayland take it from the bundle or desktop entry instead).
fn window_icon() -> Result<WindowIcon, Box<dyn std::error::Error>> {
  let decoded = zgui_image::decode(include_bytes!("../assets/icon/panda-128.png"), zgui_image::Limits::default())?;
  // The decoder hands out premultiplied texels; the window icon wants straight alpha.
  let mut rgba = decoded.texels.to_vec();
  for px in rgba.as_chunks_mut::<4>().0 {
    let alpha = u16::from(px[3]);
    for c in &mut px[..3] {
      // Fully transparent pixels stay as they are.
      if let Some(straight) = (u16::from(*c) * 255).checked_div(alpha) {
        *c = straight.min(255) as u8;
      }
    }
  }
  Ok(WindowIcon::from_rgba(rgba, decoded.size.width, decoded.size.height)?)
}
