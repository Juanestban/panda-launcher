use panda_core::{account, auth, net};

// shortcut: console sign-in to exercise panda-core, replace with the zgui UI in Phase 5.
#[tokio::main]
async fn main() {
  if let Err(e) = run().await {
    eprintln!("error: {e}");
    std::process::exit(1);
  }
}

async fn run() -> Result<(), auth::AuthError> {
  let http = net::client();
  match account::resume(&http).await? {
    account::Resume::Online(s) => {
      println!("Signed in as {} (ownership verified)", s.profile.name)
    }
    account::Resume::Offline(a) => {
      println!("Offline: playing as {} (verified before)", a.profile.name)
    }
    account::Resume::SignedOut => {
      let code = auth::device_code(&http).await?;
      println!("Open {} and enter the code {}", code.verification_uri, code.user_code);
      let msa = auth::poll(&http, &code).await?;
      let session = auth::sign_in(&http, msa).await?;
      account::save(&session)?;
      println!("Signed in as {} (ownership verified)", session.profile.name);
    }
  }
  Ok(())
}
