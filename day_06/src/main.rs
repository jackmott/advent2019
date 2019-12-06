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
    // Make a hash of each planet name to it's parent
    let h: HashMap<String, String> = read_file("input.txt")
        .map(|line| {
            let orbits: Vec<&str> = line.split(')').collect();
            (orbits[1].to_string(), orbits[0].to_string())
        })
        .collect();

    // Part 1 - compute the sum of all paths
    let mut count = 0;
    for key in h.keys() {
        count += count_ancestors(key, &h);
    }
    println!("count:{}", count);

    // Part 2
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
