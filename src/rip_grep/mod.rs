use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use tui::widgets::Table;
use std::process::{Command, Stdio};

use crate::rip_grep::nodes::Nodes;
use crate::rip_grep::nodes::Node;

mod nodes;

pub struct RipGrep {
    search_term: String,
    pub search_term_buffer: String,
    after_context: usize,
    before_context: usize,
    folder: String,
}

pub struct Explorer {
    pub nodes: Nodes,
    pub grep: RipGrep,
}

impl Explorer {
    pub fn new(search_term: String, folder: String) -> Self {
        let mut grep = RipGrep::new(search_term, folder);
        let nodes = grep.run();
        Self {
            nodes, grep,
        }
    }

    pub fn run_wrapper(&mut self) {
        if self.grep.search_term != self.grep.search_term_buffer {
            self.nodes = self.grep.run();
        }
    }

    pub fn get_file_name_matches(&self) -> String{
        self.nodes.0.iter().fold("".to_string(), |res, n| res + " " + &n.file_name()).trim().to_string()
    }

    pub fn update_context(&mut self, i: usize, delta: isize) {
        let n: &mut Node = self.nodes.0.get_mut(i).unwrap();
        n.update_context(&self.grep, delta);
    }

    pub fn node_detail(&self, i: usize, offset_detail: usize) -> Table {
        match self.nodes.0.get(i) {
            Some(n) => n.detail(offset_detail),
            None => Table::new(vec![])
        }
    }

    pub fn get_node(&self, i: usize) -> &Node {
        self.nodes.0.get(i).expect("Must have a node")
    }
}

impl RipGrep {
    pub fn new(search_term: String, folder: String) -> Self {
        let after_context = 1;
        let before_context = 1;
        Self {
            search_term_buffer: search_term.clone(),
            search_term,
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
        let (search_term, after_context, before_context, folder) = (&self.search_term, &self.after_context, &self.before_context, &self.folder);
        format!("{search_term} --json -A {after_context} -B {before_context} {folder}")
    }

    fn args(&self, after_context: usize, before_context: usize, file: String) -> String {
        let search_term = &self.search_term;
        format!("{search_term} --json -A {after_context} -B {before_context} {file}")
    }

    fn run_immutable(&self, after_context: usize, before_context: usize, file: String) -> String {
        let args = self.args(after_context, before_context, file);
        Self::launch_rg(args).unwrap()
    }

    fn run(&mut self) -> Nodes {
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
        let args = format!("{search_term} --json -A {after_context} -B {before_context}");
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
        let (search_term, after_context, before_context, folder) = (&self.search_term, &self.after_context, &self.before_context, &self.folder);
        write!(f, "rg {search_term} --json -A {after_context} -B {before_context} {folder}")
    }
}

impl Display for Explorer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.grep)
    }
}


