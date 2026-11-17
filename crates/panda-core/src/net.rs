use std::time::Duration;

/// Identifies the launcher to Modrinth, GitHub and Mojang. Repo URL only, never personal data.
pub const USER_AGENT: &str = concat!("panda-launcher/", env!("CARGO_PKG_VERSION"), " (+https://github.com/juanestban/panda-launcher)");

/// The one HTTP client the launcher uses; clone it freely (it is reference-counted).
pub fn client() -> reqwest::Client {
  // Idle timeouts, not a total one: a 200 MB Java download may take minutes on a slow link.
  reqwest::Client::builder()
    .user_agent(USER_AGENT)
    .connect_timeout(Duration::from_secs(15))
    .read_timeout(Duration::from_secs(30))
    .build()
    .expect("TLS backend initialises")
}

#[cfg(test)]
mod tests {
  #[test]
  fn user_agent_has_no_contact_details() {
    assert!(!super::USER_AGENT.contains('@'));
    assert!(super::USER_AGENT.starts_with("panda-launcher/"));
  }
}
