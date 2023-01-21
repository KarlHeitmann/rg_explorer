use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod node;
pub use node::{Node,Type};
use crate::ui::FilterMode;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
struct AuxType {
    r#type: Type,
}

#[derive(Debug)]
pub struct Nodes(pub Vec<Node>);

impl Nodes {
    pub fn new(data_raw: Vec<&str>, after_context: usize, before_context: usize) -> Self {
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
                    let n: Node = Node::new(aux_vecs, after_context, before_context);
                    v.0.push(n);
                    aux_vecs = vec![];
                }
            }
        }
        v.0.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        v
    }

    pub fn filtered_nodes(&self, folder_filter: String, filter_mode: &FilterMode) -> Vec<&Node> {
        let items = &self.0;
        match filter_mode {
            FilterMode::Embrace => {
                items.into_iter().filter(|node| node.file_name().contains(&folder_filter)).collect()
            },
            FilterMode::Omit => {
                items.into_iter().filter(|node| !node.file_name().contains(&folder_filter)).collect()
            }
        }
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

impl Display for Nodes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.iter().fold(Ok(()), |result, node| {
            result.and_then(|_| writeln!(f, "{}", node))
        })
    }
}

