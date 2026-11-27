# Error handling for external commands

## Intent

Make InitRu report actionable errors when a command it launches fails, and stop
the current generator instead of continuing as if setup had succeeded.

## Why

The shared command helper currently panics when it cannot start a process and
only prints an exit code when a process fails. Most generators ignore that
failure and continue; Django is the exception, but reports failure through a
boolean. Users need to know which setup command failed and why.

## Goals

- Report the executable and arguments for a failed command.
- Distinguish a process that could not start from one that exited unsuccessfully,
  including the OS error or exit status.
- Propagate the first command failure to the CLI, stop subsequent generator
  steps, and exit with a nonzero status.
- Preserve command stdout/stderr and leave any already-created project files in
  place for diagnosis.
- Apply consistently to external commands launched by every generator,
  including optional Git initialization.

## Non-goals

- Handle failures from commands users run later inside generated projects.
  InitRu installs Vite dependencies with `npm`; it does not run Vite itself.
- Change file-creation error handling, generator input validation beyond what is
  needed to return errors consistently, or generated templates.
- Roll back or delete partially generated projects.
- Add dependencies.

## Constraints and decisions

- Use Rust's standard error and process types; do not add dependencies.
- Keep each child process's existing inherited terminal output and add one
  InitRu diagnostic on stderr.
- Stop at the first failed command; do not run later setup commands or optional
  Git steps.
- Retain partial project files after failure.

## Approaches considered

1. **Propagate typed errors to the CLI (chosen).** The command helper represents
   spawn failures and unsuccessful exit statuses distinctly. Generators
   propagate errors, and the CLI formats the diagnostic and exits nonzero. This
   provides consistent behavior and useful failure context.
2. **Keep boolean results and check each call.** This would require less change
   to the helper's interface, but spreads reporting and failure propagation
   across each generator and cannot represent spawn and exit failures as
   precisely.

## What remains to the implementer

Add a shared typed error for command and existing Django preflight failures;
return it from command helpers and generators; handle it once in the CLI; and
extend the existing integration tests to cover npm spawn/exit failures, Git
failure, early stopping, and the preserved successful scaffold behavior.
