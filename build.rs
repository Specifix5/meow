use std::process::Command;

fn main() {
  let output = Command::new("git")
    .args(&["rev-parse", "--short", "HEAD"])
    .output()
    .expect("Failed to get git commit hash");

  let str = String::from_utf8_lossy(&output.stdout);
  let git_hash = str.trim();

  println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
