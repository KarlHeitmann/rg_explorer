use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod node;
pub use node::{Node,Type};
pub mod rip_grep;

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

