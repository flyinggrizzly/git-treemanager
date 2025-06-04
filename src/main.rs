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
    std::process::exit(manage_worktree().map_or_else(
        |err| {
            eprintln!("Error: {}", err);
            err.code()
        },
        std::convert::identity,
    ));
}

fn manage_worktree() -> Result<i32, GitTreeManagerError> {
    let lookup = Repository::open_from_env();
    if lookup.is_err() {
        return Err(GitTreeManagerError::MissingRepository);
    };

    let repo = lookup.unwrap();

    let cli = Cli::parse();

    normalize_subcommand(cli.subcommand.as_str())
        .and_then(|subcommand| perform_subcommand(subcommand, cli, repo))
        .map(|_| 0)
}

enum Subcommand {
    Create,
}

fn normalize_subcommand(subcommand: &str) -> Result<Subcommand, GitTreeManagerError> {
    match subcommand {
        "create" => Ok(Subcommand::Create),
        "c" => Ok(Subcommand::Create),
        _ => Err(GitTreeManagerError::UnknownSubcommand(subcommand.into())),
    }
}

fn perform_subcommand(
    subcommand: Subcommand,
    cli: Cli,
    repo: Repository,
) -> Result<(), GitTreeManagerError> {
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
