use std::fmt;

#[derive(Debug)]
pub enum GitTreeManagerError {
    MissingHomeError,
    MissingRepositoryError,
    UncreatedBranchError(String),
    AlreadyCreatedBranchError(String),
}

impl fmt::Display for GitTreeManagerError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GitTreeManagerError::MissingHomeError => {
                write!(formatter, "The HOME environment variable is not set.")
            }
            GitTreeManagerError::MissingRepositoryError => {
                write!(formatter, "No Git repository found in the current directory.")
            }
            GitTreeManagerError::UncreatedBranchError(branch) => {
                write!(formatter, "Branch {} does not exist. Retry with the '-b' flag.", branch)
            }
            GitTreeManagerError::AlreadyCreatedBranchError(branch) => {
                write!(formatter, "Branch {} already exists. Retry without the '-b' flag.", branch)
            }
        }
    }
}

impl std::error::Error for GitTreeManagerError {}
