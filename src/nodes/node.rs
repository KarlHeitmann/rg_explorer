use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

pub mod r#type;
pub use r#type::Type;

pub mod data;
pub use data::{Data, SubnodeBegin, Begin};

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    // pub r#type: Type,
    // r#type: Type,
    // data: Data,
    begin: Begin,
    context: Option<Data>,
    // r#match: vec![Data],
    r#match: Vec<Data>,
    end: Data,
}

impl Node {
    pub fn new(data_raw: Vec<(&str, Type)>) -> Self {
        // todo!();
        let mut begin: Option<Begin> = None;
        let mut r#match: Vec<Data> = vec![];
        let mut context: Option<Data> = None;
        let mut end: Option<Data> = None;
        for (d, t) in data_raw {
            match t {
                Type::begin => {
                    let sub_node_begin = Self::parse_subnode_begin(d).expect("begin expected");
                    begin = Some(sub_node_begin.data)
                },
                Type::r#match => {
                    r#match.push(Self::parse_data(d).expect("match expected")) // XXX This can blow up
                },
                Type::context => {
                    context = Some(Self::parse_data(d).expect("context expected")); // XXX This can blow up
                },
                Type::end => {
                    end = Some(Self::parse_data(d).expect("end expected")); // XXX This can blow up
                },
                _ => {}
            }
        }
        Self {
            begin: begin.unwrap(),
            r#match,
            context,
            end: end.unwrap(),
        }
    }
    fn parse_subnode_begin(d: &str) -> Result<SubnodeBegin> {
        let n: SubnodeBegin = serde_json::from_str(d)?;
        Ok(n)
    }
    fn parse_data(d: &str) -> Result<Data> {
        let n: Data = serde_json::from_str(d)?;
        Ok(n)
    }
    pub fn detail(&self) -> (String, String, String, String, String) {
        /*
        match self.r#type {
            Type::r#match => (
                self.data.path.as_ref().expect("data.path has None").to_string(),
                self.data.lines.as_ref().expect("data.lines has None").to_string(),
                // "name".to_string(),
                self.data.line_number.as_ref().expect("data.line_number has None").to_string(),
                self.data.absolute_offset.as_ref().expect("data.absolute_offset has None").to_string(),
                // "age".to_string(),
                String::new(),
                // "created_at".to_string()
            ),
            Type::summary => (
                // self.data.elapsed_total.as_ref().expect("data.elapsed_total is None").to_string(),
                self.data.elapsed_total.as_ref().expect("data.elapsed_total is None").human.to_string(),
                // String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ),
            // _ => ("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string())
            _ => (String::from(""), String::new(), String::new(), String::new(), String::new())
        }
        */
        // TODO
        (
            // self.data.elapsed_total.as_ref().expect("data.elapsed_total is None").to_string(),
            // self.data.elapsed_total.as_ref().expect("data.elapsed_total is None").human.to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        )
    }
    pub fn summary(&self) -> String {
        // TODO
        // self.r#type.to_string()
        self.begin.path.text.to_string()
        // "".to_string()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write!(f, "Rg Explorer: {:?}\n{:?}", self.r#type, self.data)
        write!(f, "TODO!!!!")
    }
}

