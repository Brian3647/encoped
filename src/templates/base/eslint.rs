use crate::concatln;

pub const ESLINTRC_JSON: &str = concatln!(
	"{",
	"\t\"env\": {",
	"\t\t\"es2021\": true,",
	"\t\t\"browser\": true",
	"\t},",
	"\t\"extends\": [\"eslint:recommended\", \"plugin:@typescript-eslint/recommended\"],",
	"\t\"parser\": \"@typescript-eslint/parser\",",
	"\t\"parserOptions\": {",
	"\t\t\"ecmaVersion\": \"latest\",",
	"\t\t\"sourceType\": \"module\"",
	"\t},",
	"\t\"plugins\": [\"@typescript-eslint\"],",
	"\t\"rules\": {",
	"\t\t\"quotes\": [\"error\", \"single\"],",
	"\t\t\"semi\": [\"error\", \"always\"]",
	"\t},",
	"\t\"ignorePatterns\": [\"dist\", \"example\"]",
	"}"
);
