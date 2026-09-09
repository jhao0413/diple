//! Process-level checks for the non-interactive CLI surface.

use std::process::Command;

#[test]
fn help_and_version_need_no_terminal() {
    let binary = env!("CARGO_BIN_EXE_diple");

    let version = Command::new(binary).arg("--version").output().expect("run --version");
    assert!(version.status.success());
    assert_eq!(String::from_utf8_lossy(&version.stdout), "diple 0.1.0\n");
    assert!(version.stderr.is_empty());

    let help = Command::new(binary).arg("--help").output().expect("run --help");
    assert!(help.status.success());
    let stdout = String::from_utf8_lossy(&help.stdout);
    assert!(stdout.contains("Usage: diple [OPTIONS] [REPO]"));
    assert!(stdout.contains("--version"));
    assert!(help.stderr.is_empty());
}
