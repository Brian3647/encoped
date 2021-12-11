use crate::concatln;

pub const TSCONFIG_JSON: &str = concatln!(
	"{",
	"\t\"compilerOptions\": {",
	"\t\t\"target\": \"ESNext\",",
	"\t\t\"useDefineForClassFields\": true,",
	"\t\t\"module\": \"ESNext\",",
	"\t\t\"lib\": [\"ESNext\", \"DOM\"],",
	"\t\t\"moduleResolution\": \"Node\",",
	"\t\t\"strict\": true,",
	"\t\t\"sourceMap\": true,",
	"\t\t\"esModuleInterop\": true,",
	"\t\t\"noEmit\": true,",
	"\t\t\"noUnusedLocals\": true,",
	"\t\t\"noUnusedParameters\": true,",
	"\t\t\"noImplicitReturns\": true",
	"\t},",
	"\t\"include\": [\"**/*.ts\", \"**/*.d.ts\"],",
	"\t\"exclude\": [\"node_modules\", \"dist\"]",
	"}"
);
