use std::fmt::{Display, Formatter, Result as FmtResult};
use std::process::{Command, Stdio};
use std::str;

pub struct RipGrep<'a> {
    search_term: &'a str,
}
impl<'a> RipGrep<'a> {
    pub fn new(search_term: &'a str) -> Self {
        Self {
            search_term
        }
    }
    pub fn run_command(&self) -> String {
        let command = format!("{} --json", self.search_term);
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
        Self::strip_trailing_newline(s).to_string()
    }

    fn strip_trailing_newline(input: &str) -> &str {
        input
            .strip_suffix("\r\n")
            .or(input.strip_suffix("\n"))
            .unwrap_or(input)
    }
}

impl Display for RipGrep<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "rg {} --json", self.search_term)
    }
}

