use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod r#type;
pub use r#type::Type;

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Serialize, Deserialize, Debug)]
struct Path {
    text: String,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Lines {
    text: String,
}

impl Display for Lines {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.text)
    }
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
struct Elapsed {
    secs: usize,
    nanos: usize,
    human: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    elapsed: Elapsed,
    searches: usize,
    searches_with_match: usize,
    bytes_searched: usize,
    bytes_printed: usize,
    matched_lines: usize,
    matches: usize,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    path: Option<Path>,
    lines: Option<Lines>,
    line_number: Option<usize>,
    absolute_offset: Option<usize>,
    submatches: Option<Vec<Submatch>>,
    // binary_offset: Option<String>, // TODO: binary_offset I don't want to implement because I don't know exactly which type of data it is. More info here: https://docs.rs/grep-printer/0.1.6/grep_printer/struct.JSON.html#message-end
    stats: Option<Stats>,
    elapsed_total: Option<Elapsed>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    // pub r#type: Type,
    r#type: Type,
    data: Data,
}

impl Node {
    pub fn detail(&self) -> (String, String, String, String, String) {
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
            // _ => ("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string())
            _ => (String::from(""), String::new(), String::new(), String::new(), String::new())
        }
        
    }
    pub fn summary(&self) -> String {
        self.r#type.to_string()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Rg Explorer: {:?}\n{:?}", self.r#type, self.data)
    }
}

