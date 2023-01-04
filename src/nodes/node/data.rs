use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

use tui::{
    text::{Spans, Span},
    style::{Style, Color},
};

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
    pub text: String,
}

impl Display for Lines {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SubSubMatch { // TODO: change this name to match. You can keep on nesting data structures on src/nodes/node/match/submatch/etc as example
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Submatch { // TODO: change this name to match. You can keep on nesting data structures on src/nodes/node/match/submatch/etc as example
    r#match: SubSubMatch,
    start: usize,
    end: usize,
}

impl Submatch {
    pub fn submatches_details(&self) -> String {
        todo!();
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    pub path: Path,
    pub lines: Lines,
    pub line_number: usize,
    pub absolute_offset: usize,
    submatches: Vec<Submatch>,
    // binary_offset: Option<String>, // TODO: binary_offset I don't want to implement because I don't know exactly which type of data it is. More info here: https://docs.rs/grep-printer/0.1.6/grep_printer/struct.JSON.html#message-end
}

impl Match {
    pub fn line_match(&self) -> String {
        format!("{}:{}", self.line_number, self.lines.text)
    }
    pub fn pretty_line_match(&self) -> Spans {
        let mut output = self.lines.text.clone();
        let mut vs: Vec<Span> = vec![Span::raw(format!("{}:", self.line_number))];
        for submatch in self.submatches.iter() {
            let (left_side, output_rest) = output.split_once(&submatch.r#match.text).unwrap();
            vs.push(Span::raw(left_side.to_string()));
            vs.push(Span::styled(&submatch.r#match.text, Style::default().fg(Color::Yellow)));
            output = output_rest.to_string();
        }
        vs.push(Span::raw(output));
        Spans::from(vs)
    }
    pub fn total_submatches(&self) -> usize {
        self.submatches.len()
    }
    pub fn submatches_details(&self) -> String {
        // self.submatches.submatches_details()
        let mut output = String::new();
        for submatch in self.submatches.iter() {
            output.push_str(&format!("{}\n", submatch.r#match.text));
        }
        output
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubnodeMatch {
    pub data: Match,
}


