use crate::concatln;

pub const ROLLUP_CONFIG: &str = concatln!(
	"import typescript from 'rollup-plugin-typescript2';",
	"import { terser } from 'rollup-plugin-terser';",
	"",
	"const production = !process.env.ROLLUP_WATCH;",
	"",
	"export default {",
	"\tinput: 'src/main.ts',",
	"\toutput: [",
	"\t\t{",
	"\t\t\tfile: 'dist/index.js',",
	"\t\t\tformat: 'iife'",
	"\t\t}",
	"\t],",
	"\tplugins: [",
	"\t\ttypescript(),",
	"\t\tproduction && terser(),",
	"\t]",
	"};"
);
