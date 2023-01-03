use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

mod io;

#[derive(Serialize, Deserialize, Debug)]
enum Type {
    begin,
    r#match,
    end,
    summary,
}
/*
enum Type {
    Begin,
    Match,
    End,
    Summary,
}
*/

#[derive(Serialize, Deserialize, Debug)]
struct Path {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Lines {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    path: Option<Path>,
    lines: Option<Lines>,
    line_number: Option<usize>,
    absolute_offset: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    r#type: Type,
    data: Data,
    // age: u8,
    // phones: Vec<String>,
}

/*
impl Node {
    pub fn new(data_raw: &str) -> Self {
        Self {
            r#type: Type::begin,
            data: 
            // age: 0,
        }
    }
}
*/

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
        let mut v: Vec<Node> = vec![];

        for d in data_raw {
            // let n: Node = serde_json::from_str(d)?;
            let n:Node = Self::parse_data(d).unwrap();

            v.push(n)
        }
        Self {
            // nodes: vec![Node::new("")],
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

fn run(results: Vec<&str>) -> Result<()> {
    let parsed_result = Nodes::new(results);
    println!("{}", parsed_result);

    Ok(())
}


fn main() {
    println!("Hello, world!");
    let results = io::run_command();
    // let parsed_result = result;
    run(results.split("\n").collect::<Vec<&str>>());
    // let r = rg_wrapper::RgWrapper::new(results.split("\n").collect::<Vec<&str>>());


}
