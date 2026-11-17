use panda_core::account::{self, Resume};
use panda_core::launch::{Launch, Player};
use panda_core::version;
use panda_core::{auth, fabric, java, mods, net, paths::Dirs};

// shortcut: console front-end to exercise panda-core, replace with the zgui UI in Phase 5.
// Usage: panda-launcher [minecraft-version]   (defaults to the latest release)
//        panda-launcher mods <version> search <text> | add <project> | remove <project> | list
#[tokio::main]
async fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  let result = match args.first().map(String::as_str) {
    Some("mods") => mods_command(&args[1..]).await,
    _ => run(args.first().cloned()).await,
  };
  if let Err(e) = result {
    eprintln!("error: {e}");
    std::process::exit(1);
  }
}

async fn mods_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
  let (http, dirs) = (net::client(), Dirs::platform());
  let arg = |i: usize| args.get(i).map(String::as_str).ok_or("usage: panda-launcher mods <version> search|add|remove|list [arg]");
  let game = arg(0)?;
  match arg(1)? {
    "search" => {
      for hit in mods::search(&http, game, arg(2)?, 0).await?.hits {
        println!("{:<10} {:<30} {}", hit.project_id, hit.title, hit.description);
      }
    }
    "add" => println!("Installed: {}", mods::install(&http, &dirs, game, arg(2)?).await?.join(", ")),
    "remove" => println!("Removed: {}", mods::uninstall(&dirs, game, arg(2)?, false)?.join(", ")),
    "list" => {
      for m in mods::list(&dirs, game)?.mods {
        println!("{:<10} {:<30} {}", m.project_id, m.title, if m.explicit { "" } else { "(dependency)" });
      }
    }
    other => return Err(format!("unknown mods command {other}").into()),
  }
  Ok(())
}

async fn run(wanted: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
  let http = net::client();
  let dirs = Dirs::platform();

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
