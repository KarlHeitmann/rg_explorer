use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

// Cell::from(Spans::from(vec![Span::styled("My", Style::default().fg(Color::Yellow)), Span::raw(" text"),])),
use tui::widgets::{ Cell, Row, Table, };
use crate::explorer::RipGrep;
use crate::explorer::nodes::AuxType;

pub mod r#type;
pub use r#type::Type;

pub mod data;
pub use data::{Data, SubnodeBegin, SubnodeMatch, SubnodeContext, Begin, Match, Context};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    begin: Begin,
    // context: Vec<Context>, // Option<Data>,
    r#match: Vec<Match>,
    end: Data,
    after_context: usize,
    before_context: usize,
}

impl Node {
    pub fn update_context(&mut self, rip_grep: &RipGrep, delta: isize) {
        if delta > 0 {
            self.after_context += delta as usize;
            self.before_context += delta as usize;
        } else {
            let delta: usize = delta.wrapping_abs() as usize;
            if self.before_context > 0 { self.before_context -= delta; }
            if self.after_context > 0 { self.after_context -= delta; }
        }
        let output = rip_grep.run_immutable(self.after_context, self.before_context, self.file_name());
        self.r#match = vec![];
        let output = output.split("\n").collect::<Vec<&str>>();
        for d in output {
            let t = Self::parse_type(d).expect("Error parsing type at first level. Expected begin, match, end, context or summary");
            match t.r#type {
                Type::r#match | Type::context => {
                    self.r#match.push(Self::parse_subnode_match(d).expect("match expected").data)
                },
                _ => {}
            }
        }

    }
    fn parse_type(d: &str) -> Result<AuxType> {
        let n: AuxType = serde_json::from_str(d)?;
        Ok(n)
    }

    pub fn file_name(&self) -> String {
        self.begin.path.text.clone()
    }

    pub fn include_filter(&self, folder_filter: &Vec<String>) -> bool {
        folder_filter.iter().fold(false, |acc, file| acc || self.begin.path.text.contains(file))
    }

    pub fn new(data_raw: Vec<(&str, Type)>, after_context: usize, before_context: usize) -> Self {
        // todo!(); // XXX Use todo! macro to left a function without implementation, so beautiful :D
        let mut begin: Option<Begin> = None;
        let mut r#match: Vec<Match> = vec![];
        // let mut context: Vec<Context> = vec![];
        let mut end: Option<Data> = None;
        for (d, t) in data_raw {
            match t {
                Type::begin => {
                    let sub_node_begin = Self::parse_subnode_begin(d).expect("begin expected");
                    begin = Some(sub_node_begin.data)
                },
                Type::r#match => {
                    let subnode_begin = Self::parse_subnode_match(d).expect("match expected");
                    r#match.push(subnode_begin.data) // XXX This can blow up
                },
                Type::context => {
                    let subnode_match = Self::parse_subnode_match(d).expect("context expected");
                    r#match.push(subnode_match.data);
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
            end: end.unwrap(),
            after_context, before_context,
        }
    }

    fn parse_subnode_begin(d: &str) -> Result<SubnodeBegin> {
        let n: SubnodeBegin = serde_json::from_str(d)?;
        Ok(n)
    }

    fn parse_subnode_match(d: &str) -> Result<SubnodeMatch> {
        let n: SubnodeMatch = serde_json::from_str(d)?;
        Ok(n)
    }

    fn parse_data(d: &str) -> Result<Data> {
        let n: Data = serde_json::from_str(d)?;
        Ok(n)
    }

    pub fn len_matches_all(&self) -> usize {
        self.r#match.len()
    }

    pub fn detail(&self, offset_detail: usize) -> Table {
        Table::new(
            self.r#match[offset_detail..].iter().map(|m| {
                Row::new(vec![
                    Cell::from(m.pretty_line_match()),
                    // Cell::from(Spans::from(vec![Span::styled("My", Style::default().fg(Color::Yellow)), Span::raw(" text"),])),
                ])
            })
        )
    }

    pub fn summary(&self) -> String {
        format!(" { }", self.begin.path.text.to_string())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write!(f, "Rg Explorer: {:?}\n{:?}", self.r#type, self.data)
        write!(f, "TODO!!!!")
    }
}


