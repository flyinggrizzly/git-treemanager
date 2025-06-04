use std::fmt;

#[derive(Debug)]
pub enum GitTreeManagerError {
  MissingHomeEnvVar,
  MissingRepository,
  UncreatedBranch(String),
  AlreadyCreatedBranch(String),
  UnknownSubcommand(String),
  UnspecifiedBranch,
  Git2Error(git2::Error),
  FsError(String),
}

impl fmt::Display for GitTreeManagerError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      GitTreeManagerError::MissingHomeEnvVar => {
        write!(formatter, "The HOME environment variable is not set.")
      }
      GitTreeManagerError::MissingRepository => {
        write!(formatter, "No Git repository found in the current directory.")
      }
      GitTreeManagerError::UncreatedBranch(branch) => {
        write!(formatter, "Branch {} does not exist. Retry with the '-b' flag.", branch)
      }
      GitTreeManagerError::AlreadyCreatedBranch(branch) => {
        write!(
          formatter,
          "Branch {} already exists. Retry without the '-b' flag.",
          branch
        )
      }
      GitTreeManagerError::UnknownSubcommand(subcommand) => {
        write!(
          formatter,
          "Subcommand {} does not exist. Run 'git treemanager --help' to see valid subcommands.",
          subcommand
        )
      }
      GitTreeManagerError::UnspecifiedBranch => {
        write!(formatter, "No branch specified.")
      }
      GitTreeManagerError::Git2Error(git2_error) => {
        write!(formatter, "Git error: {}", git2_error)
      }
      GitTreeManagerError::FsError(message) => {
        write!(formatter, "File system error: {}", message)
      }
    }
  }
}

impl std::error::Error for GitTreeManagerError {}

impl GitTreeManagerError {
  pub fn code(&self) -> i32 {
    match self {
      GitTreeManagerError::MissingHomeEnvVar => 1,
      GitTreeManagerError::MissingRepository => 2,
      GitTreeManagerError::UncreatedBranch(_) => 3,
      GitTreeManagerError::AlreadyCreatedBranch(_) => 4,
      GitTreeManagerError::UnknownSubcommand(_) => 5,
      GitTreeManagerError::UnspecifiedBranch => 6,
      GitTreeManagerError::Git2Error(_) => 7,
      GitTreeManagerError::FsError(_) => 8,
    }
  }
}
