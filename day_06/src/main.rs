use std::collections::HashMap;
use utils::*;

#[derive(Debug)]
struct OrbitNode {
    children: Vec<String>,
    parent: Option<String>,
}

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
    read_file("input.txt").for_each(|line| {
        let orbits: Vec<&str> = line.split(')').collect();
        let parent = orbits[0].to_string();
        let child = orbits[1].to_string();

        let child_clone = child.clone();
        if !h.contains_key(&child) {
            h.insert(
                child_clone,
                OrbitNode {
                    parent: Some(parent.clone()),
                    children: Vec::new(),
                },
            );
        } else {
            match h.get_mut(&child) {
                Some(c) => c.parent = Some(parent.clone()),
                None => (),
            }
        }

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

    let mut orbits = 0;
    for key in h.keys() {
        orbits += count_children(key, &h);
    }

    println!("orbits:{}", orbits);

    let you = &h["YOU"];
    let mut parent = &h[&you.parent.clone().unwrap()];
    let mut count_to_common_parent = 0;
    while !node_has_child(parent, "SAN", &h) {
        count_to_common_parent += 1;
        parent = &h[&parent.parent.clone().unwrap()];
    }
    let san = &h["SAN"];
    parent = &h[&san.parent.clone().unwrap()];
    while !node_has_child(parent, "YOU", &h) {
        count_to_common_parent += 1;
        parent = &h[&parent.parent.clone().unwrap()];
    }
    println!("count:{}", count_to_common_parent);
}
