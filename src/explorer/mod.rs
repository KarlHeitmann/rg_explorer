use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use tui::widgets::Table;
use std::process::{Command, Stdio};

use crossterm::event::KeyCode;

use crate::ui::FilterMode;
use crate::explorer::nodes::Nodes;
use crate::explorer::nodes::Node;

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
    pub filter_mode: FilterMode,
    folder_filter: Vec<String>,
    folder_filter_i: usize,
    pub grep: RipGrep,
}

impl Explorer {
    pub fn new(search_term: String, folder: String) -> Self {
        let mut grep = RipGrep::new(search_term, folder);
        let nodes = grep.run();
        Self {
            folder_filter: vec![String::from("")],
            folder_filter_i: 0,
            filter_mode: FilterMode::Contain,
            nodes, grep,
        }
    }

    pub fn show_folder_filter(&self) -> String {
        self.folder_filter.join(",")

    }

    pub fn update_folder_filter(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(c) => { self.folder_filter[self.folder_filter_i].push(c); },
            KeyCode::Backspace => { self.folder_filter[self.folder_filter_i].pop(); },
            _ => {}
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

    pub fn filtered_nodes(&self) -> Vec<&Node> {
        let items = &self.nodes.0;
        match self.filter_mode {
            FilterMode::Contain => {
                items.into_iter().filter(|node| node.include_filter(&self.folder_filter)).collect()
            },
            FilterMode::Omit => {
                items.into_iter().filter(|node| !node.include_filter(&self.folder_filter)).collect()
            }
        }
    }

    pub fn update_context(&mut self, i: usize, delta: isize) {
        let mut binding = match self.filter_mode {
            FilterMode::Contain => self.nodes.0.iter_mut().filter(|node| node.include_filter(&self.folder_filter)).collect::<Vec<&mut Node>>(),
            FilterMode::Omit => self.nodes.0.iter_mut().filter(|node| !node.include_filter(&self.folder_filter)).collect::<Vec<&mut Node>>(),
        };
        let ns = binding.get_mut(i).unwrap();
        ns.update_context(&self.grep, delta);
    }

    pub fn node_detail(&self, i: usize, offset_detail: usize) -> (String, Table) {
        match self.get_node(i) {
            Some(n) => (n.file_name(), n.detail(offset_detail)),
            None => (String::from(""), Table::new(vec![]))
        }
    }

    pub fn get_node(&self, i: usize) -> Option<&Node> {
        let items = &self.nodes.0;
        let items: Vec<&Node> = match self.filter_mode {
            FilterMode::Contain => {
                items.into_iter().filter(|node| node.include_filter(&self.folder_filter)).collect()
            },
            FilterMode::Omit => {
                items.into_iter().filter(|node| !node.include_filter(&self.folder_filter)).collect()
            }
        };
        items.get(i).copied()
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


