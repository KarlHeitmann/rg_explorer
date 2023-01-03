use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

pub mod node;
pub use node::Node;

pub struct RgExplorer {
    nodes: Nodes,
}

#[derive(Debug)]
pub struct Nodes(pub Vec<Node>);

impl RgExplorer {
    pub fn new(data_raw: Vec<&str>) -> Self {
        let mut v: Nodes = Nodes { 0: vec![]};

        for d in data_raw {
            let n = Self::parse_data(d);

            match n {
                Ok(val) => v.0.push(val),
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

impl Display for RgExplorer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Rg Explorer: {:?}\n\n====================\n\n{}", self.nodes, self.nodes)
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}

