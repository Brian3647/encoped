use std::collections::HashMap;

use crate::concatln;

mod eslint;
mod html;
mod rollup;
mod tsconfig;

pub use eslint::ESLINTRC_JSON;
pub use html::BASE_HTML;
pub use rollup::ROLLUP_CONFIG;
pub use tsconfig::TSCONFIG_JSON;

pub const PACKAGE_JSON: &str = concatln!(
	"{",
	"\t\"name\": \"{name}\",",
	"\t\"version\": \"0.0.0\",",
	"\t\"private\": true,",
	"\t\"license\": \"GPL-3.0-or-later\"",
	"}"
);

pub const DEPENDENCIES: &[&str] = &[
	"rollup",
	"rollup-plugin-terser",
	"rollup-plugin-typescript2",
	"typescript",
];

pub const PRETTIERRC: &str = concatln!(
	"{",
	"\t\"useTabs\": true,",
	"\t\"tabWidth\": 2,",
	"\t\"endOfLine\": \"lf\",",
	"\t\"trailingComma\": \"none\",",
	"\t\"singleQuote\": true,",
	"\t\"semi\": true,",
	"\t\"printWidth\": 80",
	"}"
);

pub const GITIGNORE: &str = concatln!("node_modules", ".DS_Store", "dist");
pub const GITATTRIBUTES: &str = "rollup.config.js	linguist-language=TypeScript\n";

lazy_static::lazy_static! {
	pub static ref FILES: HashMap<&'static str, &'static str> = {
		let mut res = HashMap::new();

		res.insert(".eslintrc.json", ESLINTRC_JSON);
		res.insert("rollup.config.js", ROLLUP_CONFIG);
		res.insert("tsconfig.json", TSCONFIG_JSON);
		res.insert("package.json", PACKAGE_JSON);
		res.insert(".prettierrc", PRETTIERRC);
		res.insert(".gitignore", GITIGNORE);
		res.insert(".gitattributes", GITATTRIBUTES);
		res.insert("public/index.html", BASE_HTML);
		res.insert("src/main.ts", "console.log('Hello, world!');\n");
		res.insert("templates/hello.html", "<h1>This is awesome!</h1>\n");
		res.insert("templates/information/math.html", "Did you know 1 + 1 = 2?\n");
		res.insert("templates/information/all.html", "<p>{{ templates::information/math }}</p>\n{{ templates::information/example }}\n");
		res.insert("templates/information/example.html", "This is quiet.. add something in templates/information/example.html!\n");

		res
	};
}
