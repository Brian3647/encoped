use std::fs::copy;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::fs::write as write_file;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

use colored::Colorize;
use glob::glob;
use regex::Regex;

lazy_static::lazy_static! {
	pub static ref BUILD_REGEX: Regex = {
		Regex::new("\\{\\{ templates::([^\\}])* \\}\\}").unwrap()
	};
}

pub fn compile() {
	let dist = PathBuf::from("./dist");
	let templates = PathBuf::from("./templates");

	if !templates.exists() {
		println!("{} 'templates' directory", "~> creating".bright_yellow());

		create_dir_all(&templates).unwrap_or_else(|e| {
			println!("{} to create templates directory: {}", "~> failed".red(), e);
			exit(1);
		});
	}

	if !dist.exists() {
		println!("{} 'dist' directory", "~> creating".bright_yellow(),);

		create_dir_all(&dist).unwrap_or_else(|e| {
			println!("{} to create dist directory: {}", "~> failed".red(), e);
			exit(1);
		});
	}

	let paths = glob("./public/**/*.html").unwrap_or_else(|e| {
		eprintln!("{} to read glob pattern: {}", "~> failed".red(), e);
		exit(1);
	});

	for entry in paths {
		match entry {
			Ok(path) => {
				println!("{} '{}'", "~> building".bright_blue(), path.display());
				let before = Instant::now();
				let path = PathBuf::from(&path.to_str().unwrap()[7..]);
				let file = dist.join(&path);

				if !file.exists() {
					println!("{} {}", "~> creating".bright_yellow(), file.display());

					let parent = file.parent().unwrap_or(&path);

					if !parent.exists() {
						println!("{} {}", "~> creating".bright_yellow(), parent.display());

						create_dir_all(&file.parent().unwrap_or(&path)).unwrap_or_else(|e| {
							println!("{} to create directory: {}", "~> failed".red(), e);
							exit(1);
						});
					}

					File::create(&file).unwrap_or_else(|e| {
						eprintln!("{} to create dist file: {}", "~> failed".red(), e);
						exit(1);
					});
				}

				let res = build_file(&PathBuf::from("./public/").join(path));

				match write_file(&file, res) {
					Ok(_) => (),
					Err(e) => {
						eprintln!("{} {}", "~> error writing html file:".red(), e);
						exit(1);
					}
				}

				println!(
					"{} {} in {:?}",
					"~> wrote".bright_green(),
					file.display(),
					before.elapsed(),
				);
			}

			Err(e) => {
				eprintln!("{} {}", "~> error with glob:".red(), e.into_error());
				exit(1);
			}
		}
	}
}

pub fn build_file(path: &Path) -> String {
	let src = match read_to_string(&path) {
		Ok(x) => x,
		Err(e) => {
			eprintln!("{} loading '{}': {}", "~> error".red(), &path.display(), e);

			return String::new();
		}
	};

	let mut res = src.trim().to_string();

	for x in BUILD_REGEX.find_iter(&src) {
		let name = x.as_str().replace("{{ templates::", "").replace(" }}", "");
		let template_path = PathBuf::from("./templates/").join(name + ".html");

		if template_path == path {
			eprintln!(
				"{} template cannot reference itself",
				"~> error loading html file:".red()
			);

			return String::new();
		}

		let template = build_file(&template_path);

		res = res.replace(x.as_str(), &template);
	}

	res
}

fn copy_assets() {
	fn copy_assets_recursively(assets_dir: &Path, dist_dir: &Path) {
		for entry in read_dir(assets_dir).unwrap() {
			let entry = &entry.unwrap();
			let file_name = entry.file_name();
			let file_path = assets_dir.join(&file_name);
			let dist_dir = dist_dir.join(&file_name);

			if file_path.is_dir() {
				copy_assets_recursively(&file_path, &dist_dir);
			} else {
				let dist_parent = dist_dir.parent().unwrap();

				if !dist_parent.exists() {
					create_dir_all(dist_parent).unwrap();
				}

				copy(&file_path, &dist_dir).unwrap();
				println!("{} {}", "~> copied".bright_green(), file_path.display());
			}
		}
	}

	let assets_dir = Path::new("./assets");
	let dist_dir = Path::new("./dist");

	copy_assets_recursively(assets_dir, dist_dir);
}

pub fn release() {
	println!("{}", "~> building (1/3): html".bright_magenta());
	compile();
	println!("{}", "~> building (2/3): assets".bright_magenta());
	copy_assets();
	println!("{}", "~> building (3/3): typescript".bright_magenta());

	Command::new("bun")
    	.arg("build")
		.arg("src/main.ts")
		.arg("--target")
		.arg("browser")
		.arg("--minify-syntax")
		.arg("--minify-whitespace")
		.arg("--outfile")
		.arg("dist/build.js")
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.stdin(Stdio::inherit())
		.output()
		.unwrap_or_else(|e| {
			eprintln!("{} : {}", "~> error in bun:".red(), e);
			exit(1);
		});

	println!("\n{}", "~> building: done".bright_magenta());
}

pub fn dev() {
	println!("{}", "~> building (1/3): html".bright_magenta());
	compile();
	println!("{}", "~> building (2/3): assets".bright_magenta());
	copy_assets();
	println!("{}", "~> building (3/3): typescript".bright_magenta());

	Command::new("bun")
    	.arg("build")
		.arg("src/main.ts")
		.arg("--target")
		.arg("browser")
		.arg("--outfile")
		.arg("dist/build.js")
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.stdin(Stdio::inherit())
		.output()
		.unwrap_or_else(|e| {
			eprintln!("{} : {}", "~> error in bun:".red(), e);
			exit(1);
		});

	println!("\n{}", "~> building: done".bright_magenta());
}
