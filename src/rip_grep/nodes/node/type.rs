use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Type {
    begin,
    r#match,
    end,
    context,
    summary,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let s = match self {
            Self::begin => "Begin",
            Self::r#match => "Match",
            Self::end => "End",
            Self::context => "Context",
            Self::summary => "Summary",
        };
        write!(f, "{}", s)
    }
}

