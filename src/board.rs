use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::edge::Edge;
use super::node::Node;
use super::tag::Tagger;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Board {
    pub nodes: HashMap<i32, Node>,
    pub node_tagger: Tagger,
    pub edges: HashMap<i32, Edge>,
    pub edge_tagger: Tagger,
}

impl Board {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            node_tagger: Tagger::new(),
            edges: HashMap::new(),
            edge_tagger: Tagger::new(),
        }
    }
}

impl Board {
    pub fn add_node(&mut self) -> i32 {
        let idx = self.node_tagger.new_tag();
        self.nodes.insert(idx, Node::new());
        idx
    }

    pub fn remove_node(&mut self, idx: &i32) -> Option<Node> {
        let p = self.nodes.remove_entry(idx)?;
        self.node_tagger.remove_tag(p.0);
        for idx in &p.1.edges {
            self.remove_edge(idx);
        }
        Some(p.1)
    }

    pub fn add_edge(&mut self, from: i32, to: i32) -> Option<i32> {
        if from == to {
            return None;
        }

        if !self.nodes.contains_key(&from) {
            return None;
        }

        if !self.nodes.contains_key(&to) {
            return None;
        }

        let idx = self.edge_tagger.new_tag();
        if let Some(_) = self.edges.insert(idx, Edge::new(from, to)) {
            unreachable!("The edge {} was already registered!", idx);
        }
        if let Some(a) = self.nodes.get_mut(&from) {
            a.edges.insert(idx);
        }
        if let Some(a) = self.nodes.get_mut(&to) {
            a.edges.insert(idx);
        }

        Some(idx)
    }

    pub fn remove_edge(&mut self, idx: &i32) -> Option<Edge> {
        let p = self.edges.remove_entry(idx)?;

        for i in [&p.1.to, &p.1.from] {
            if let Some(v) = self.nodes.get_mut(i) {
                v.edges.remove(&p.0);
            }
        }

        Some(p.1)
    }

    pub fn save(&self, path: Option<&str>) {
        let yaml_str = serde_yaml::to_string(&self).expect("Unable to serialize `Board`");

        let default_path = "save/session.yaml";
        let save_path = path.unwrap_or(default_path);

        if let Some(parent_dir) = Path::new(save_path).parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).expect("Unable to create save directory");
            }
        }

        fs::write(save_path, yaml_str).expect(&format!("Unable to save file: `{}`", save_path));
        if path.is_none() {
            let date = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let save_path = format!("save/session_{date}.yaml");
            self.save(Some(&save_path));
        }
    }

    pub fn load(path: Option<&str>) -> Board {
        let default_path = "save/session.yaml";
        let load_path = path.unwrap_or(default_path);

        if let Some(parent_dir) = Path::new(load_path).parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).expect("Unable to create save directory");
            }
        }

        let yaml_str = match fs::read_to_string(load_path) {
            Ok(content) => content,
            Err(_) => return Board::new(),
        };

        serde_yaml::from_str(&yaml_str).expect("Unable to deserialize `Board`")
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn node_add_remove() {
        let mut b = Board::new();
        for _ in 0..5 {
            b.add_node();
        }

        b.remove_node(&0);
        b.remove_node(&1);
        b.remove_node(&3);

        assert!(b.nodes.contains_key(&2));
        assert!(b.nodes.contains_key(&4));
        assert_eq!(b.nodes.len(), 2);
        b.remove_node(&2);
        b.remove_node(&4);
        assert!(b.remove_node(&0).is_none());
        assert_eq!(b.nodes.len(), 0);
    }

    #[test]
    fn edge_add_remove() {
        let mut b = Board::new();
        for _ in 0..5 {
            b.add_node();
        }
        b.add_edge(0, 1);
        b.add_edge(2, 3);
        b.add_edge(0, 2);
        b.add_edge(5, 10);

        b.remove_node(&0);

        assert!(b.nodes.get(&0).is_none());
        assert_eq!(b.nodes.get(&1).expect("missing node!").edges.len(), 0);
        assert!(b.nodes.get(&2).expect("missing node!").edges.contains(&1));
        assert_eq!(b.nodes.get(&2).expect("missing node!").edges.len(), 1);
        assert!(b.nodes.get(&3).expect("missing node!").edges.contains(&1));
        assert_eq!(b.nodes.get(&3).expect("missing node!").edges.len(), 1);
        assert_eq!(b.nodes.get(&4).expect("missing node!").edges.len(), 0);
    }
}
