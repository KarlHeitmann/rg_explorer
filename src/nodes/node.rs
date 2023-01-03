use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
enum Type {
    begin,
    r#match,
    end,
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
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Rg Explorer: {:?}\n{:?}", self.r#type, self.data)
    }
}

