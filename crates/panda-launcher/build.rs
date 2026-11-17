//! Windows only: put the panda icon into the exe so Explorer and shortcuts show it.

fn main() {
  println!("cargo::rerun-if-changed=assets/icon/panda.ico");
  #[cfg(windows)]
  winresource::WindowsResource::new().set_icon("assets/icon/panda.ico").compile().expect("embed the exe icon");
}
