mod cli;
mod error;
mod subcommands;
mod utils;

use cli::Cli;
use subcommands::Subcommand;
use utils::{get_repo, success_exit_code};

fn main() {
  let cli = Cli::build();

  let exit_code = get_repo()
    .and_then(|repo| Subcommand::identify_and_perform(cli, repo))
    .map(success_exit_code)
    .map_err(|err| {
      eprintln!("{}", err);
      err.code()
    })
    .unwrap();

  std::process::exit(exit_code)
}
