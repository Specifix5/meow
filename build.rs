use std::process::Command;

fn main() {
  let output_git_hash_short = Command::new("git")
    .args(&["rev-parse", "--short", "HEAD"])
    .output()
    .expect("Failed to get git commit hash");

  let output_git_hash_long = Command::new("git")
    .args(&["rev-parse", "HEAD"])
    .output()
    .expect("Failed to get git commit hash");

  let str_output_git_hash_short = String::from_utf8_lossy(&output_git_hash_short.stdout);
  let git_hash = str_output_git_hash_short.trim();

  let str_output_git_hash_long = String::from_utf8_lossy(&output_git_hash_long.stdout);
  let git_hash_long = str_output_git_hash_long.trim();

  println!("cargo:rustc-env=GIT_HASH={}", git_hash);
  println!("cargo:rustc-env=GIT_HASH_LONG={}", git_hash_long);
}
