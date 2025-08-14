# PasswordGenerator.rs

A simple, fast CLI to generate strong passwords with configurable length and character sets. Configuration can be provided via flags or a TOML file, with flags priority.

Documentation generated with cargo doc and automated with github actions is available here : https://lemuffinman.github.io/PasswordGenerator.rs

## Pedagogy

This project was my first Rust application, designed as a learning exercise to explore the language fundamentals and ecosystem. Here's what I learned by building this password generator:

### Rust Language Fundamentals
- **Ownership & Borrowing**: Managing memory safety without garbage collection
- **Pattern Matching**: Using `match`, `if let`, and handling `Option`/`Result` types
- **Error Handling**: The `?` operator and `Result<T, E>` pattern for robust error propagation
- **Structs & Implementations**: Organizing data with `struct` and adding behavior with `impl` blocks
- **Traits**: Understanding `Default`, `Debug`, and deriving common functionality
- **Modules**: Code organization with `mod`, `use`, and visibility (`pub`)

### Cargo Ecosystem
- **Project Management**: `cargo new`, `cargo run`, `cargo build --release`
- **Documentation**: `cargo doc --open` for generating and viewing project docs
- **Dependencies**: Managing external crates in `Cargo.toml`

### External Crates Integration
- **[Clap](https://docs.rs/clap/)** (v4): Modern CLI argument parsing with derive macros
  - `#[derive(Parser)]` for automatic CLI generation
  - Argument validation, help text, and subcommands
  - Optional arguments and default values
  
- **[Serde](https://docs.rs/serde/)** (v1): Serialization/deserialization framework
  - `#[derive(Deserialize, Serialize)]` for automatic TOML parsing
  - `#[serde(default)]` for handling missing configuration fields
  - Working with `toml::from_str()` and `toml::to_string_pretty()`

### Rust Design Patterns Learned
- **Builder Pattern**: Constructing complex configurations step-by-step
- **Method Chaining**: Fluent interfaces for configuration overrides
- **Type Safety**: Leveraging Rust's type system to prevent runtime errors
- **Zero-Cost Abstractions**: Writing high-level code without performance penalties
- **Immutability by Default**: Explicit `mut` for safer, more predictable code

### Development Practices
- **Configuration Hierarchy**: Defaults → Config File → CLI Arguments (priority order)
- **Graceful Degradation**: Fallback to safe defaults when config files are missing/invalid
- **Documentation**: Writing clear docstrings and examples for `cargo doc`
- **CI Documentation Deployment**: Automatic generation and deployment of Rust docs to GitHub Pages on each push to `main` 

### Key Takeaways
- **Ecosystem Quality**: High-quality crates with excellent documentation
- **Memory Safety**: No null pointer dereferences or buffer overflows by design

### 📖 Learning Resources Used
- [The Rust Programming Language](https://doc.rust-lang.org/book/) (chapters 1-10)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clap Documentation](https://docs.rs/clap/) and examples
- [Serde Documentation](https://serde.rs/) and TOML integration guides

This project was a discover of a strong type system and helpful compiler messages made learning the language an enjoyable experience.

## Features

- Customizable length and charset : lowercase, uppercase, digits, symbols and password rules
- Configurable via `config.toml`, custom config files with --file, or CLI flags
- JSON output of the effective configuration

## Demo 

![Password Generator Demo](assets/cassette.gif)

*Demo created with [VHS](https://github.com/charmbracelet/vhs)*

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
