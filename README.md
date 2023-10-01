# Encoped

A build-time _fast af_ tool to write static apps with html and TypeScript

## Features

- Template-based
- ESLint, Prettier and BunJS integration
- No extra runtime code in your aplication
- TypeScript support by default (with bun)
- No runtime dependencies

## Install

Requirements:

- [Rust](http://rust-lang.org)

Run `cargo install --git https://github.com/Brian3647/encoped` and you're done! You can use the CLI running `encoped`.

## General Usage

Encoped will look for files in `./public/**/*.html` replacing the following syntax:

```html
{{ templates::template_name }}
```

Where `template_name` is any html file in `./templates`.

for example, `{{ templates::hello/world }}` will be replaced with the contents of `./templates/hello/world.html`

## CLI usage

```
USAGE:
    encoped <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    Builds the project
    help     Prints this message or the help of the given subcommand(s)
    new      Creates a new project
    watch    ReBuilds on file change
```

You can also run `encoped help [subcommand]` to get more information about a subcommand.
