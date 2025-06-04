use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use git2::Repository;

mod error;
use error::GitTreeManagerError;

mod utils;
use utils::mkdir_p;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
struct Cli {
  #[arg(help = "Subcommand to run", index = 1)]
  subcommand: String,

  #[arg(help = "Branch name", index = 2)]
  branch: Option<String>,

  #[arg(short = 'b', help = "Create a new branch")]
  new_branch: bool,
}

fn main() {
  let cli = Cli::parse();

  let exit_code = get_repo()
    .and_then(|repo| get_subcommand(cli.subcommand.as_str()).map(|subcommand| (repo, subcommand)))
    .and_then(|(repo, subcommand)| perform_subcommand(subcommand, cli, repo))
    .map(|_| 0)
    .map_err(|err| {
      eprintln!("Error: {}", err);
      err.code()
    })
    .unwrap();

  std::process::exit(exit_code)
}

fn get_repo() -> Result<Repository, GitTreeManagerError> {
  Repository::open_from_env().map_err(|_| GitTreeManagerError::MissingRepository)
}

enum Subcommand {
  Create,
}

fn get_subcommand(subcommand: &str) -> Result<Subcommand, GitTreeManagerError> {
  match subcommand {
    "create" => Ok(Subcommand::Create),
    "c" => Ok(Subcommand::Create),
    _ => Err(GitTreeManagerError::UnknownSubcommand(subcommand.into())),
  }
}

fn perform_subcommand(subcommand: Subcommand, cli: Cli, repo: Repository) -> Result<(), GitTreeManagerError> {
  match subcommand {
    Subcommand::Create => perform_create(cli, repo),
  }
}

fn perform_create(cli: Cli, repo: Repository) -> Result<(), GitTreeManagerError> {
  create_worktree(cli, repo)
}

fn create_worktree(cli: Cli, repo: Repository) -> Result<(), GitTreeManagerError> {
  let branch_name = cli.branch.ok_or(GitTreeManagerError::UnspecifiedBranch)?;
  let create_branch = cli.new_branch;

  let mut worktree_opts = git2::WorktreeAddOptions::new();
  let worktrees_dir = get_default_worktree_location()?;

  mkdir_p(&worktrees_dir)?;

  let worktree_path = worktrees_dir.join(&branch_name);

  let reference;
  let checkout_existing;

  if create_branch {
    let head = repo.head().map_err(GitTreeManagerError::Git2Error)?;
    reference = repo
      .branch(
        branch_name.as_str(),
        &head.peel_to_commit().map_err(GitTreeManagerError::Git2Error)?,
        false,
      )
      .map(|branch| branch.into_reference())
      .map_err(GitTreeManagerError::Git2Error)?;

    checkout_existing = false;
  } else {
    reference = repo
      .find_branch(&branch_name, git2::BranchType::Local)
      .map(|branch| branch.into_reference())
      .map_err(GitTreeManagerError::Git2Error)?;

    checkout_existing = true;
  };

  worktree_opts.reference(Some(&reference));
  worktree_opts.checkout_existing(checkout_existing);

  repo
    .worktree(&branch_name, &worktree_path, Some(&worktree_opts))
    .map_err(GitTreeManagerError::Git2Error)?;

  Ok(())
}

fn get_default_worktree_location() -> Result<PathBuf, GitTreeManagerError> {
  match std::env::var("HOME") {
    Ok(home) => Ok(PathBuf::from(home).join("worktrees")),
    Err(_) => Err(GitTreeManagerError::MissingHomeEnvVar),
  }
}
