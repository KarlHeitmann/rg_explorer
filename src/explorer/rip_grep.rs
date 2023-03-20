use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use std::process::{Command, Stdio};

use crate::explorer::Nodes;

pub struct RipGrep {
    pub search_term: String,
    pub search_term_buffer: String,
    after_context: usize,
    before_context: usize,
    extra: String,
    folder: String,
}

impl RipGrep {
    pub fn new(search_term: String, folder: String, extra: String) -> Self {
        let after_context = 1;
        let before_context = 1;
        Self {
            search_term_buffer: search_term.clone(),
            search_term, extra,
            after_context, before_context, folder,
        }
    }

    fn launch_rg(arguments: String) -> Option<String> {
        // let command = format!("{} --json -A {} -B {}", arguments, );
        let arguments = arguments.split(' ');
        let child = Command::new("rg")
            .args(arguments)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");
        let output = child
            .wait_with_output()
            .expect("failed to wait on child");

        let s = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        if s == "" { return None; }
        Some(Self::strip_trailing_newline(s).to_string())
    }

    fn rg_args(&self) -> String {
        let (search_term, after_context, before_context, folder, extra) = (&self.search_term, &self.after_context, &self.before_context, &self.folder, &self.extra);
        format!("{search_term} --json -A {after_context} -B {before_context} {extra} {folder}")
    }

    fn args(&self, after_context: usize, before_context: usize, file: String) -> String {
        let search_term = &self.search_term;
        format!("{search_term} --json -A {after_context} -B {before_context} {file}")
    }

    pub fn run_immutable(&self, after_context: usize, before_context: usize, file: String) -> String {
        let args = self.args(after_context, before_context, file);
        Self::launch_rg(args).unwrap()
    }

    pub fn run(&mut self) -> Nodes {
        self.search_term = self.search_term_buffer.clone();
        let args = self.rg_args();
        let res = Self::launch_rg(args);
        match res {
            Some(res) => {
                let res = res.split("\n").collect::<Vec<&str>>();
                Nodes::new(res, self.after_context.clone(), self.before_context.clone())
            },
            None => {
                Nodes::new(vec![], self.after_context.clone(), self.before_context.clone())
            }
        }
    }

    pub fn raw_output(&self) -> String {
        let (search_term, after_context, before_context) = (&self.search_term, &self.after_context, &self.before_context);
        let args = format!("{search_term} -A {after_context} -B {before_context}");
        let res = Self::launch_rg(args);
        match res {
            Some(res) => format!("{res}"),
            None => String::from("No results :(")
        }
    }

    fn strip_trailing_newline(input: &str) -> &str {
        input
            .strip_suffix("\r\n")
            .or(input.strip_suffix("\n"))
            .unwrap_or(input)
    }
}

impl Display for RipGrep {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (search_term, after_context, before_context, folder, extra) = (&self.search_term, &self.after_context, &self.before_context, &self.folder, &self.extra);
        // write!(f, "rg '{search_term}' --json -A {after_context} -B {before_context} {folder} {extra}")
        write!(f, "rg {search_term} --json -A {after_context} -B {before_context} {folder} {extra}")
    }
}
