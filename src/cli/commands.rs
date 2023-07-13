use colored::Colorize;
use notify::watcher;
use notify::Error;
use notify::RecursiveMode;
use notify::Watcher;

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::build::compile;
use crate::templates::base::DEPENDENCIES;
use crate::templates::base::FILES;

pub fn new(dir: &Path, yarn: bool) {
	println!(
		"{} a project in {}",
		"~> creating".bright_green(),
		dir.display()
	);

	match fs::metadata(&dir) {
		Ok(_) => {
			eprintln!("{} {} already exists", "~> error:".red(), dir.display());
			exit(1);
		}

		Err(e) => {
			if e.kind() == ErrorKind::NotFound {
				fs::create_dir_all(&dir).expect("Unable to create dir");
			} else {
				eprintln!("~> error in fs::metadata: {}", e);
				exit(1);
			}
		}
	};

	FILES.iter().for_each(|file| {
		let path = dir.join(file.0);
		let dir = path.parent().unwrap();

		if !dir.exists() {
			fs::create_dir_all(dir).expect("Unable to create parent dir");
		}

		let contents = file.1.replace("{name}", dir.to_str().unwrap());
		let res = fs::write(&path, contents);

		res.unwrap_or_else(|e| {
			eprintln!("{} unable to write file: {}", "~> error:".red(), e);

			exit(1);
		});
	});

	Command::new(if yarn { "yarn" } else { "npm" })
		.arg(if yarn { "add" } else { "install" })
		.args(DEPENDENCIES)
		.current_dir(dir)
		.status()
		.unwrap_or_else(|e| {
			eprintln!(
				"{} unable to install dependencies: {}",
				"~> error:".red(),
				e
			);

			exit(1);
		});
}

fn err_handler(e: Error, path: Option<PathBuf>) -> ! {
	match e {
		Error::Generic(x) => {
			eprintln!(
				"{} {}{}",
				"~> generic error:".red(),
				x,
				match path {
					Some(x) => format!(" (path = '{}')", x.display()),
					None => "".into(),
				}
			);

			exit(1);
		}

		Error::Io(x) => {
			eprintln!(
				"{} {}{}",
				"~> IO error:".red(),
				x,
				match path {
					Some(x) => format!(" (path = '{}')", x.display()),
					None => "".into(),
				}
			);
			exit(1);
		}

		Error::PathNotFound => {
			match path {
				Some(p) => eprintln!("{} path '{}' not found", "~> error:".red(), p.display()),
				None => eprintln!("{} path not found", "~> error:".red()),
			}

			exit(1);
		}

		Error::WatchNotFound => {
			eprintln!("{}: watch not found", "~> error".red());
			exit(1);
		}
	}
}

fn spawn_rollup() {
	Command::new("./node_modules/.bin/rollup")
		.arg("-c")
		.arg("-w")
		.spawn()
		.unwrap_or_else(|e| {
			eprintln!("{} unable to spawn rollup: {}", "~> error:".red(), e);
			exit(1);
		});
}

pub fn watch() {
	compile();
	spawn_rollup();
	println!("{} for changes", "~> watching".bright_green());

	let (sender, receiver) = channel();
	let mut watcher = match watcher(sender, Duration::from_secs(1)) {
		Ok(w) => w,
		Err(x) => err_handler(x, None),
	};

	let mut watch_path = |path| match watcher.watch(path, RecursiveMode::Recursive) {
		Ok(_) => (),
		Err(x) => err_handler(x, Some(PathBuf::from(path))),
	};

	watch_path("./public");
	watch_path("./templates");

	loop {
		match receiver.recv() {
			Ok(_) => compile(),
			Err(e) => {
				eprintln!("{} {:?}", "~> watch error:".red(), e);
				exit(1)
			}
		};
	}
}
