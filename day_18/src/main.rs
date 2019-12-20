use bitflags::*;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::VecDeque;
use utils::*;

type MapGraph = Graph<MapNodeData, i32, Undirected, usize>;

bitflags! {
    struct Keys:u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const E = 1 << 4;
        const F = 1 << 5;
        const G = 1 << 6;
        const H = 1 << 7;
        const I = 1 << 8;
        const J = 1 << 9;
        const K = 1 << 10;
        const L = 1 << 11;
        const M = 1 << 12;
        const N = 1 << 13;
        const O = 1 << 14;
        const P = 1 << 15;
        const Q = 1 << 16;
        const R = 1 << 17;
        const S = 1 << 18;
        const T = 1 << 19;
        const U = 1 << 20;
        const V = 1 << 21;
        const W = 1 << 22;
        const X = 1 << 23;
        const Y = 1 << 24;
        const Z = 1 << 25;
    }
}
impl Keys {
    fn from_char(c: char) -> Keys {
        match c.to_lowercase().nth(0).unwrap() {
            'a' => Keys::A,
            'b' => Keys::B,
            'c' => Keys::C,
            'd' => Keys::D,
            'e' => Keys::E,
            'f' => Keys::F,
            'g' => Keys::G,
            'h' => Keys::H,
            'i' => Keys::I,
            'j' => Keys::J,
            'k' => Keys::K,
            'l' => Keys::L,
            'm' => Keys::M,
            'n' => Keys::N,
            'o' => Keys::O,
            'p' => Keys::P,
            'q' => Keys::Q,
            'r' => Keys::R,
            's' => Keys::S,
            't' => Keys::T,
            'u' => Keys::U,
            'v' => Keys::V,
            'w' => Keys::W,
            'x' => Keys::X,
            'y' => Keys::Y,
            'z' => Keys::Z,
            _ => panic!("invalid key/door"),
        }
    }
}

struct Map {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
}
impl Map {
    fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, bool)> {
        let mut result = Vec::new();

        let (open, visited) = self.is_open(pos.up());
        if open {
            result.push((pos.up(), visited));
        }

        let (open, visited) = self.is_open(pos.down());
        if open {
            result.push((pos.down(), visited));
        }

        let (open, visited) = self.is_open(pos.left());
        if open {
            result.push((pos.left(), visited));
        }

        let (open, visited) = self.is_open(pos.right());
        if open {
            result.push((pos.right(), visited));
        }

        println!("get unvisited n for {:?}", pos);
        println!("{:?}", result);
        result
    }

    fn get_tile(&self, pos: Pos) -> &Tile {
        let index = pos.y as usize * self.w + pos.x as usize;
        &self.tiles[index]
    }

    fn get_index(&self, pos: Pos) -> usize {
        pos.y * self.w + pos.x
    }
    fn is_open(&self, pos: Pos) -> (bool, bool) {
        self.get_tile(pos).is_open()
    }
    fn visit(&mut self, pos: Pos) {
        let index = self.get_index(pos);
        self.tiles[index].visited = true;
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn up(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }
}

use TileType::*;
#[derive(Copy, Clone, Debug, PartialEq)]
enum TileType {
    Space,
    Wall,
    Door(Keys),
    Key(Keys),
    Start,
}
impl TileType {
    fn is_open(&self) -> bool {
        match self {
            Wall => false,
            _ => true,
        }
    }

    fn from_char(c: char) -> TileType {
        match c {
            _ if c >= 'a' && c <= 'z' => Key(Keys::from_char(c)),
            _ if c >= 'A' && c <= 'Z' => Door(Keys::from_char(c)),
            '@' => Start,
            '.' => Space,
            '#' => Wall,
            _ => panic!("invalid tile:{}", c),
        }
    }
}

