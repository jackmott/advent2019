use std::collections::HashMap;
use utils::*;

fn count_ancestors(node: &str, h: &HashMap<String, String>) -> usize {
    match h.get(node) {
        Some(parent) => 1 + count_ancestors(parent, h),
        None => 0,
    }
}

fn contains_parent(node: &str, target_parent: &str, h: &HashMap<String, String>) -> bool {
    match h.get(node) {
        Some(parent) => {
            if parent == target_parent {
                return true;
            } else {
                return contains_parent(parent, target_parent, h);
            }
        }
        None => false,
    }
}

fn main() {
    let mut h = HashMap::<String, String>::new();

    // Build up a tree in the form of a hash table with nodes indexed by planet name
    read_file("input.txt").for_each(|line| {
        let orbits: Vec<&str> = line.split(')').collect();
        let parent = orbits[0];
        let child = orbits[1];

        h.insert(child.to_string(), parent.to_string());
    });

    let mut count = 0;
    for key in h.keys() {
        count += count_ancestors(key, &h);
    }
    println!("count:{}", count);

    count = 0;
    let mut node = &h["SAN"];
    // count distance to common ancestor
    while !contains_parent("YOU", node, &h) {
        node = &h[node];
        count += 1;
    }
    node = &h["YOU"];
    while !contains_parent("SAN", node, &h) {
        node = &h[node];
        count += 1;
    }
    println!("count:{}", count);
}
