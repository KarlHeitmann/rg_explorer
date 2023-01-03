use std::process::{Command, Stdio};
use std::str;

pub fn run_command() -> String {
    let command = format!("fn --json");
    let child = Command::new("rg")
        .args(command.split(' '))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    assert!(output.status.success());
    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    strip_trailing_newline(s).to_string()
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}


