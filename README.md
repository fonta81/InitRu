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
- **JavaScript Support:**
  - Creates `src/` directory, `package.json` (ES modules, `start` script), and `src/main.js`.
- **TypeScript Support:**
  - Creates a Node.js TypeScript project with `src/main.ts`, strict `tsconfig.json`, and npm build/start scripts.
  - Installs TypeScript and compiles the project to `dist/`.
- **React Support:**
  - Creates a Vite-based React project (`index.html`, `vite.config.js`, `src/main.jsx`, `src/App.jsx`).
  - Installs `react`, `react-dom`, `vite`, and `@vitejs/plugin-react` automatically (`npm install`).
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
  javascript  Scaffold a JavaScript project
  typescript  Scaffold a TypeScript project
  react   Scaffold a React project (Vite)
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

7. **Create a JavaScript project:**
   ```bash
   InitRu javascript my_app
   ```

8. **Create a JavaScript project with Git initialization:**
   ```bash
   InitRu -g javascript my_app
   ```

9. **Create a React project:**
   ```bash
   InitRu react my_app
   ```

10. **Create a TypeScript project:**
    ```bash
    InitRu typescript my_app
    ```

11. **Create a React project with Git initialization:**
    ```bash
    InitRu -g react my_app
    ```

12. **Display help:**
    ```bash
    InitRu -h
    InitRu python -h
    InitRu bash -h
    InitRu rust -h
    InitRu javascript -h
    InitRu typescript -h
    InitRu react -h
    ```

---


## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for complete details.

Copyright (c) 2026 fonta81.
