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
                Type::begin | Type::r#match | Type::summary => {
                    aux_vecs.push((d, t.r#type))
                },
                Type::end => {
                    aux_vecs.push((d, t.r#type));
                    let n: Node = Node::new(aux_vecs);
                    v.0.push(n);
                    aux_vecs = vec![];
                }
                _ => {}
            }
        }
        v
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
    pub search_term: String,
    search_term_buffer: String, // TODO: This is the buffer. While editing change this buffer. Run
                                // will run only if search_term_buffer is different search_term
    pub nodes: Nodes,
}

impl RipGrep {
    pub fn new(search_term: String) -> Self {

        Self {
            nodes: Nodes::new(Self::launch_rg(&search_term).split("\n").collect::<Vec<&str>>()),
            search_term_buffer: search_term.clone(),
            search_term,
        }
    }
    fn launch_rg(search_term: &String) -> String {
        let command = format!("{} --json", search_term);
        let child = Command::new("rg")
            .args(command.split(' '))
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");
        let output = child
            .wait_with_output()
            .expect("failed to wait on child");

        assert!(output.status.success()); // Catch failing case: no matches, rg exits with status code 1
        let s = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        Self::strip_trailing_newline(s).to_string()
    }

    pub fn run(&mut self) {
        let res = Self::launch_rg(&self.search_term);
        let res = res.split("\n").collect::<Vec<&str>>();
        self.nodes = Nodes::new(res);
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
        write!(f, "rg {} --json", self.search_term)
    }
}