#[derive(Clone, Debug)]
struct Tile {
    tile_type: TileType,
    pos: Pos,
    visited: bool,
}
impl Tile {
    fn from_char(c: char, pos: Pos) -> Tile {
        Tile {
            tile_type: TileType::from_char(c),
            pos,
            visited: false,
        }
    }
    fn is_open(&self) -> (bool, bool) {
        (self.tile_type.is_open(), self.visited)
    }
}

#[derive(Clone, Copy, Debug)]
struct MapNodeData {
    pos: Pos,
    key: Keys,
    door: Keys,
}
impl MapNodeData {
    fn from_tile(tile: &Tile) -> MapNodeData {
        let door = match tile.tile_type {
            Door(key) => key,
            _ => Keys::empty(),
        };
        let key = match tile.tile_type {
            Key(key) => key,
            _ => Keys::empty(),
        };
        MapNodeData {
            pos: tile.pos,
            key: key,
            door: door,
        }
    }
}

fn build_map_graph(
    node_index: NodeIndex<usize>,
    map: &mut Map,
    graph: &mut MapGraph,
    key_nodes: &mut Vec<NodeIndex<usize>>,
    pos_node_map: &mut HashMap<Pos, NodeIndex<usize>>,
) {
    let node_data = graph[node_index];
    pos_node_map.insert(node_data.pos, node_index);
    let tile = map.get_tile(node_data.pos);
    match tile.tile_type {
        Key(_) => key_nodes.push(node_index),
        _ => (),
    }
    map.visit(node_data.pos);
    for (pos, visited) in map.get_neighbors(node_data.pos) {
        let tile = map.get_tile(pos);

        if !visited {
            let next_node_index = graph.add_node(MapNodeData::from_tile(tile));
            graph.update_edge(node_index, next_node_index, 1);
            build_map_graph(next_node_index, map, graph, key_nodes, pos_node_map);
        } else {
            let next_node_index = *pos_node_map.get(&pos).unwrap();
            graph.update_edge(node_index, next_node_index, 1);
        };
    }
}


fn indices_to_string(indices:&Vec<NodeIndex<usize>>) -> String{
    indices.iter().map(|i| i.index().to_string()).collect()
}

