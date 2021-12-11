pub mod commands;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Cli {
	/// ReBuilds on file change
	Watch,

	/// Builds the project on release mode
	Build,

	/// Creates a new project
	New {
		/// Project directory
		#[structopt(default_value = ".", parse(from_os_str))]
		dir: PathBuf,

		/// Uses yarn instead of npm for installing dependencies
		#[structopt(long)]
		yarn: bool,
	},
}
