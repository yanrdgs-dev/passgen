# Passgen

Passgen is a fast CLI tool built in Rust to generate passwords with file storage capabilities.

# Resources

Passgen generates passwords with a combination of:
- Lowercase letters (a-z);
- Uppercase letters (A-Z);
- Numbers (0-9);
- Special symbols (!@#$%&*+-=?_).

It allows for the user to specify the length of the password, and saves it on a secure default directory (`~/.passgen/`).

# Usage
## Requirements
- You must have Rust installed (version 1.50+).

## Installation
```bash
git clone https://github.com/yanrdgs-dev/passgen.git
cd passgen
cargo build --release
```

## Execution
```bash
cargo run <password_length>
```

### Example
```bash
cargo run 16
```

Then, the program will generate and display the password, and prompt the user for save location (default: `~./passgen/`). It also prompts the user for filename (default: `password.txt`), and saves the password to the specified path.

# Configuration
## Customizing character sets
You can modify the `generate_password` function in `src/main.rs`:
```rust
let symbols: Vec<char> = "!@#$%&*+-=?_".chars().collect();
```

## Changing default path behavior
You can edit the `get_file_path` function to change:
- Default directory
- Default filename
- Default extension

# Security considerations
- Passgen uses Rust's secure random generator. See [rand](https://crates.io/crates/rand) for more.
- Excludes problematic characters by default: spaces, quotes (`'`, `"`), backslashes (`\`) and special shell characters (`|`, `;`)

# Contributing
Contributions are welcome! Please open an issue or pull request with your improvements.
