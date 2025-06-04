use clap::Parser;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Cli {
  #[arg(help = "Subcommand to run", index = 1)]
  pub subcommand: String,

  #[arg(help = "Branch name", index = 2)]
  pub branch: Option<String>,

  #[arg(short = 'b', help = "Create a new branch")]
  pub new_branch: bool,
}

impl Cli {
  pub fn build() -> Self {
    Self::parse()
  }
}
