pub mod commands;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Cli {
	/// ReBuilds on file change
	Watch,

	/// Builds the project on release mode
	Build,

	/// Builds the project without minifying
	#[structopt(name = "build-dev")]
	BuildDev,

	/// Creates a new project
	New {
		/// Project directory
		#[structopt(default_value = ".", parse(from_os_str))]
		dir: PathBuf,
	},
}
