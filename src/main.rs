pub mod build;
pub mod cli;
pub mod helpers;
pub mod templates;

use cli::Cli;
use structopt::StructOpt;

fn main() {
	let cli = Cli::from_args();

	match cli {
		Cli::Build => build::release(),
		Cli::BuildDev => build::dev(),
		Cli::Watch => cli::commands::watch(),
		Cli::New { dir } => cli::commands::new(&dir),
	}
}
