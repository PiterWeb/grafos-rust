use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::io;
use std::process::exit;

use crate::graph::GraphDocument;
use crate::nodes::Node;

static NONE_OPTION: &str = "User did not select anything";

pub fn print_menu(doc: &mut GraphDocument) -> std::io::Result<()> {
    let menu_options = vec![
        "New Node",
        "New connection",
        "Remove Node",
        "Print Graph",
        "Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&menu_options)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(0) => new_node(doc),
        Some(1) => _ = new_connection(doc),
        Some(2) => _ = remove_node(doc),
        Some(3) => {
            println!("---GRAPH: ---");
            print!("{}", doc);
            println!("---------");
            _ = print_menu(doc);
        }
        Some(4) => exit(0),
        Some(_index) => _ = print_menu(doc),
        None => {
            println!("{}", NONE_OPTION);
            _ = print_menu(doc);
        }
    }

    Ok(())
}

fn new_node(mut doc: &mut GraphDocument) {
    println!("NEW NODE:");
    println!("Attributes (Separed by commas) :");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_err) => input = String::new(),
        Ok(_n) => (),
    };

    let attr: Vec<String> = input.split(",").map(String::from).collect();

    let mut n = Node::new();

    if attr.len() != 0 {
        n.set_attributtes(attr);
    }

    doc.append_node(&n);

    println!("Node {} created", n.get_id());
    _ = print_menu(&mut doc);
}

fn new_connection(mut doc: &mut GraphDocument) -> std::io::Result<()> {
    println!("SELECT NODE 1:");
    let mut node_ids = doc.get_nodes_ids();

    let first_option = Select::with_theme(&ColorfulTheme::default())
        .items(&node_ids)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let mut nodes1 = doc.get_nodes();
    let mut nodes2 = nodes1.clone();

    let first_node = match first_option {
        Some(n) => (nodes1.get_mut(node_ids[n]), n),
        None => (nodes1.get_mut(node_ids[0]), 0),
    };

    node_ids.remove(first_node.1);

    println!("SELECT NODE 2:");

    let second_option = Select::with_theme(&ColorfulTheme::default())
        .items(&node_ids)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let second_node = match second_option {
        Some(n) => (nodes2.get_mut(node_ids[n]), n),
        None => (nodes2.get_mut(node_ids[0]), 0),
    };

    match first_node.0 {
        Some(n1) => {
            match second_node.0 {
                Some(n2) => {
                    println!("{}:{}", n1.get_id(), n2.get_id());
                    doc.append_connection(n1, n2);
                    doc.append_connection(n2, n1)
                }
                None => println!("Second Node is not valid"),
            };
        }
        None => println!("First Node is not valid"),
    }

    _ = print_menu(&mut doc);
    Ok(())
}

fn remove_node(doc: &mut GraphDocument) -> std::io::Result<()> {
    let node_ids = doc.get_nodes_ids();

    let option = Select::with_theme(&ColorfulTheme::default())
        .items(&node_ids)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let node_id_selected = match option {
        Some(n) => node_ids[n],
        None => node_ids[0],
    };

    let mut nodes = doc.get_nodes();

    let selected_node = nodes.get_mut(node_id_selected).unwrap();

    doc.remove_node(&selected_node);

    _ = print_menu(doc);
    Ok(())
}
