use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    pub name: String,
    pub notes: String,
    pub edges: HashSet<i32>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            name: String::new(),
            notes: String::new(),
            edges: HashSet::new(),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let edges_str: Vec<String> = self.edges.iter().map(|edge| edge.to_string()).collect();
        let edges_display = edges_str.join(", ");
        write!(f, "{}/{}", self.name, edges_display)
    }
}
