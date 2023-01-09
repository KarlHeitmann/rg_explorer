use std::fmt::{Display, Formatter, Result as FmtResult};
use std::process::{Command, Stdio};
use std::str;
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod node;
pub use node::{Node,Type};

pub struct RgExplorer {
    nodes: Nodes,
}
use tui::widgets::Table;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
struct AuxType {
    r#type: Type,
}

#[derive(Debug)]
pub struct Nodes(pub Vec<Node>);

/*
 * ESTRATEGIA:
 * Tomar data_raw
 * iterarlo
 * Identificar el tipo de data_raw node: begin, match, context, end, summary
 * Si es begin, match, context, agregarlo a un un vec auxiliar
 * Si es end, cerrar el arreglo y construir el Node con ese vector auxiliar de strings. Vaciar el
 *   vec auxiliar
 * Si summary, crear el nodo final y terminar. Agregar los nodos a node
 */
impl Nodes {
    pub fn new(data_raw: Vec<&str>) -> Self {
        let mut v: Nodes = Nodes { 0: vec![]};
        let mut aux_vecs: Vec<(&str, Type)> = vec![];

        for d in data_raw {
            let t = Self::parse_type(d).expect("Error parsing type at first level. Expected begin, match, end, context or summary");
            match t.r#type {
                Type::begin | Type::r#match | Type::summary | Type::context => {
                    aux_vecs.push((d, t.r#type))
                },
                Type::end => {
                    aux_vecs.push((d, t.r#type));
                    let n: Node = Node::new(aux_vecs);
                    v.0.push(n);
                    aux_vecs = vec![];
                }
            }
        }
        v.0.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        v
    }

    pub fn filtered_nodes(&self, folder_filter: String) -> Vec<&Node> {
        let items = &self.0;
        items.into_iter().filter(|node| node.file_name().contains(&folder_filter)).collect()
    }

    pub fn node_matches_count(&self, i: usize) -> usize {
        self.0.get(i).expect("Node must exists").len_matches_all()
    }

    pub fn len(&self) -> usize {
        let Nodes(foo) = self;
        foo.len()
    }

    fn parse_type(d: &str) -> Result<AuxType> {
        let n: AuxType = serde_json::from_str(d)?;
        Ok(n)
    }
}

impl Display for RgExplorer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Rg Explorer: {:?}\n\n====================\n\n{}", self.nodes, self.nodes)
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.iter().fold(Ok(()), |result, node| {
            result.and_then(|_| writeln!(f, "{}", node))
        })
    }
}

pub struct RipGrep {
    search_term: String,
    pub search_term_buffer: String,
    pub nodes: Nodes,
    after_context: usize,
    before_context: usize,
    folder: String,
}

impl RipGrep {
    pub fn new(search_term: String) -> Self {
        let after_context = 1;
        let before_context = 1;
        let folder = String::from(".");
        // let args = format!("{} --json", search_term);
        let args = format!("{search_term} --json -A {after_context} -B {before_context} {folder}");
        let data_raw = Self::launch_rg(args);
        // let data_raw = Self::launch_rg(format!("{} --json -A {} -B {}", &search_term, after_context, before_context));
        match data_raw {
            Some(data_raw) => Self {
                nodes: Nodes::new(data_raw.split("\n").collect::<Vec<&str>>()),
                search_term_buffer: search_term.clone(),
                search_term,
                after_context, before_context, folder,
            },
            None => Self {
                nodes: Nodes::new(vec![]),
                search_term_buffer: search_term.clone(),
                search_term,
                after_context, before_context, folder,
            }
        }
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
        let child = Command::new("rg")
            .args(arguments.split(' '))
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

    pub fn run_wrapper(&mut self) {
        if self.search_term != self.search_term_buffer {
            self.run();
        }
    }

    fn run(&mut self) {
        self.search_term = self.search_term_buffer.clone();
        // let args = format!("{} --json", self.search_term);
        // let search_term = &self.search_term;
        let (search_term, after_context, before_context, folder) = (&self.search_term, &self.after_context, &self.before_context, &self.folder);
        // let args = format!("{search_term} --json");
        let args = format!("{search_term} --json -A {after_context} -B {before_context} {folder}");
        let res = Self::launch_rg(args);
        match res {
            Some(res) => {
                let res = res.split("\n").collect::<Vec<&str>>();
                self.nodes = Nodes::new(res);
            },
            None => {
                self.nodes = Nodes::new(vec![])
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
        write!(f, "rg {} --json", self.search_term_buffer)
    }
}

