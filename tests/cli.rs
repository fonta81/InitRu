use std::process::{Command, Output};

fn run_initru(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_InitRu"))
        .args(args)
        .output()
        .expect("InitRu should run")
}

#[test]
fn help_lists_commands_and_aliases() {
    let output = run_initru(&["--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for expected in [
        "python",
        "py",
        "bash",
        "sh",
        "rust",
        "rs",
        "javascript",
        "js",
        "typescript",
        "ty",
        "react",
        "rt",
        "golang",
        "go",
        "django",
        "dj",
        "-g",
    ] {
        assert!(
            stdout.contains(expected),
            "help should include {expected}: {stdout}"
        );
    }
}

#[test]
fn no_subcommand_prints_help_hint() {
    let output = run_initru(&[]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "run -h for help"
    );
}
