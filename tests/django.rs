#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX epoch")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("initru-django-{}-{nonce}", std::process::id()));
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

fn stub_python(bin_dir: &Path) {
    stub_command(
        bin_dir,
        "python3",
        r#"printf 'python3:%s\n' "$*" >> "$INITRU_DJANGO_CALLS"
if [ "$INITRU_DJANGO_FAIL_VENV" = "1" ]; then
    exit 21
fi
/bin/mkdir -p .venv/bin
/bin/cat > .venv/bin/python <<'PYTHON'
#!/bin/sh
printf 'venv:%s\n' "$*" >> "$INITRU_DJANGO_CALLS"
if [ "$2" = "pip" ] && [ "$INITRU_DJANGO_FAIL_PIP" = "1" ]; then
    exit 22
fi
if [ "$2" = "django" ] && [ "$INITRU_DJANGO_FAIL_STARTPROJECT" = "1" ]; then
    exit 23
fi
exit 0
PYTHON
/bin/chmod +x .venv/bin/python"#,
    );
}

fn run_initru(
    args: &[&str],
    bin_dir: &Path,
    cwd: &Path,
    calls: &Path,
    fail: Option<&str>,
) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_InitRu"));
    command
        .args(args)
        .current_dir(cwd)
        .env("PATH", bin_dir)
        .env("INITRU_DJANGO_CALLS", calls);
    if let Some(name) = fail {
        command.env(name, "1");
    }
    command.output().expect("InitRu should run")
}

#[test]
fn django_creates_named_root_and_invokes_startproject_in_place() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    let project = temp.path().join("BlogSite");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);

    let output = run_initru(&["django", "BlogSite"], &bin_dir, temp.path(), &calls, None);

    assert!(
        output.status.success(),
        "generator should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(project.is_dir());
    assert_eq!(
        fs::read_to_string(project.join("requirements.txt"))
            .expect("requirements.txt should exist"),
        "Django\n"
    );
    assert_eq!(
        fs::read_to_string(calls).expect("Python commands should be recorded"),
        "python3:-m venv .venv\nvenv:-m pip install Django\nvenv:-m django startproject BlogSite .\n"
    );
}

#[test]
fn django_rejects_invalid_module_name() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);

    let output = run_initru(&["django", "bad-name"], &bin_dir, temp.path(), &calls, None);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains("unrecognized subcommand"),
        "Django should be a recognized command"
    );
    assert!(!temp.path().join("bad-name").exists());
    assert!(!calls.exists(), "Python should not run for an invalid name");
}

#[test]
fn django_rejects_python_keyword() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);

    let output = run_initru(&["django", "class"], &bin_dir, temp.path(), &calls, None);

    assert_eq!(output.status.code(), Some(1));
    assert!(!temp.path().join("class").exists());
    assert!(!calls.exists(), "Python should not run for a keyword");
}

#[test]
fn django_rejects_package_name_that_shadows_django() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);

    let output = run_initru(&["django", "django"], &bin_dir, temp.path(), &calls, None);

    assert_eq!(output.status.code(), Some(1));
    assert!(!temp.path().join("django").exists());
    assert!(
        !calls.exists(),
        "Python should not run for a shadowing name"
    );
}

#[test]
fn django_refuses_existing_project_directory_without_overwriting_files() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    let project = temp.path().join("existing");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    fs::create_dir_all(&project).expect("existing project directory should be created");
    fs::write(project.join("requirements.txt"), "keep this dependency\n")
        .expect("existing requirements should be written");
    stub_python(&bin_dir);

    let output = run_initru(&["django", "existing"], &bin_dir, temp.path(), &calls, None);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        fs::read_to_string(project.join("requirements.txt"))
            .expect("existing requirements should remain"),
        "keep this dependency\n"
    );
    assert!(
        !calls.exists(),
        "Python should not run for an existing path"
    );
}

#[test]
fn django_stops_when_virtualenv_creation_fails() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);

    let output = run_initru(
        &["django", "failed_venv"],
        &bin_dir,
        temp.path(),
        &calls,
        Some("INITRU_DJANGO_FAIL_VENV"),
    );

    assert!(!output.status.success());
    assert_eq!(
        fs::read_to_string(calls).expect("venv command should be recorded"),
        "python3:-m venv .venv\n"
    );
}

#[test]
fn django_stops_when_install_fails() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);

    let output = run_initru(
        &["django", "failed_install"],
        &bin_dir,
        temp.path(),
        &calls,
        Some("INITRU_DJANGO_FAIL_PIP"),
    );

    assert!(!output.status.success());
    assert_eq!(
        fs::read_to_string(calls).expect("Python commands should be recorded"),
        "python3:-m venv .venv\nvenv:-m pip install Django\n"
    );
}

#[test]
fn django_fails_when_git_initialization_fails() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);
    stub_command(&bin_dir, "git", "exit 31");

    let output = run_initru(
        &["-g", "django", "git_failure"],
        &bin_dir,
        temp.path(),
        &calls,
        None,
    );

    assert_eq!(output.status.code(), Some(1));
    assert!(!temp.path().join("git_failure/.gitignore").exists());
}

#[test]
fn django_returns_failure_when_startproject_fails() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);

    let output = run_initru(
        &["django", "failed_start"],
        &bin_dir,
        temp.path(),
        &calls,
        Some("INITRU_DJANGO_FAIL_STARTPROJECT"),
    );

    assert!(!output.status.success());
    assert!(
        fs::read_to_string(calls)
            .expect("Python commands should be recorded")
            .ends_with("venv:-m django startproject failed_start .\n")
    );
}

#[test]
fn django_gitignore_is_optional_and_covers_generated_artifacts() {
    let temp = TestDir::new();
    let bin_dir = temp.path().join("bin");
    let calls = temp.path().join("calls");
    fs::create_dir_all(&bin_dir).expect("bin directory should be created");
    stub_python(&bin_dir);
    stub_command(&bin_dir, "git", "exit 0");

    let without_git = run_initru(
        &["django", "without_git"],
        &bin_dir,
        temp.path(),
        &calls,
        None,
    );
    assert!(without_git.status.success());
    assert!(!temp.path().join("without_git/.gitignore").exists());

    let with_git = run_initru(
        &["-g", "django", "with_git"],
        &bin_dir,
        temp.path(),
        &calls,
        None,
    );
    assert!(with_git.status.success());
    let gitignore = fs::read_to_string(temp.path().join("with_git/.gitignore"))
        .expect(".gitignore should exist when Git is requested");
    assert!(gitignore.lines().any(|line| line == ".venv/"));
    assert!(gitignore.lines().any(|line| line == "__pycache__/"));
    assert!(gitignore.lines().any(|line| line == "db.sqlite3"));
}

#[test]
fn help_lists_the_django_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_InitRu"))
        .arg("--help")
        .output()
        .expect("InitRu help should run");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("django"));
}
