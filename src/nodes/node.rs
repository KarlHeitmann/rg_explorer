use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
enum Type {
    begin,
    r#match,
    end,
    context,
    summary,
}
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
    r#type: Type,
    data: Data,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Rg Explorer: {:?}\n{:?}", self.r#type, self.data)
    }
}

