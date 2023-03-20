use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use tui::widgets::Table;
use std::process::{Command, Stdio};

use crossterm::event::KeyCode;

use crate::ui::FilterMode;
use crate::explorer::nodes::Nodes;
use crate::explorer::nodes::Node;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

mod nodes;

pub struct RouteNode {
    domain: String,
    prefix: Option<String>,
    verb: String,
    uri_pattern: String,
    controller_action: String,
}

impl RouteNode {
    pub fn new(domain: String, s: String) -> Result<Self, &'static str> {
        let ss = s.split(" ");
        let ss = ss.filter(|s| !s.is_empty() );
        match ss.clone().count() {
            4 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    domain,
                    prefix: Some(ss.get(0).unwrap().to_string()), verb: ss.get(1).unwrap().to_string(), uri_pattern: ss.get(2).unwrap().to_string(), controller_action: ss.get(3).unwrap().to_string()
                })
            },
            3 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    domain,
                    prefix: None, verb: ss.get(0).unwrap().to_string(), uri_pattern: ss.get(1).unwrap().to_string(), controller_action: ss.get(2).unwrap().to_string()
                })
            }
            _ => {Err("Invalid number of strings in line")}
        }
    }
    pub fn route(&self, target: &String) -> Result<String, &'static str> {
        if target.starts_with("app/views/") {
            let (_, file) = target.split_at(10);
            let (file_name, extension) = file.split_once(".").unwrap();
            let mut file_data: Vec<&str> = file_name.split("/").collect();
            let action = file_data.pop().unwrap();
            // println!("{:?}||||{}", file_data.join("::"), action);
            let target = format!("{}#{}", file_data.join("/"), action);

            match self.controller_action.contains(&target) {
                true => Ok(format!("{}{} | controller_action: {} | target: {}", self.domain, self.uri_pattern, self.controller_action, target)),
                false => Err("controller_action don't contain target"),
            }
        } else {
            Err("Target doesn't starts with `app/views`")
        }
    }
}

pub struct Routes {
    // path: String,
    domain: String,
    route_nodes: Vec<RouteNode>,
}

impl Routes {
    pub fn new(domain: &str) -> Option<Self> {
        // Create a path to the desired file
        let path = Path::new("routes.txt");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let routes = match File::open(&path) {
            Err(_) => None,
            Ok(mut file) => { 
                let mut s = String::new();
                let res = match file.read_to_string(&mut s) {
                    Err(_) => None,
                    Ok(_) => {
                        // print!("{} contains:\n{}", display, s);
                        let mut route_nodes: Vec<RouteNode> = vec![];
                        let ss = s.split("\n");
                        for s in ss {
                            match RouteNode::new(domain.to_string(), s.to_string()) {
                                Ok(route_node) => route_nodes.push(route_node),
                                Err(_e) => {}
                            }
                        }
                        Some(Self {
                            domain: domain.to_string(),
                            route_nodes,
                        })
                    }
                };

                res
            }
        };

        routes
    }
    pub fn find(&self, target: String) -> Option<String> {
        let mut result = None;
        for route_node in &self.route_nodes {
            match route_node.route(&target) {
                Ok(route) => {
                    result = Some(route);
                    break;
                },
                Err(_) => {},
            }
        }
        result
    }
}

pub struct RipGrep {
    search_term: String,
    pub search_term_buffer: String,
    after_context: usize,
    before_context: usize,
    extra: String,
    folder: String,
}

pub struct Explorer {
    pub nodes: Nodes,
    pub filter_mode: FilterMode,
    folder_filter: Vec<String>,
    folder_filter_i: usize,
    routes: Option<Routes>,
    pub grep: RipGrep,
}

impl Explorer {
    pub fn new(search_term: String, folder: String, word: Option<String>, ignorecase: Option<String>) -> Self {
        let extra = match (word, ignorecase) {
            (Some(_), Some(_)) => String::from("-w -i"),
            (None, Some(_)) => String::from("-i"),
            (Some(_), None) => String::from("-w"),
            (None, None) => String::new()
        };
        let mut grep = RipGrep::new(search_term, folder, extra);
        let nodes = grep.run();
        let routes = Routes::new("http://localhost:3000");

        Self {
            routes,
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
            KeyCode::Enter => {
                self.folder_filter.push(String::new());
                self.folder_filter_i += 1;
            },
            KeyCode::Backspace => {
                let ff = &mut self.folder_filter[self.folder_filter_i];

                if ff.len() > 0 {
                    ff.pop();
                } else if self.folder_filter.len() > 0 {
                    self.folder_filter.pop();
                    self.folder_filter_i -= 1;
                }
            },
            _ => {}
        }
    }

    pub fn run_wrapper(&mut self) {
        if self.grep.search_term != self.grep.search_term_buffer {
            self.nodes = self.grep.run();
        }
    }

    pub fn get_file_name_matches(&self) -> String{
        self.filtered_nodes().iter().fold("".to_string(), |res, n| res + " " + &n.file_name()).trim().to_string()
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
            Some(n) => { 
                let m = match &self.routes {
                    Some(routes) => { routes.find(n.file_name()) },
                    None => None
                };
                let s = n.file_name();
                let tmp = n.detail(offset_detail, m);
                (s, tmp)
            },
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
        let (search_term, after_context, before_context, folder) = (&self.search_term, &self.after_context, &self.before_context, &self.folder);
        write!(f, "rg {search_term} --json -A {after_context} -B {before_context} {folder}")
    }
}

impl Display for Explorer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.grep)
    }
}


