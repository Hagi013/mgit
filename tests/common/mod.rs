use std::path::Path;
use std::process::Command;

pub fn git(dir: &Path, args: &[&str]) -> String {
  let out = Command::new("git")
    .args(args)
    .current_dir(dir)
    .env("GIT_AUTHOR_NAME", "T").env("GIT_AUTHOR_EMAIL", "t@example.com")
    .env("GIT_AUTHOR_DATE", "1700000000 +0900")
    .env("GIT_COMMITTER_NAME", "T").env("GIT_COMMITTER_EMAIL", "t@example.com")
    .env("GIT_COMMITTER_DATE", "1700000000 +0900")
    .output()
    .expect("failed to run git");

  assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
  String::from_utf8(out.stdout).unwrap().trim().to_string()
}

pub fn temp_repo() -> tempfile::TempDir {
  let dir = tempfile::tempdir().unwrap();
  git(dir.path(), &["init", "-q"]);
  dir
}
