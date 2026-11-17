use panda_core::account::{self, Resume};
use panda_core::launch::{Launch, Player};
use panda_core::version;
use panda_core::{auth, fabric, java, net, paths::Dirs};

// shortcut: console front-end to exercise panda-core, replace with the zgui UI in Phase 5.
// Usage: panda-launcher [minecraft-version]   (defaults to the latest release)
#[tokio::main]
async fn main() {
  if let Err(e) = run().await {
    eprintln!("error: {e}");
    std::process::exit(1);
  }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
  let http = net::client();
  let dirs = Dirs::platform();
  let wanted = std::env::args().nth(1);

  let player = match account::resume(&http).await? {
    Resume::Online(s) => player(&s.profile, s.access_token),
    // Offline play only for an account whose ownership was verified before (ADR 0004).
    Resume::Offline(a) => player(&a.profile, "0".into()),
    Resume::SignedOut => {
      let code = auth::device_code(&http).await?;
      println!("Open {} and enter the code {}", code.verification_uri, code.user_code);
      let session = auth::sign_in(&http, auth::poll(&http, &code).await?).await?;
      account::save(&session)?;
      player(&session.profile, session.access_token)
    }
  };
  println!("Playing as {}", player.name);

  let progress = |done: usize, total: usize| {
    if done == total || done.is_multiple_of(500) {
      println!("  {done}/{total}");
    }
  };
  let v = match version::manifest(&http).await {
    Ok(manifest) => {
      let id = wanted.unwrap_or_else(|| manifest.latest.release.clone());
      println!("Installing Minecraft {id} with Fabric");
      let vanilla = version::install(&http, &dirs, manifest.find(&id)?, &progress).await?;
      fabric::install(&http, &dirs, vanilla, &progress).await?
    }
    // No network: use the profile already installed.
    Err(e) => fabric::load(&dirs, &wanted.ok_or(e)?)?,
  };
  let java = java::ensure(&http, &dirs, &v.java_version, &progress).await?;

  let game_dir = dirs.profile(&fabric::profile_name(&v.id));
  let launch = Launch { version: &v, java: &java, game_dir: &game_dir, player: &player, max_memory_mb: 4096 };
  let status = launch.spawn(&dirs)?.wait()?;
  println!("Minecraft exited ({status}); log in {}", dirs.logs().display());
  Ok(())
}

fn player(profile: &auth::Profile, access_token: String) -> Player {
  Player { name: profile.name.clone(), uuid: profile.id.clone(), access_token }
}
