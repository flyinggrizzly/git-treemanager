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
  #[arg(help = "Subcommand to run", index = 1)]
  subcommand: String,

  #[arg(help = "Branch name")]
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
  Ok(())
}

fn get_default_worktree_location() -> Result<PathBuf, GitTreeManagerError> {
  match std::env::var("HOME") {
    Ok(home) => Ok(PathBuf::from(home).join("worktrees")),
    Err(_) => Err(GitTreeManagerError::MissingHomeEnvVar),
  }
}
