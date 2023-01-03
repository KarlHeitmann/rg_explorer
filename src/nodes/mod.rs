use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

pub mod node;
pub use node::Node;

pub struct Nodes {
    nodes: Vec<Node>,
}

impl Nodes {
    pub fn new(data_raw: Vec<&str>) -> Self {
        let mut v: Vec<Node> = vec![];

        for d in data_raw {
            let n = Self::parse_data(d);

            match n {
                Ok(val) => v.push(val),
                Err(e) => println!("{e:?}\n{}", d),
            }
        }
        Self {
            nodes: v,
        }
    }
    fn parse_data(d: &str) -> Result<Node> {
        let n: Node = serde_json::from_str(d)?;
        Ok(n)
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Rg Explorer: {:?}", self.nodes)
    }
}

