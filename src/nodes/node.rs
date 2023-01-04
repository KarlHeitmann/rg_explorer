use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod r#type;
pub use r#type::Type;

pub mod data;
pub use data::Data;

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
            Type::summary => (
                // self.data.elapsed_total.as_ref().expect("data.elapsed_total is None").to_string(),
                self.data.elapsed_total.as_ref().expect("data.elapsed_total is None").human.to_string(),
                // String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
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

