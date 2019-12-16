use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut nodes = HashMap::new();

    for line in buffer.split_whitespace() {
        let parsed: Vec<_> = line.split(')').collect();
        let (parent_id, child_id) = (parsed[0], parsed[1]);

        let child = nodes.entry(child_id.to_string()).or_insert_with(Vec::new);
        child.push(parent_id.to_string());

        let parent = nodes.entry(parent_id.to_string()).or_insert_with(Vec::new);
        parent.push(child_id.to_string());
    }

    let mut queue = VecDeque::new();
    queue.push_back("YOU");
    let mut depth_queue = VecDeque::new();
    depth_queue.push_front(0);
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let node_key = queue.pop_front().unwrap();
        let depth = depth_queue.pop_front().unwrap();
        if !visited.contains(&node_key.to_string()) {
            visited.insert(node_key.to_string());

            if node_key == "SAN" {
                // subtract 2 because we don't count the jump from us to the planet we are orbiting and the jump
                // that santa is orbiting to santa
                println!("{}", depth - 2);
                break;
            }
            let node = nodes.get(&node_key.to_string()).unwrap();

            for child in node {
                queue.push_back(child);
                depth_queue.push_back(depth + 1);
            }
        }
    }

    Ok(())
}
