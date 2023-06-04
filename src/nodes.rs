use random_string::generate;

struct NodeIdentified {
    id: String,
}

impl NodeIdentified {
    fn random_id() -> String {
        return generate(10, String::from("1234567890"));
    }
    fn new() -> Self {
        Self {
            id: Self::random_id(),
        }
    }
    fn get_id(&self) -> String {
        return self.id.clone();
    }
}

impl From<String> for NodeIdentified {
    fn from(value: String) -> Self {
        Self { id: value }
    }
}

pub struct Node {
    identifier: NodeIdentified,
    attributes: Vec<String>,
    connections: Vec<String>,
}

impl Node {
    pub fn get_id(&self) -> String {
        return self.identifier.get_id();
    }

    pub fn new() -> Self {
        Self {
            identifier: NodeIdentified::new(),
            attributes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn get_attributtes(&self) -> &Vec<String> {
        return &self.attributes;
    }

    pub fn get_connections(&self) -> &Vec<String> {
        return &self.connections;
    }

    fn attributes_to_string(&self) -> String {
        return self.get_attributtes().join(";");
    }

    fn connections_to_string(&self) -> String {
        return self.get_connections().join(";");
    }

    pub fn set_attributtes(&mut self, attr: Vec<String>) {
        self.attributes = attr;
    }

    pub fn set_connections(&mut self, conn: Vec<String>) {
        self.connections = conn;
    }

    pub fn add_attributte(&mut self, attr: String) {
        self.attributes.push(attr);
    }

    pub fn add_connection(&mut self, n: &mut Node) {
        self.connections.push(n.get_id());
        n.connections.push(self.get_id())
    }

    pub fn remove_connection(&mut self, n: &mut Node) {
        let mut index = self
            .connections
            .iter()
            .position(|c| c == &n.get_id())
            .unwrap();

        self.connections.remove(index);

        index = n.connections.iter().position(|c| c == &n.get_id()).unwrap();

        n.connections.remove(index);
    }

    pub fn is_connected_with(&self, n: Node) -> bool {
        return self.connections.iter().any(|c| *c == n.get_id());
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        format!(
            "Node: {};\n\tAttributtes: {}\n\tConnections: {}",
            self.get_id(),
            self.attributes_to_string(),
            self.connections_to_string()
        )
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            identifier: NodeIdentified::from(self.identifier.get_id()),
            attributes: self.attributes.clone(),
            connections: self.connections.clone(),
        }
    }
}
