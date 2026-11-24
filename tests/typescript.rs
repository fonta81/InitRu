#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX epoch")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("initru-typescript-{}-{nonce}", std::process::id()));
        fs::create_dir_all(&path).expect("temporary directory should be created");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn stub_command(bin_dir: &Path, name: &str, body: &str) {
    let path = bin_dir.join(name);
    fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("stub command should be written");
    let mut permissions = fs::metadata(&path)
        .expect("stub command should exist")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("stub command should be executable");
}

fn run_initru(args: &[&str], bin_dir: &Path, npm_marker: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_InitRu"))
        .args(args)
        .current_dir(npm_marker.parent().expect("marker should have a parent"))
        .env("PATH", bin_dir)
        .env("INITRU_NPM_MARKER", npm_marker)
        .output()
        .expect("InitRu should run")
}

#[test]
fn typescript_scaffold_survives_npm_failure_and_normalizes_package_name() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let project = temp.path().join("My Project!");
    let npm_marker = temp.path().join("npm-args");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_command(
        &bin_dir,
        "npm",
        "printf '%s\\n' \"$@\" > \"$INITRU_NPM_MARKER\"\nexit 23",
    );

    let output = run_initru(&["typescript", "My Project!"], &bin_dir, &npm_marker);

    assert!(
        output.status.success(),
        "generator should continue after npm failure: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(project.join("src/main.ts").is_file());
    assert!(project.join("tsconfig.json").is_file());
    assert!(project.join("README.md").is_file());

    let package_json =
        fs::read_to_string(project.join("package.json")).expect("package.json should exist");
    assert!(package_json.contains("\"name\": \"my-project\""));
    assert!(package_json.contains("\"build\": \"tsc\""));
    assert!(package_json.contains("\"start\": \"node dist/main.js\""));
    assert!(package_json.contains("\"typescript\""));

    let tsconfig =
        fs::read_to_string(project.join("tsconfig.json")).expect("tsconfig should exist");
    assert!(tsconfig.contains("\"strict\": true"));
    assert!(tsconfig.contains("\"outDir\": \"./dist\""));
    assert_eq!(
        fs::read_to_string(npm_marker).expect("npm should be invoked"),
        "install\n"
    );
}

#[test]
fn gitignore_is_created_only_when_git_is_requested() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let no_git_project = temp.path().join("without-git");
    let git_project = temp.path().join("with-git");
    let npm_marker = temp.path().join("npm-args");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_command(
        &bin_dir,
        "npm",
        "printf '%s\\n' \"$@\" > \"$INITRU_NPM_MARKER\"\nexit 0",
    );
    stub_command(&bin_dir, "git", "exit 0");

    let no_git_output = run_initru(&["typescript", "without-git"], &bin_dir, &npm_marker);
    assert!(no_git_output.status.success());
    assert!(!no_git_project.join(".gitignore").exists());

    let git_output = run_initru(&["-g", "typescript", "with-git"], &bin_dir, &npm_marker);
    assert!(git_output.status.success());
    let gitignore =
        fs::read_to_string(git_project.join(".gitignore")).expect(".gitignore should exist");
    assert!(gitignore.lines().any(|line| line == "node_modules/"));
    assert!(gitignore.lines().any(|line| line == "dist/"));
}

#[test]
fn help_lists_the_typescript_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_InitRu"))
        .arg("--help")
        .output()
        .expect("InitRu help should run");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("typescript"));
}
