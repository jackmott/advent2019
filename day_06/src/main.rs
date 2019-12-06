use std::collections::HashMap;
use utils::*;

#[derive(Debug)]
struct OrbitNode {
    children: Vec<String>,
    parent: Option<String>,
}


// does this node have the target_child as a descendant?
fn node_has_child(node: &OrbitNode, target_child: &str, h: &HashMap<String, OrbitNode>) -> bool {
    for child in &node.children {
        if child == target_child {
            return true;
        };
        let node = &h[child];
        if node_has_child(node, target_child, h) {
            return true;
        }
    }
    false
}

// Get the total number of descendants this node has
fn count_children(planet: &str, h: &HashMap<String, OrbitNode>) -> usize {
    if h.contains_key(planet) {
        let children = &h.get(planet).unwrap().children;
        let mut count = 0;
        for child in children {
            count += count_children(&child, h) + 1;
        }
        count
    } else {
        0
    }
}

fn main() {
    let mut h = HashMap::<String, OrbitNode>::new();

    // Build up a tree in the form of a hash table with nodes indexed by planet name
    read_file("input.txt").for_each(|line| {
        let orbits: Vec<&str> = line.split(')').collect();
        let parent = orbits[0].to_string();
        let child = orbits[1].to_string();

        let child_clone = child.clone();
        //add a child as node with no children if it isn't in the tree already
        if !h.contains_key(&child) {
            h.insert(
                child_clone,
                OrbitNode {
                    parent: Some(parent.clone()),
                    children: Vec::new(),
                },
            );
        } else {
            // If a child is already in the tree, set it's parent
            match h.get_mut(&child) {
                Some(c) => c.parent = Some(parent.clone()),
                None => (),
            }
        }

        // if the parent is already in the tree, add the new child
        // otherwise insert the parent into the tree with this one child
        if h.contains_key(&parent) {
            h.get_mut(&parent).unwrap().children.push(child);
        } else {
            let parent = parent.clone();
            h.insert(
                parent,
                OrbitNode {
                    parent: None,
                    children: vec![child],
                },
            );
        }
    });

    // Count up total size of each tree and subtree for part1
    let mut orbits = 0;
    for key in h.keys() {
        orbits += count_children(key, &h);
    }

    println!("orbits:{}", orbits);

    // For part two, travese from YOU to the fist common ancestor of SAN
    let you = &h["YOU"];
    let mut parent = &h[&you.parent.clone().unwrap()];
    let mut count_to_common_parent = 0;
    while !node_has_child(parent, "SAN", &h) {
        count_to_common_parent += 1;
        parent = &h[&parent.parent.clone().unwrap()];
    }
    // And then from SAN to the first common ancestor of YOU
    let san = &h["SAN"];
    parent = &h[&san.parent.clone().unwrap()];
    while !node_has_child(parent, "YOU", &h) {
        count_to_common_parent += 1;
        parent = &h[&parent.parent.clone().unwrap()];
    }
    // The sum of these is the number of orbital transfers we need
    println!("count:{}", count_to_common_parent);
}
