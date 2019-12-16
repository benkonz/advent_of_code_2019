use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Node {
    pub id: String,
    pub parent: Option<String>,
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in buffer.split_whitespace() {
        let parsed: Vec<_> = line.split(')').collect();
        let (parent_id, child_id) = (parsed[0], parsed[1]);
        if !nodes.contains_key(&parent_id.to_string()) {
            let parent = Node {
                id: parent_id.to_string(),
                parent: None,
            };
            nodes.insert(parent.id.to_string(), parent);
        }

        let child = Node {
            id: child_id.to_string(),
            parent: Some(parent_id.to_string()),
        };

        nodes.insert(child_id.to_string(), child);
    }

    let mut count = 0;
    for node in nodes.values() {
        count += get_steps_to_root(node, &nodes);
    }

    println!("{}", count);

    Ok(())
}

fn get_steps_to_root(node: &Node, nodes: &HashMap<String, Node>) -> i32 {
    if let Some(parent) = &node.parent {
        let parent_node = nodes.get(&parent.to_string()).unwrap();
        1 + get_steps_to_root(parent_node, nodes)
    } else {
        0
    }
}
