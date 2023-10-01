use crate::concatln;

pub const BASE_HTML: &str = concatln!(
	"<!DOCTYPE html>",
	"<html>",
	"<head>",
	"\t<meta charset=\"UTF-8\">",
	"\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">",
	"\t<title>{name}</title>",
	"\t<link rel=\"stylesheet\" href=\"/style.css\" />",
	"\t<script src=\"/build.js\"></script>",
	"</head>",
	"<body>",
	"\t{{ templates::hello }}",
	"\t{{ templates::information/all }}",
	"</body>",
	"</html>",
);
