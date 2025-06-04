use std::fmt;

#[derive(Debug)]
pub enum GitTreeManagerError {
    MissingHomeEnvVar,
    MissingRepository,
    UncreatedBranch(String),
    AlreadyCreatedBranch(String),
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
                write!(formatter, "Branch {} already exists. Retry without the '-b' flag.", branch)
            }
        }
    }
}

impl std::error::Error for GitTreeManagerError {}
