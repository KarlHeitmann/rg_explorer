use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

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
struct Match {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Submatch {
    r#match: Match,
    start: usize,
    end: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    path: Option<Path>,
    lines: Option<Lines>,
    line_number: Option<usize>,
    absolute_offset: Option<usize>,
    submatches: Option<Vec<Submatch>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
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