fn main() {
    let mut w = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut y = 0;
    let mut key_count = 0;
    for line in read_file("input.txt") {
        w = line.len();
        let mut x = 0;
        for c in line.chars() {
            let tile = Tile::from_char(c, Pos { x, y });
            match tile.tile_type {
                Key(_) => key_count += 1,
                _ => (),
            }
            tiles.push(tile);
            x += 1;
        }
        y += 1;
    }

    let mut final_keys = Keys::empty();
    let mut char_key = 'a';
    let mut key_vec = Vec::new();
    for _ in 0..key_count {
        let key = Keys::from_char(char_key);
        final_keys = final_keys | key;
        key_vec.push(key);
        char_key = (char_key as u8 + 1) as char;
    }

    let h = tiles.len() / w;
    let mut map = Map { tiles, w, h };

    let start_tile_indices: Vec<usize> = map
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| match tile.tile_type {
            Start => true,
            _ => false,
        })
        .map(|(index, _)| index)
        .collect();

    let mut map_graph: MapGraph = Graph::default();
    let start_node_indices: Vec<NodeIndex<usize>> = start_tile_indices
        .iter()
        .map(|index| map_graph.add_node(MapNodeData::from_tile(&map.tiles[*index])))
        .collect();

    let mut key_nodes = Vec::new();
    for index in &start_node_indices {
        key_nodes.push(*index);
    }
    println!("key nodes after append:{:?}", key_nodes);
    let mut pos_node_map = HashMap::new();
    for index in &start_node_indices {
        build_map_graph(
            *index,
            &mut map,
            &mut map_graph,
            &mut key_nodes,
            &mut pos_node_map,
        );
        println!("graph node count:{}", map_graph.node_count());
    }
    println!("key nodes after building graph:{:?}", key_nodes);


    println!("key nodes:{:?}", key_nodes);
    let key_node_map: HashMap<Keys, NodeIndex<usize>> = key_nodes
        .iter()
        .map(|index| {
            let node = map_graph[*index];
            let key = node.key;
            (key, *index)
        })
        .collect();
    println!("key node map:{:?}", key_node_map);




    let mut queue = VecDeque::new();
    let mut cost_map = HashMap::new();
    cost_map.insert((indices_to_string(&start_node_indices), Keys::empty()), 0);
    queue.push_back((0, Keys::empty(), start_node_indices));

    // key your at and keys you have -> cost

    let mut min_cost = 0;
    let mut old_queue_size = 0;
    let mut path_map = HashMap::new();
    while queue.len() > 0 {
        //   println!("-----------------");
        let (current_cost, current_keys, current_nodes) = queue.pop_front().unwrap();
        if current_keys == final_keys {
            println!("woo");
            if min_cost == 0 {
                min_cost = current_cost;
            } else if current_cost < min_cost {
                min_cost = current_cost;
            }
            //break;
        }

        if (queue.len() as i32 - old_queue_size as i32).abs() % 1000 == 0 {
            println!("queue size{} ", queue.len());
            old_queue_size = queue.len()
        }
        for next_key in key_vec.iter().filter(|key| !current_keys.contains(**key)) {
            for (i,_) in current_nodes.iter().enumerate() {

            let next_node = key_node_map[next_key];
            let mut next_nodes = current_nodes.clone();
            let current_node = current_nodes[i];
            next_nodes[i] = next_node;
            //  println!("can I get from {:?} to {:?}?", current_key, next_key);
            let cost = if path_map.contains_key(&(current_node, next_key, current_keys)) {
                // println!("cache hit");
                path_map[&(current_node, next_key, current_keys)]
            } else {
             //   println!("looking for {:?} to {:?}", current_key, next_key);
              //  println!("{:?}", key_node_map);
                let result = astar(
                    &map_graph,
                    current_node,
                    |finish| finish == next_node,
                    |e| {
                        if !current_keys.contains(map_graph[e.target()].door)
                            && e.target() != key_node_map[next_key]
                        {
                            //println!("edge:{:?} -> {:?}",map_graph[e.source()].pos, map_graph[e.target()].pos);
                            9999999
                        } else {
                            //println!("edge:{:?} -> {:?}",map_graph[e.source()].pos, map_graph[e.target()].pos);
                            *e.weight()
                        }
                    },
                    |n| {
                        if !current_keys.contains(map_graph[n].door) {
                            9999999
                        } else {
                            let start_pos = map_graph[n].pos;
                            let end_pos = map_graph[next_node].pos;
                            (start_pos.x as i32 - end_pos.x as i32).abs() + (start_pos.y as i32 - end_pos.y as i32).abs()
                        }
                    },
                );
                match result {
                    Some((cost, _)) => cost,
                    None => 9999999,
                }
            };
            path_map.insert((current_node, next_key, current_keys), cost);
            // println!("maybe at cost:{}", cost);
            if cost >= 9999999 {
                continue;
            }
            // println!("yes at cost:{}", cost);

            let new_keys = *next_key | current_keys;
            let new_cost = current_cost + cost;
            let next_nodes_str = indices_to_string(&next_nodes);
            match cost_map.get_mut(&(next_nodes_str.clone(), new_keys)) {
                Some(cost) if *cost <= new_cost => continue,
                Some(cost) => *cost = new_cost,
                None => {
                    let _ = cost_map.insert((next_nodes_str, new_keys), new_cost);
                }
            }
            /*  println!(
                "currently have {:?} at cost {} considering {:?} with pathlen {} new cost:{}",
                current_keys,
                current_cost,
                path.end_key,
                path.path.len(),
                new_cost
            );*/

            // println!("pushing {:?} with cost {} ", path.end_key, new_cost);
            queue.push_back((new_cost, new_keys, next_nodes));
        }
    }
    }

    println!("win!:{}", min_cost);
}
