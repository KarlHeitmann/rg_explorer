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
    pub nodes: Nodes,
    after_context: usize,
    before_context: usize,
    folder: String,
}

impl RipGrep {
    pub fn get_file_name_matches(&self) -> String{
        self.nodes.0.iter().fold("".to_string(), |res, n| res + " " + &n.file_name()).trim().to_string()
    }
    pub fn new(search_term: String, folder: String) -> Self {
        let after_context = 1;
        let before_context = 1;
        // let folder = String::from(".");
        // let args = format!("{} --json", search_term);
        let args = format!("{search_term} --json -A {after_context} -B {before_context} {folder}");
        let data_raw = Self::launch_rg(args);
        // let data_raw = Self::launch_rg(format!("{} --json -A {} -B {}", &search_term, after_context, before_context));
        match data_raw {
            Some(data_raw) => Self {
                nodes: Nodes::new(data_raw.split("\n").collect::<Vec<&str>>(), after_context, before_context),
                search_term_buffer: search_term.clone(),
                search_term,
                after_context, before_context, folder,
            },
            None => Self {
                nodes: Nodes::new(vec![], after_context, before_context),
                search_term_buffer: search_term.clone(),
                search_term,
                after_context, before_context, folder,
            }
        }
    }

    pub fn update_context(&self, i: usize, delta: isize) {
        let node = self.nodes.0.get(i).unwrap();
        // node.update_context(delta)
        node.update_context(self, delta)
    }

    pub fn decrease_context(&mut self) {
        if self.after_context > 0 { self.after_context -= 1; }
        if self.before_context > 0 { self.before_context -= 1; }
        self.run();
    }

    pub fn increase_context(&mut self) {
        self.after_context += 1;
        self.before_context += 1;
        self.run();
    }

    // fn launch_rg(arguments: &String) -> Option<String> {
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

        // assert!(output.status.success()); // Catch failing case: no matches, rg exits with status code 1
        let s = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        if s == "" { return None; }
        Some(Self::strip_trailing_newline(s).to_string())
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

    pub fn run_wrapper(&mut self) {
        if self.search_term != self.search_term_buffer {
            self.run();
        }
    }

    fn rg_args(&self) -> String {
        let (search_term, after_context, before_context, folder) = (&self.search_term, &self.after_context, &self.before_context, &self.folder);
        format!("{search_term} --json -A {after_context} -B {before_context} {folder}")
    }

    fn args(&self, after_context: usize, before_context: usize) -> String {
        let (search_term, folder) = (&self.search_term, &self.folder);
        format!("{search_term} --json -A {after_context} -B {before_context} {folder}")
    }

    /*
    // fn run_immutable(&self, after_context: usize, before_context: usize) -> String {
    // fn run_immutable(&self, after_context: usize, before_context: usize) -> Vec<String> {
    fn run_immutable(&self, after_context: usize, before_context: usize) -> Vec<&str> {
        let args = self.args(after_context, before_context);
        match Self::launch_rg(args) {
            Some(res) => {
                res.split("\n").collect::<Vec<&str>>()
            },
            None => {
                vec![]
            }
        }
    }
    */

    fn run_immutable(&self, after_context: usize, before_context: usize) -> String {
        let args = self.args(after_context, before_context);
        Self::launch_rg(args).unwrap()
    }

    fn run(&mut self) {
        self.search_term = self.search_term_buffer.clone();
        let args = self.rg_args();
        let res = Self::launch_rg(args);
        match res {
            Some(res) => {
                let res = res.split("\n").collect::<Vec<&str>>();
                self.nodes = Nodes::new(res, self.after_context, self.before_context);
            },
            None => {
                self.nodes = Nodes::new(vec![], self.after_context, self.before_context)
            }
        }
    }

    pub fn raw_output(&self) -> String {
        let (search_term, after_context, before_context) = (&self.search_term, &self.after_context, &self.before_context);
        let args = format!("{search_term} --json -A {after_context} -B {before_context}");
        // let res:String = Self::launch_rg(args);
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


