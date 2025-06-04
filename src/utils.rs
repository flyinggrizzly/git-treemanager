use std::fs;
use std::path::PathBuf;

use git2::Repository;

use crate::error::GitTreeManagerError;

pub fn get_repo() -> Result<Repository, GitTreeManagerError> {
  Repository::open_from_env().map_err(|_| GitTreeManagerError::MissingRepository)
}

pub fn mkdir_p(dir: &PathBuf) -> Result<(), GitTreeManagerError> {
  fs::exists(dir)
    .and_then(|exists| if exists { Ok(()) } else { fs::create_dir_all(dir) })
    .map_err(|_| GitTreeManagerError::FsError(format!("Could not create directory {}", dir.display())))
}

pub fn get_default_worktree_location() -> Result<PathBuf, GitTreeManagerError> {
  match std::env::var("HOME") {
    Ok(home) => Ok(PathBuf::from(home).join("worktrees")),
    Err(_) => Err(GitTreeManagerError::MissingHomeEnvVar),
  }
}

pub fn success_exit_code(_: ()) -> i32 {
  0
}
