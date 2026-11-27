# External Command Error Handling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Report the command and cause when an InitRu-launched process fails, stop the generator, and return a nonzero CLI exit status.

**Architecture:** Add one typed `InitError` for process spawn failures, unsuccessful process exit, and Django's existing preflight validation errors. The shared process helper and every generator return `Result`; `main` reports the error once and exits unsuccessfully.

**Tech Stack:** Rust 2024 standard library, Clap, Rust integration tests.

**Spec:** `docs/superpowers/specs/2026-11-27-external-command-errors-design.md`

## Global Constraints

- Add no dependencies.
- Preserve child stdout/stderr behavior and report InitRu's diagnostic on stderr.
- Stop on the first failed command and preserve files already generated.
- Cover commands launched by InitRu only; Vite commands later run by users are out of scope.
- Leave file-creation failure handling and generated templates unchanged.

## Review Focus

- **A command that cannot start** must return a diagnostic rather than panic; pin this in `typescript_reports_when_npm_cannot_be_started`.
- **A command that exits unsuccessfully** must identify the command and status, fail the CLI, and retain the generated scaffold; pin this in `typescript_stops_and_reports_when_npm_exits_nonzero`.
- **Optional Git initialization failure** must fail the CLI and prevent later `.gitignore` creation; pin this in `typescript_reports_git_failure_without_writing_gitignore`.
- **A failed command in a multi-step generator** must prevent subsequent setup commands; pin this in `django_stops_when_virtualenv_creation_fails` and `django_stops_when_install_fails`.
- **Successful command sequences** must continue generating their expected files; retain coverage in `gitignore_is_created_only_when_git_is_requested` and `django_creates_named_root_and_invokes_startproject_in_place`.

---

### Task 1: Propagate external command errors through the CLI

**Files:**
- Modify: `src/utils/general.rs`
- Modify: `src/main.rs`
- Modify: `src/make/bash.rs`
- Modify: `src/make/django.rs`
- Modify: `src/make/golang.rs`
- Modify: `src/make/javascript.rs`
- Modify: `src/make/python.rs`
- Modify: `src/make/react.rs`
- Modify: `src/make/rust.rs`
- Modify: `src/make/typescript.rs`
- Modify: `tests/typescript.rs`
- Modify: `tests/django.rs`

**Interfaces:**
- Produces `InitError` in `src/utils/general.rs` with `Validation(String)`, `CommandSpawn { program: String, args: Vec<String>, source: std::io::Error }`, and `CommandFailed { program: String, args: Vec<String>, status: std::process::ExitStatus }` variants. Implement `Display` with the program, arguments, and failure cause/status; implement `std::error::Error`.
- Changes `ejecutar_comando(comando: &str, args: &[&str], directorio: Option<&str>)` and `inicializar_git(directorio: &str)` to return `Result<(), InitError>`.
- Changes each public generator (`bash`, `django`, `golang`, `javascript`, `python`, `react`, `rust`, `typescript`) to return `Result<(), InitError>`.
- `main` handles the generator result once, prints an error to stderr, and exits with status 1 on failure. Preserve the existing no-subcommand help hint.

- [ ] **Step 1: Add or update integration tests**

In `tests/typescript.rs`, update the existing npm-failure case to `typescript_stops_and_reports_when_npm_exits_nonzero`. Assert that a stub `npm` exit status 23 makes the CLI fail, stderr identifies `npm install` and status 23, only one npm invocation occurs, and already-created scaffold files remain.

Add `typescript_reports_when_npm_cannot_be_started`: omit the npm stub, assert a nonzero CLI status and a diagnostic identifying `npm` and the spawn cause.

Add `typescript_reports_git_failure_without_writing_gitignore`: let npm succeed and make the Git stub fail; assert a nonzero status, a diagnostic identifying `git init` and its status, and no `.gitignore`.

In `tests/django.rs`, assert that existing venv and pip failure tests report the failed command and stop before the next command; assert the Git failure test reports Git.

- [ ] **Step 2: Run focused tests and confirm the new expectations fail**

Run: `cargo test --test typescript --test django`

Expected: FAIL because npm failures currently return success, missing commands panic, and most generators ignore failed statuses.

- [ ] **Step 3: Implement typed command errors**

In `src/utils/general.rs`, make `ejecutar_comando` return `CommandSpawn` when `Command::status` cannot start the process and `CommandFailed` for a non-success status. Preserve inherited output streams. Make `inicializar_git` return and propagate the same `Result`.

- [ ] **Step 4: Propagate errors from generators and report them once**

Change all generator functions to return `Result<(), InitError>`. Use `?` for every required external command and optional Git initialization so no later command runs after failure. Convert Django's invalid-name and existing-directory checks to `InitError::Validation`, preserving their current messages.

In `src/main.rs`, collect the selected generator's result, print its error once to stderr, and exit 1 on failure. Keep the existing help hint when no subcommand is selected.

- [ ] **Step 5: Run focused and full tests**

Run: `cargo test --test typescript --test django`

Expected: all selected integration tests pass, including failure diagnostics, early stopping, and preserved scaffold files.

Run: `cargo test`

Expected: all unit and integration tests pass.

- [ ] **Step 6: Commit the completed change**

```bash
git add src tests docs/superpowers/specs/2026-11-27-external-command-errors-design.md docs/superpowers/plans/2026-11-27-external-command-errors.md
git commit -m "fix: report external command failures"
```
