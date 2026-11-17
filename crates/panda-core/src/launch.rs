//! Build the Java command line and start the game.

use crate::Result;
use crate::paths::Dirs;
use crate::version::{Arg, OneOrMany, Version, allowed};
use std::collections::HashMap;
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Who is playing. Only built from a verified account (ADR 0004).
pub struct Player {
  pub name: String,
  pub uuid: String,
  /// Minecraft access token; `"0"` when playing offline. Never logged.
  pub access_token: String,
}

pub struct Launch<'a> {
  pub version: &'a Version,
  pub java: &'a Path,
  /// The profile folder (ADR 0006), passed as `--gameDir`.
  pub game_dir: &'a Path,
  pub player: &'a Player,
  pub max_memory_mb: u32,
}

impl Launch<'_> {
  /// Full argument list after `java`.
  pub fn args(&self, dirs: &Dirs) -> Vec<String> {
    let v = self.version;
    let sep = if cfg!(windows) { ";" } else { ":" };
    let path = |p: &Path| p.display().to_string();
    let classpath: Vec<String> = v.classpath(dirs).iter().map(|p| path(p)).collect();
    let vars: HashMap<&str, String> = HashMap::from([
      ("natives_directory", path(&dirs.natives(&v.id))),
      ("launcher_name", "panda-launcher".into()),
      ("launcher_version", env!("CARGO_PKG_VERSION").into()),
      ("classpath", classpath.join(sep)),
      ("classpath_separator", sep.into()),
      ("library_directory", path(&dirs.libraries())),
      ("version_name", v.id.clone()),
      ("version_type", v.kind.clone()),
      ("game_directory", path(self.game_dir)),
      ("assets_root", path(&dirs.assets())),
      ("assets_index_name", v.asset_index.id.clone()),
      ("auth_player_name", self.player.name.clone()),
      ("auth_uuid", self.player.uuid.clone()),
      ("auth_access_token", self.player.access_token.clone()),
      ("auth_xuid", "0".into()),
      ("clientid", "0".into()),
      ("user_type", "msa".into()),
      ("user_properties", "{}".into()),
    ]);

    let (jvm, game) = v.arguments.as_ref().map(|a| (&a.jvm[..], &a.game[..])).unwrap_or_default();
    let mut out = vec![format!("-Xmx{}M", self.max_memory_mb)];
    out.extend(expand(jvm, &vars));
    out.push(v.main_class.clone());
    out.extend(expand(game, &vars));
    out
  }

  /// Start the game; its output goes to `logs/<version>.log`.
  pub fn spawn(&self, dirs: &Dirs) -> Result<Child> {
    std::fs::create_dir_all(self.game_dir)?;
    std::fs::create_dir_all(dirs.logs())?;
    let log = std::fs::File::create(dirs.logs().join(format!("{}.log", self.version.id)))?;
    Ok(Command::new(self.java).args(self.args(dirs)).current_dir(self.game_dir).stdout(log.try_clone()?).stderr(log).stdin(Stdio::null()).spawn()?)
  }
}

fn expand<'a>(args: &'a [Arg], vars: &'a HashMap<&str, String>) -> impl Iterator<Item = String> + 'a {
  args
    .iter()
    .flat_map(|a| match a {
      Arg::Plain(s) => vec![s.as_str()],
      Arg::Ruled { rules, value } if allowed(rules) => match value {
        OneOrMany::One(s) => vec![s.as_str()],
        OneOrMany::Many(v) => v.iter().map(String::as_str).collect(),
      },
      Arg::Ruled { .. } => vec![],
    })
    .map(|s| fill(s, vars))
}

/// Replace `${name}` placeholders; unknown ones are left as they are.
fn fill(s: &str, vars: &HashMap<&str, String>) -> String {
  let mut out = String::new();
  let mut rest = s;
  while let Some(start) = rest.find("${") {
    let Some(len) = rest[start..].find('}') else { break };
    let key = &rest[start + 2..start + len];
    out.push_str(&rest[..start]);
    match vars.get(key) {
      Some(v) => out.push_str(v),
      None => out.push_str(&rest[start..=start + len]),
    }
    rest = &rest[start + len + 1..];
  }
  out + rest
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fill_replaces_known_placeholders_only() {
    let vars = HashMap::from([("a", "1".to_string()), ("natives_directory", "/n".to_string())]);
    assert_eq!(fill("-Djava.library.path=${natives_directory}", &vars), "-Djava.library.path=/n");
    assert_eq!(fill("${a}-${b}-${a}", &vars), "1-${b}-1");
    assert_eq!(fill("no vars", &vars), "no vars");
    assert_eq!(fill("broken ${a", &vars), "broken ${a");
  }

  #[test]
  fn args_put_main_class_between_jvm_and_game_args() {
    let version: Version = serde_json::from_str(
      r#"{"id":"1.21.10","type":"release","mainClass":"net.minecraft.client.main.Main",
          "arguments":{"game":["--username","${auth_player_name}","--gameDir","${game_directory}",
                               {"rules":[{"action":"allow","features":{"is_demo_user":true}}],"value":"--demo"}],
                       "jvm":["-cp","${classpath}"]},
          "assetIndex":{"id":"27","sha1":"x","size":1,"url":"u"},
          "downloads":{"client":{"sha1":"x","size":1,"url":"u"}},
          "libraries":[],"javaVersion":{"component":"java-runtime-delta","majorVersion":21}}"#,
    )
    .unwrap();
    let dirs = Dirs { root: "/data".into() };
    let player = Player { name: "Steve".into(), uuid: "u".into(), access_token: "t".into() };
    let launch = Launch { version: &version, java: Path::new("java"), game_dir: Path::new("/g"), player: &player, max_memory_mb: 4096 };
    let args = launch.args(&dirs);
    assert_eq!(args[0], "-Xmx4096M");
    assert_eq!(args[1], "-cp");
    assert!(args[2].ends_with("1.21.10.jar"));
    assert_eq!(&args[3..], ["net.minecraft.client.main.Main", "--username", "Steve", "--gameDir", "/g"]);
  }
}
