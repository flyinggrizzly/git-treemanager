use std::fs;
use std::path::PathBuf;

use crate::error::GitTreeManagerError;

pub fn mkdir_p(dir: &PathBuf) -> Result<(), GitTreeManagerError> {
  fs::exists(dir)
    .and_then(|exists| if exists { Ok(()) } else { fs::create_dir_all(dir) })
    .map_err(|_| GitTreeManagerError::FsError(format!("Could not create directory {}", dir.display())))
}
