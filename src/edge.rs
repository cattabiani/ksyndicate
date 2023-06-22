use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Edge {
    pub name: String,
    pub notes: String,
    pub from: i32,
    pub to: i32,
}

impl Edge {
    pub fn new(from: i32, to: i32) -> Edge {
        Edge {
            name: String::new(),
            notes: String::new(),
            from,
            to,
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{} -> {}", self.name, self.from, self.to)
    }
}
