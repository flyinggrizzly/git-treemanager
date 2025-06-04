use git2::Repository;

use crate::cli::Cli;
use crate::error::GitTreeManagerError;
use crate::utils::{get_default_worktree_location, mkdir_p};

pub enum Subcommand {
  Create,
}

impl Subcommand {
  pub fn identify_and_perform(cli: Cli, repo: Repository) -> Result<(), GitTreeManagerError> {
    identify(&cli.subcommand).and_then(|subcommand| perform(subcommand, cli, repo))
  }
}

fn identify(subcommand: &str) -> Result<Subcommand, GitTreeManagerError> {
  match subcommand {
    "create" => Ok(Subcommand::Create),
    "c" => Ok(Subcommand::Create),
    _ => Err(GitTreeManagerError::UnknownSubcommand(subcommand.into())),
  }
}

fn perform(subcommand: Subcommand, cli: Cli, repo: Repository) -> Result<(), GitTreeManagerError> {
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
