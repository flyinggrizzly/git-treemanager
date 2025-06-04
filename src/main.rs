use anyhow::Result;
use clap::Parser;
use git2::Repository;
use std::path::PathBuf;

mod error;
use error::GitTreeManagerError;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
struct Cli {
    #[arg(help = "Branch name")]
    branch: Option<String>,

    #[arg(short = 'b', help = "Create a new branch")]
    new_branch: bool,
}

fn main() -> Result<(), GitTreeManagerError> {
    let cli = Cli::parse();

    let lookup = Repository::open_from_env();
    if lookup.is_err() { return Err(GitTreeManagerError::MissingRepository); };

    let repo = lookup.unwrap();

    let branch = cli.branch.as_deref().unwrap();
    let new_branch = if cli.new_branch {
        "new-branch"
    } else {
        "extant-branch"
    };
    let valid_basic_input = if validate_branch_args(branch, repo, cli.new_branch).is_ok() {
        "good to go"
    } else {
        "bad input"
    };

    println!("Branch: {}", branch);
    println!("New Branch: {}", new_branch);
    println!("Input ready: {}", valid_basic_input);
    Ok(())
}

fn validate_branch_args(
    branch: &str,
    repo: Repository,
    create_branch: bool,
) -> Result<(), GitTreeManagerError> {
    let local_branch = repo.find_branch(branch, git2::BranchType::Local);
    let remote_branch = repo.find_branch(branch, git2::BranchType::Remote);

    match (create_branch, local_branch, remote_branch) {
        (true, Ok(_), _) => Err(GitTreeManagerError::AlreadyCreatedBranch(
            branch.to_string(),
        )),
        (true, _, Ok(_)) => Err(GitTreeManagerError::AlreadyCreatedBranch(
            branch.to_string(),
        )),
        (false, Err(_), Err(_)) => Err(GitTreeManagerError::UncreatedBranch(
            branch.to_string(),
        )),
        _ => Ok(()),
    }
}

fn get_default_worktree_location() -> Result<PathBuf, GitTreeManagerError> {
    match std::env::var("HOME") {
        Ok(home) => Ok(PathBuf::from(home).join("worktrees")),
        Err(_) => Err(GitTreeManagerError::MissingHomeEnvVar),
    }
}
