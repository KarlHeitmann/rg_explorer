use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

use tui::{
    text::{Span, Spans, Text},
    style::{Color, Modifier, Style},
    widgets::{
        Cell, Row, Table,
    },
};

pub mod r#type;
pub use r#type::Type;

pub mod data;
pub use data::{Data, SubnodeBegin, SubnodeMatch, Begin, Match};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    begin: Begin,
    context: Option<Data>,
    r#match: Vec<Match>,
    end: Data,
}

impl Node {
    pub fn new(data_raw: Vec<(&str, Type)>) -> Self {
        // todo!(); // XXX Use todo! macro to left a function without implementation, so beautiful :D
        let mut begin: Option<Begin> = None;
        let mut r#match: Vec<Match> = vec![];
        let mut context: Option<Data> = None;
        let mut end: Option<Data> = None;
        for (d, t) in data_raw {
            match t {
                Type::begin => {
                    let sub_node_begin = Self::parse_subnode_begin(d).expect("begin expected");
                    begin = Some(sub_node_begin.data)
                },
                Type::r#match => {
                    let subnode_begin = Self::parse_subnode_match(d).expect("begin expected");
                    r#match.push(subnode_begin.data) // XXX This can blow up
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
    fn parse_subnode_match(d: &str) -> Result<SubnodeMatch> {
        let n: SubnodeMatch = serde_json::from_str(d)?;
        Ok(n)
    }
    fn parse_data(d: &str) -> Result<Data> {
        let n: Data = serde_json::from_str(d)?;
        Ok(n)
    }
    pub fn detail(&self) -> Table {
        let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
        /*
        let spans = Spans::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::raw(" text"),
        ]);
        Spans::from(vec![Span::styled("My", Style::default().fg(Color::Yellow)), Span::raw(" text"),])
        */
        Table::new(
            self.r#match.iter().map(|m| {
                Row::new(vec![
                    Cell::from(Span::raw(m.line_match())),
                    // Cell::from(Span::raw(&m.path.text)),
                    Cell::from(Span::styled("My \n text", style)),
                    // Cell::from(Span::raw(&m.path.text)),
                    Cell::from(Spans::from(vec![Span::styled("My", Style::default().fg(Color::Yellow)), Span::raw(" text"),])),
                    // Cell::from(Text::from("The first line\nThe second line")),
                    Cell::from(Text::from(m.submatches_details())),
                ]).height(m.total_submatches().try_into().unwrap())
            })
        )
    }
    pub fn summary(&self) -> String {
        self.begin.path.text.to_string()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write!(f, "Rg Explorer: {:?}\n{:?}", self.r#type, self.data)
        write!(f, "TODO!!!!")
    }
}

