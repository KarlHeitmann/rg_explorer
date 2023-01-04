use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Path {
    pub text: String,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lines {
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
pub struct Elapsed {
    secs: usize,
    nanos: usize,
    pub human: String,
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
pub struct Data {
    pub path: Option<Path>,
    pub lines: Option<Lines>,
    pub line_number: Option<usize>,
    pub absolute_offset: Option<usize>,
    submatches: Option<Vec<Submatch>>,
    // binary_offset: Option<String>, // TODO: binary_offset I don't want to implement because I don't know exactly which type of data it is. More info here: https://docs.rs/grep-printer/0.1.6/grep_printer/struct.JSON.html#message-end
    stats: Option<Stats>,
    pub elapsed_total: Option<Elapsed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Begin {
    pub path: Path,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubnodeBegin {
    pub data: Begin,
}


