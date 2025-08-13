# PasswordGenerator.rs

A simple, fast CLI to generate strong passwords with configurable length and character sets. Configuration can be provided via flags or a TOML file, with flags taking precedence.

## Features

- Customizable length
- Select character sets: lowercase, uppercase, digits, symbols
- Optional duplicate characters (disabled by default)
- Configurable via `config.toml` or CLI flags
- Entropy display (Shannon approximation)
- Debug mode to inspect effective config and charset
- JSON output of the effective configuration

## Install

Prerequisites: Rust (stable) and Cargo.

```bash
# Build and run locally
cargo run -- [OPTIONS]

# Or install the binary to your $HOME/.cargo/bin
cargo install --path .
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
- Use `duplicate` as the key name. The key `allow_duplicate` is not recognized.
- Any missing key falls back to a safe default (shown above).
- You can point to another file with `--file /path/to/config.toml`.
- Flags always override file values.

## Output

On success, the program prints a password like:

```text
Generated password : Bc4p3Jz2QmN7xK
```

Optional extra output:
- With `--entropy`: prints the entropy estimate.
- With `--json`: prints the effective configuration as a single-line JSON string.
- With `--debug`: prints the effective configuration and the constructed charset.

## Entropy

The displayed entropy uses the common approximation:

- Entropy ≈ length × log2(charset_size)

This assumes independent draws with replacement. When `--duplicate` is not set (the default), characters are sampled without replacement, which slightly reduces the theoretical entropy per character; the above approximation remains a practical upper bound.

## Errors and troubleshooting

- "Error : length = 0": set a length > 0.
- "Charset empty": you disabled all character sets; enable at least one.
- "Charset is too small to guarantee no duplicated char": when duplicates are disallowed (default), `length` must be ≤ the size of the effective charset. Either reduce `length`, enable more sets, or pass `--duplicate`.

## Development

```bash
# Run
cargo run -- [OPTIONS]

# Build release
cargo build --release
```

## License

Specify a license for this project (e.g., MIT, Apache-2.0).
