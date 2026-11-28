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
        let path = std::env::temp_dir().join(format!("initru-c-{}-{nonce}", std::process::id()));
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

#[test]
fn c_creates_a_compilable_source_layout_and_builds_it() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let project = temp.path().join("hello_c");
    let calls = temp.path().join("gcc-args");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_command(
        &bin_dir,
        "gcc",
        "printf '%s\\n' \"$*\" > \"$INITRU_GCC_MARKER\"\nexit 0",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_InitRu"))
        .args(["c", "hello_c"])
        .current_dir(temp.path())
        .env("PATH", bin_dir)
        .env("INITRU_GCC_MARKER", &calls)
        .output()
        .expect("InitRu should run");

    assert!(
        output.status.success(),
        "C generator should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(project.join("README.md").is_file());
    assert_eq!(
        fs::read_to_string(project.join("src/main.c")).expect("main.c should exist"),
        "#include <stdio.h>\n\nint main(void) {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}\n"
    );
    assert!(project.join("Makefile").is_file());
    assert_eq!(
        fs::read_to_string(calls).expect("gcc should be invoked"),
        "-Wall -Wextra -std=c11 src/main.c -o main\n"
    );
}

#[test]
fn c_gitignore_is_optional_and_compiler_failure_is_reported() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_command(&bin_dir, "gcc", "exit 42");

    let output = Command::new(env!("CARGO_BIN_EXE_InitRu"))
        .args(["-g", "c", "git_c"])
        .current_dir(temp.path())
        .env("PATH", bin_dir)
        .output()
        .expect("InitRu should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("gcc"),
        "stderr should identify gcc: {stderr}"
    );
    assert!(
        stderr.contains("42"),
        "stderr should include status 42: {stderr}"
    );
    assert!(!temp.path().join("git_c/.gitignore").exists());
}
