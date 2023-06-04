mod graph;
mod nodes;

fn main() {
    let mut n1 = nodes::Node::new();

    let node_attr: Vec<String> =
        Vec::from(["Blue".to_string(), "Red".to_string(), "Green".to_string()]);

    let mut n2 = nodes::Node::new();
    n2.set_attributtes(node_attr);

    n1.add_connection(&mut n2);

    n1.add_attributte("Purple".to_string());

    let mut doc = graph::GraphDocument::new();

    doc.append_node(&n1);
    doc.append_node(&n2);

    let mut n3 = nodes::Node::new();

    n3.add_attributte("Yellow".to_string());

    doc.append_node(&n3);

    doc.append_connection(&n1, &mut n3);

    print!("{}", doc);
}
