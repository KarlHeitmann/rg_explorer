use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

mod io;

#[derive(Serialize, Deserialize, Debug)]
enum Type {
    Begin,
    Match,
    End,
    Summary,
}

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    r#type: Type,
    age: u8,
    // phones: Vec<String>,
}

impl Node {
    pub fn new(data_raw: &str) -> Self {
        Self {
            r#type: Type::Begin,
            age: 0,
        }
    }
}

struct Nodes {
    nodes: Vec<Node>,
    // pub data: Vec<json::JsonValue>,

}

impl Nodes {
    pub fn new(data_raw: Vec<&str>) -> Self {
        /*
        let mut v: Vec<String> = vec![];
        let mut js: Vec<json::JsonValue> = vec![];

        for d in data_raw {
            v.push(d.to_string());
            let s = match  json::parse(d) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            js.push(s);
        }
        Self {
            nodes: vec![node::Node::new()],
            data: js,
        }
        */
        Self {
            nodes: vec![Node::new("")],
        }
    }
}


impl Display for Nodes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        /*
        let mut string = String::from("");

        for d in &self.data {
            let num: String = d["type"].to_string();
            string.push_str(&num);
        }

        write!(f, "RgWrapper: {}\n{:?}", string, self.nodes)
        */
        // write!(f, "RgWrapper: {}\n{:?}", string, self.nodes)
        write!(f, "RgWrapper: {:?}", self.nodes)
    }
}




fn main() {
    println!("Hello, world!");
    let results = io::run_command();
    // let parsed_result = result;
    let parsed_result = Nodes::new(results.split("\n").collect::<Vec<&str>>());
    // let r = rg_wrapper::RgWrapper::new(results.split("\n").collect::<Vec<&str>>());

    println!("{}", parsed_result);

}
