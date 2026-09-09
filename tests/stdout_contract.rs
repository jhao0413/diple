//! End-to-end proof that Diple can own a terminal while stdout remains a clean result channel.

#![cfg(any(target_os = "linux", target_os = "macos"))]

mod common;

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use common::Repo;

fn run_in_script(repo: &Repo, keys: &[u8]) -> String {
    let scratch = tempfile::tempdir().unwrap();
    let transcript = scratch.path().join("terminal.typescript");
    let payload = scratch.path().join("review.txt");
    let config = scratch.path().join("config");
    std::fs::create_dir(&config).unwrap();

    let mut command = Command::new("script");
    #[cfg(target_os = "linux")]
    command.args([
        "--quiet",
        "--flush",
        "--command",
        "exec \"$DIPLE_BIN\" \"$DIPLE_REPO\" --poll 600000 >\"$DIPLE_PAYLOAD\"",
        transcript.to_str().unwrap(),
    ]);
    #[cfg(target_os = "macos")]
    command.args([
        "-qF",
        transcript.to_str().unwrap(),
        "/bin/sh",
        "-c",
        "exec \"$DIPLE_BIN\" \"$DIPLE_REPO\" --poll 600000 >\"$DIPLE_PAYLOAD\"",
    ]);
    let mut child = command
        .env("DIPLE_BIN", env!("CARGO_BIN_EXE_diple"))
        .env("DIPLE_REPO", repo.path())
        .env("DIPLE_PAYLOAD", &payload)
        .env("DIPLE_CONFIG_DIR", &config)
        .env("TERM", "xterm-256color")
        .env_remove("DIPLE_LOG")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("the platform script utility starts");

    let ready_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let terminal = std::fs::read(&transcript).unwrap_or_default();
        if terminal.windows(b"\x1b[?1049h".len()).any(|window| window == b"\x1b[?1049h") {
            break;
        }
        if let Some(status) = child.try_wait().unwrap() {
            panic!(
                "Diple exited before claiming the terminal ({status}): {:?}",
                String::from_utf8_lossy(&terminal)
            );
        }
        assert!(Instant::now() < ready_deadline, "Diple never claimed the scripted terminal");
        std::thread::sleep(Duration::from_millis(25));
    }
    child.stdin.take().unwrap().write_all(keys).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            if !status.success() {
                let terminal = std::fs::read_to_string(&transcript).unwrap_or_default();
                let output = std::fs::read_to_string(&payload).unwrap_or_default();
                panic!("script exited with {status}; terminal={terminal:?}; stdout={output:?}");
            }
            break;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            let terminal = std::fs::read_to_string(&transcript).unwrap_or_default();
            let output = std::fs::read_to_string(&payload).unwrap_or_default();
            panic!("Diple did not exit; terminal={terminal:?}; stdout={output:?}");
        }
        std::thread::sleep(Duration::from_millis(25));
    }

    std::fs::read_to_string(payload).unwrap_or_default()
}

#[test]
fn confirmed_send_is_the_only_stdout_payload() {
    let repo = Repo::init();
    repo.write("a.rs", "old\n");
    repo.commit_all("init");
    repo.write("a.rs", "new\n");

    // Focus the diff, jump to its first change, comment, confirm the comment, then confirm Send.
    let payload = run_in_script(&repo, b"\t]cstdout clean\rs\r");
    assert!(payload.contains("a.rs:1"), "location is present: {payload:?}");
    assert!(payload.contains("stdout clean"), "comment is present: {payload:?}");
    assert!(payload.ends_with('\n'));
    assert!(!payload.contains('\u{1b}'), "no terminal escape reaches stdout: {payload:?}");
    assert!(
        payload.chars().all(|ch| ch == '\n' || ch == '\t' || !ch.is_control()),
        "stdout contains no terminal control bytes: {payload:?}"
    );

    let quit_payload = run_in_script(&repo, b"q");
    assert!(quit_payload.is_empty(), "plain quit emits nothing: {quit_payload:?}");
}
