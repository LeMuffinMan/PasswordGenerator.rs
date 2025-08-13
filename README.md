# PasswordGenerator.rs

A simple, fast CLI to generate strong passwords with configurable length and character sets. Configuration can be provided via flags or a TOML file, with flags priority.

## Demo 

![Password Generator Demo](assets/cassette.gif)

*Demo created with [VHS](https://github.com/charmbracelet/vhs)*

## Features

- Customizable length and charset : lowercase, uppercase, digits, symbols and password rules
- Configurable via `config.toml`, custom config files with --file, or CLI flags
- JSON output of the effective configuration

## Install

Prerequisites: Rust (stable) and Cargo.

```bash
# Build and run locally
cargo run -- [OPTIONS]

# Generate documentation locally and consult it
cargo doc --open
```

## Usage

```text
Usage: PasswordGenerator.rs [OPTIONS]

Options:
  -f, --file <FILE>         Path to config file [default: config.toml]
  -l, --length <LENGTH>     Length of password
  -c, --charset [CHARSET]   "luds" activates l(owercase), u(ppercase), d(igit), s(ymbol). If provided without a value, uses "luds"
      --debug               Enable debug output
      --entropy             Show entropy information
      --json                Output the effective config in JSON
      --duplicate           Allow duplicate characters
  -h, --help                Print help
  -V, --version             Print version
```

### Character sets

- l: lowercase `a-z`
- u: uppercase `A-Z`
- d: digits `0-9`
- s: symbols `!@#$%^&*()`

When `--charset` is omitted, the effective set comes from config defaults/file (defaults are lowercase+uppercase+digits enabled, symbols disabled).

### Examples

```bash
# Basic: default config (12 chars, a-z A-Z 0-9)
cargo run --

# 20 characters, all sets (l,u,d,s)
cargo run -- -l 20 --charset

# 16 characters, only letters and digits
cargo run -- -l 16 --charset lud

# 24 characters, include symbols, allow duplicates
cargo run -- -l 24 --charset luds --duplicate

# Show entropy estimate
cargo run -- -l 18 --charset --entropy

# JSON-print the effective configuration (in addition to the password)
cargo run -- --json

# Use a specific config file
cargo run -- --file ./my-password-config.toml
```

## Configuration file (TOML)

By default the app looks for `config.toml` in the working directory. If the file is missing or invalid, safe defaults are used. All fields are optional.

Supported keys:

```toml
# config.toml
length = 12
lowercase = true
uppercase = true
digit = true
symbol = false
duplicate = false
debug = false
entropy = false
json = false
```

Notes:
- Any missing key falls back to a safe default (shown above).
- You can point to another file with `--file /path/to/config.toml`.
- Flags always override file values.
- use cargo doc --open to generate and consult the documentation.

## Output

On success, with no flags, the program prints a password like:

```text
Generated password : Bc4p3Jz2QmN7xK
```

Optional extra output:
- With `--entropy`: prints the entropy estimate.
- With `--json`: prints the effective configuration as a single-line JSON string.
- With `--debug`: prints the effective configuration and the constructed charset.

## Errors and troubleshooting

- "Error : length = 0": provide a valid length.
- "Charset empty": you disabled all character sets; enable at least one.
- "Charset is too small to guarantee no duplicated char": when duplicates are disallowed (default), `length` must be ≤ the size of the effective charset. Either reduce `length`, enable more sets, or pass `--duplicate`.

## Development

```bash
# Run
cargo run -- [OPTIONS]

# Build release
cargo build --release
```
