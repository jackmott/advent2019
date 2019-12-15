use intcomputer::*;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};

use std::thread;

const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

const WALL: i64 = 0;
const MOVED: i64 = 1;
const OXYGEN: i64 = 2;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}
impl Pos {
    fn dist(&self, n: Pos) -> i64 {
        let xdist = (self.x - n.x).abs();
        let ydist = (self.y - n.y).abs();
        xdist + ydist
    }
    fn next_pos(&self, dir: i64) -> Pos {
        match dir {
            NORTH => Pos {
                x: self.x,
                y: self.y - 1,
            },
            SOUTH => Pos {
                x: self.x,
                y: self.y + 1,
            },
            EAST => Pos {
                x: self.x - 1,
                y: self.y,
            },
            WEST => Pos {
                x: self.x + 1,
                y: self.y,
            },
            _ => panic!("impossbiiiruuu"),
        }
    }
}

fn opposite_dir(dir: i64) -> i64 {
    match dir {
        NORTH => SOUTH,
        SOUTH => NORTH,
        WEST => EAST,
        EAST => WEST,
        _ => panic!("impossible direction"),
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    pos: Pos,
    explored: bool,
    is_oxygen: bool,
    n: Option<Pos>,
    s: Option<Pos>,
    e: Option<Pos>,
    w: Option<Pos>,
}
impl Node {
    fn set_neighbor(&mut self, dir: i64, pos: Pos) {
        match dir {
            NORTH => self.n = Some(pos),
            SOUTH => self.s = Some(pos),
            EAST => self.e = Some(pos),
            WEST => self.w = Some(pos),
            _ => panic!("wat"),
        }
    }
    fn get_neighbor(&self, dir: i64) -> Option<Pos> {
        match dir {
            NORTH => self.n,
            SOUTH => self.s,
            EAST => self.e,
            WEST => self.w,
            _ => panic!("wat"),
        }
    }
}

fn check_direction(
    dir: i64,
    node: &mut Node,
    output: &Sender<i64>,
    input: &Receiver<i64>,
    nodes: &mut HashMap<Pos, Node>,
) {
    // position of node we are going to check
    let pos = node.pos.next_pos(dir);
    let opposite_dir = opposite_dir(dir);
    match nodes.get(&pos) {
        None => {
            output.send(dir).unwrap();
            let status = input.recv().unwrap();
            if status == MOVED || status == OXYGEN {
                let is_oxygen = status == OXYGEN;
                let new_node = Node {
                    pos: pos,
                    explored: false,
                    is_oxygen,
                    n: None,
                    s: None,
                    e: None,
                    w: None,
                };
                node.set_neighbor(dir, pos);
                nodes.insert(node.pos, *node);
                nodes.insert(pos, new_node);
                output.send(opposite_dir).unwrap();
                let _ = input.recv().unwrap();
            }
        }
        Some(_) => {
            node.set_neighbor(dir, pos);
            nodes.insert(node.pos, *node);
        }
    }

    match node.get_neighbor(dir) {
        Some(next_pos) => {
            let mut next_node = nodes[&next_pos];
            output.send(dir).unwrap();
            let _ = input.recv().unwrap();
            if !next_node.explored {
                explore_nodes(&mut next_node, output, input, nodes);
            }
            output.send(opposite_dir).unwrap();
            let _ = input.recv().unwrap();
        }
        _ => (),
    }
}

fn explore_nodes(
    node: &mut Node,
    output: &Sender<i64>,
    input: &Receiver<i64>,
    nodes: &mut HashMap<Pos, Node>,
) {
    node.explored = true;
    nodes.insert(node.pos, *node);
    check_direction(NORTH, node, output, input, nodes);
    check_direction(SOUTH, node, output, input, nodes);
    check_direction(EAST, node, output, input, nodes);
    check_direction(WEST, node, output, input, nodes);
}

fn astar(start: Pos, goal: Pos, nodes: &HashMap<Pos, Node>) -> usize {
    let mut frontier = PriorityQueue::new();
    frontier.push(start, 1);
    let mut came_from = HashMap::new();
    came_from.insert(start, start);
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while frontier.len() > 0 {
        let (current, _) = frontier.pop().unwrap();
        if current == goal {
            let mut count = 0;
            let mut n = current;
            while n != start {
                count += 1;
                n = came_from[&n];
            }
            return count;
        } else {
            for dir in 1..5 {
                let node = nodes[&current];
                match node.get_neighbor(dir) {
                    Some(next_pos) => {
                        let next_cost = cost_so_far[&current] + 1;
                        if !cost_so_far.contains_key(&next_pos)
                            || next_cost < cost_so_far[&next_pos]
                        {
                            cost_so_far.insert(next_pos, next_cost);
                            let priority = next_cost + goal.dist(next_pos);
                            frontier.push(next_pos, priority);
                            came_from.insert(next_pos, current);
                        }
                    }
                    None => (),
                }
            }
        }
    }
    0
}

fn imbue_precious_oxygen(nodes: &mut HashMap<Pos, Node>) -> usize {
    let mut count = 0;
    loop {
        let oxygenated_rooms: Vec<Node> =
            nodes.values().filter(|n| n.is_oxygen).map(|n| *n).collect();
        if oxygenated_rooms.len() == nodes.len() {
            break;
        } else {
            for room in oxygenated_rooms {
                for dir in 1..5 {
                    match room.get_neighbor(dir) {
                        Some(pos) => {
                            let mut neighbor = nodes[&pos];
                            neighbor.is_oxygen = true;
                            nodes.insert(pos, neighbor);
                        }
                        None => (),
                    }
                }
            }
        }
        count += 1
    }
    count
}

fn main() -> Result<(), SendError<i64>> {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // PART 1
    let (robut_output, computer_input) = channel();
    let (computer_output, robut_input) = channel();
    thread::spawn(move || {
        // Create a computer and run it
        let mut computer =
            SuperComputer::new("Robut".to_string(), digits, computer_output, computer_input);
        computer.run();
    });

    let mut nodes = HashMap::new();
    let mut start_node = Node {
        pos: Pos { x: 0, y: 0 },
        explored: false,
        is_oxygen: false,
        n: None,
        s: None,
        e: None,
        w: None,
    };
    nodes.insert(start_node.pos, start_node);
    explore_nodes(&mut start_node, &robut_output, &robut_input, &mut nodes);

    let oxygen_node = nodes.values().filter(|node| node.is_oxygen).nth(0).unwrap();

    let count = astar(start_node.pos, oxygen_node.pos, &nodes);
    println!("shortest path to precious oxygen:{}", count);

    let count = imbue_precious_oxygen(&mut nodes);
    println!("time to imbue precious oxygen everywhere:{}", count);

    Ok(())
}
