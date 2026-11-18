# InitRu

A fast and simple project scaffolding CLI tool written in Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)

**Languages:** **English** | [Español](README.es.md)

---

## Overview

InitRu is a lightweight command-line tool built with Rust designed to automate project initialization and boilerplate generation. It streamlines creating structured projects with appropriate toolchains and optional Git repository setup with a single command.

---

## Features

- **Fast Scaffolding:** Quickly generates clean project foundations.
- **Python Support:**
  - Standard directory hierarchy and starter files.
  - Automatic virtual environment setup (`python3 -m venv .venv`).
- **Bash Support:**
  - Creates `src/` directory and an executable `src/main.sh` script.
- **Rust Support:**
  - Creates `src/` directory, `Cargo.toml`, and `src/main.rs`.
  - Builds the project with `cargo build`.
- **Optional Git Integration (`-g`):**
  - Initializes a Git repository (`git init`).
  - Automatically creates a `.gitignore` pre-configured for the selected language.

---


## Usage

```text
Usage: InitRu [OPTIONS] [COMMAND]

Commands:
  python  Scaffold a Python project
  bash    Scaffold a Bash project
  rust    Scaffold a Rust project
  help    Print this message or the help of the given subcommand(s)

Options:
  -g          initialize a git repo
  -h, --help  Print help
```

### Examples

1. **Create a standard Python project:**
   ```bash
   InitRu python my_project
   ```

2. **Create a Python project with Git initialization:**
   ```bash
   InitRu -g python my_project
   ```

3. **Create a Bash project:**
   ```bash
   InitRu bash my_script
   ```

4. **Create a Bash project with Git initialization:**
   ```bash
   InitRu -g bash my_script
   ```

5. **Create a Rust project:**
   ```bash
   InitRu rust my_crate
   ```

6. **Create a Rust project with Git initialization:**
   ```bash
   InitRu -g rust my_crate
   ```

7. **Display help:**
   ```bash
   InitRu -h
   InitRu python -h
   InitRu bash -h
   InitRu rust -h
   ```

---


## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for complete details.

Copyright (c) 2026 fonta81.
