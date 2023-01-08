use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::Result;

// Cell::from(Spans::from(vec![Span::styled("My", Style::default().fg(Color::Yellow)), Span::raw(" text"),])),
use tui::{
    text::{Span, Spans},
    style::{Color, Style},
    widgets::{
        Cell, Row, Table,
    },
};

pub mod r#type;
pub use r#type::Type;

pub mod data;
pub use data::{Data, SubnodeBegin, SubnodeMatch, SubnodeContext, Begin, Match, Context};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    begin: Begin,
    context: Vec<Context>, // Option<Data>,
    r#match: Vec<Match>,
    end: Data,
}

impl Node {
    pub fn new(data_raw: Vec<(&str, Type)>) -> Self {
        // todo!(); // XXX Use todo! macro to left a function without implementation, so beautiful :D
        let mut begin: Option<Begin> = None;
        let mut r#match: Vec<Match> = vec![];
        let mut context: Vec<Context> = vec![];
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
                    // context = Some(Self::parse_data(d).expect("context expected")); // XXX This can blow up
                    let subnode_context = Self::parse_subnode_context(d).expect("context expected");
                    context.push(subnode_context.data);
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
    fn parse_subnode_context(d: &str) -> Result<SubnodeContext> {
        let n: SubnodeContext = serde_json::from_str(d)?;
        Ok(n)
    }
    fn parse_data(d: &str) -> Result<Data> {
        let n: Data = serde_json::from_str(d)?;
        Ok(n)
    }
    pub fn detail(&self, before_context: usize, after_context: usize) -> Table {
        /*
        let spans = Spans::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::raw(" text"),
        ]);
        Spans::from(vec![Span::styled("My", Style::default().fg(Color::Yellow)), Span::raw(" text"),])
        */
        let mut data: Vec<String> = vec![];
        // self.context.iter().map().col
        // let aux_context = self.context.iter().map(|c| {c.lines.text}).collect();
        // let aux_context: &mut Vec<String> = self.context.iter()
        let mut aux_context: Vec<String> = self.context.iter()
            .map(|c| {c.lines.text})
            .collect();
        data.append(&mut aux_context);
        // vec.push(self.r)
        Table::new(
        // let data = vec![];

        // Table::new(
            self.r#match.iter().map(|m| {
                Row::new(vec![
                    Cell::from(m.pretty_line_match()),
                    Cell::from(Spans::from(vec![Span::styled("My", Style::default().fg(Color::Yellow)), Span::raw(" text"),])),
                ])
                // ]).height(m.total_submatches().try_into().unwrap())
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

