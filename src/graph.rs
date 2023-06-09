use core::fmt;
use std::collections::HashMap;

use crate::nodes::Node;

pub struct GraphDocument {
    instance: HashMap<String, Node>,
}

impl GraphDocument {
    pub fn new() -> Self {
        Self {
            instance: HashMap::new(),
        }
    }

    pub fn append_node(&mut self, n: &Node) -> &mut Self {
        self.instance.insert(n.get_id(), n.clone());
        return self;
    }

    pub fn remove_node(&mut self, n: &mut Node) -> &mut Self {
        let nodes_ids = self.instance.clone();

        let conected_nodes: Vec<&String> = nodes_ids
            .keys()
            .filter(|k| n.get_connections().contains(*k))
            .collect();

        print!("{:?}", conected_nodes);

        for conn_node in conected_nodes {
            println!("{}", conn_node);
            let node_to_remove = self.instance.get_mut(conn_node);

            if node_to_remove.is_some() {
                node_to_remove.unwrap().remove_connection(n);
            }
        }

        self.instance.remove(&n.get_id());

        return self;
    }

    pub fn append_connection(&mut self, n1: &Node, n2: &mut Node) {
        self.instance
            .get_mut(&n1.get_id())
            .unwrap()
            .add_connection(n2);
    }

    pub fn remove_connection(&mut self, n1: &Node, n2: &mut Node) {
        self.instance
            .get_mut(&n1.get_id())
            .unwrap()
            .remove_connection(n2);
    }

    pub fn get_nodes_ids(&self) -> Vec<&String> {
        return self.instance.keys().collect();
    }

    pub fn get_nodes(&self) -> HashMap<String, Node> {
        return self.instance.clone();
    }
}

impl fmt::Display for GraphDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut graph = String::new();
        for (.., n) in self.instance.iter() {
            graph.push_str(&n.to_string());
            graph.push('\n');
        }

        return write!(f, "{}", graph);
    }
}
