use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashSet;
use utils::*;

struct Map {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
}
impl Map {
    fn get_tile(&self, pos: Pos) -> Option<(&Tile, usize)> {
        if pos.x as usize >= self.w || pos.y as usize >= self.h {
            None
        } else {
            let index = pos.y * self.w + pos.x;
            Some((&self.tiles[index], index))
        }
    }
    fn from_index(&self, index: usize) -> Pos {
        let x = index % self.w;
        let y = index / self.w;
        Pos { x, y }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
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
    Door(u8),
    Key(u8),
    Start,
}
impl TileType {
    fn is_open(&self, keys: &Vec<u8>) -> bool {
        match self {
            Space | Start | Key(_) => true,
            Door(key) => keys.iter().any(|k| key == k),
            Wall => false,
        }
    }

    fn from_char(c: char) -> TileType {
        match c {
            _ if c >= 'a' && c <= 'z' => Key(c as u8 - 'a' as u8),
            _ if c >= 'A' && c <= 'Z' => Door(c as u8 - 'A' as u8),
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
    visited_keys: HashSet<String>,
}
impl Tile {
    fn from_char(c: char, pos: Pos) -> Tile {
        Tile {
            tile_type: TileType::from_char(c),
            pos,
            visited_keys: HashSet::new(),
        }
    }

    fn is_open(&self, keys: &Vec<u8>) -> bool {
        self.tile_type.is_open(keys)
    }

    fn visit(&mut self, keys: &Vec<u8>) {
        let s: String = keys.iter().map(|i| i.to_string()).collect();
        self.visited_keys.insert(s);
    }

    fn is_visited(&self, keys: &Vec<u8>) -> bool {
        let s: String = keys.iter().map(|i| i.to_string()).collect();
        self.visited_keys.contains(&s)
    }
}

#[derive(Clone, Debug)]
struct NodeData {
    pos: Pos,
    keys: Vec<u8>,
}
impl NodeData {
    fn from_tile(tile: &Tile, keys: &Vec<u8>) -> NodeData {
        NodeData {
            pos: tile.pos,
            keys: keys.clone(),
        }
    }
}

fn get_neighbors(pos: Pos, map: &Map, keys: &Vec<u8>) -> Vec<usize> {
    let mut result = Vec::new();
    let up = map.get_tile(pos.up());
    match up {
        Some((up, index)) if up.is_open(keys) => result.push(index),
        _ => (),
    }

    let down = map.get_tile(pos.down());
    match down {
        Some((down, index)) if down.is_open(keys) => result.push(index),
        _ => (),
    }

    let left = map.get_tile(pos.left());
    match left {
        Some((left, index)) if left.is_open(keys) => result.push(index),
        _ => (),
    }

    let right = map.get_tile(pos.right());
    match right {
        Some((right, index)) if right.is_open(keys) => result.push(index),
        _ => (),
    }

    result
}

fn build_graph(
    start_index: NodeIndex<usize>,
    map: &mut Map,
    graph: &mut Graph<NodeData, i32, Undirected, usize>,
) {
    let mut stack: Vec<(NodeIndex<usize>, Vec<u8>)> = vec![(start_index, vec![])];
    let mut prev_stack_len = 0;
    while let Some((node_index, keys)) = stack.pop() {
        if stack.len() % 100 == 0 && prev_stack_len != stack.len() {
            println!("stack len:{}",stack.len());
            println!("graph count:{}",graph.node_count());
            prev_stack_len = stack.len();
        }

        let data = graph[node_index].clone();
        let (_, index) = map.get_tile(data.pos).unwrap();

        /*   println!(
            "visiting {:?} with {:?} at {:?} ",
            map.tiles[index].pos, keys, node_index
        );*/
        map.tiles[index].visit(&keys);
        for neighbor in get_neighbors(data.pos, map, &keys) {
            if !map.tiles[neighbor].is_visited(&keys) {
                match map.tiles[neighbor].tile_type {
                    Key(new_key) if !keys.iter().any(|k| *k == new_key) => {
                        let mut keys = keys.clone();
                        keys.push(new_key);
                        let neighbor_index =
                            graph.add_node(NodeData::from_tile(&map.tiles[neighbor], &keys));
                        graph.update_edge(node_index, neighbor_index, 1);
                        stack.push((neighbor_index, keys));
                    }
                    _ => {
                        let neighbor_index =
                            graph.add_node(NodeData::from_tile(&map.tiles[neighbor], &keys));
                        graph.update_edge(node_index, neighbor_index, 1);
                        stack.push((neighbor_index, keys.clone()));
                    }
                };
            }
        }
    }
}

fn main() {
    let mut w = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut y = 0;
    for line in read_file("input.txt") {
        w = line.len();
        let mut x = 0;
        for c in line.chars() {
            let tile = Tile::from_char(c, Pos { x, y });
            tiles.push(tile);
            x += 1;
        }
        y += 1;
    }
    let h = tiles.len() / w;
    let mut map = Map { tiles, w, h };

    let start_tile_index = map
        .tiles
        .iter()
        .position(|tile| match tile.tile_type {
            Start => true,
            _ => false,
        })
        .unwrap();

    let keys = Vec::new();
    let mut graph = Graph::<NodeData, i32, Undirected, usize>::default();
    let start_node_index = graph.add_node(NodeData::from_tile(&map.tiles[start_tile_index], &keys));
    //map.tiles[start_tile_index].visit(Keys::empty());

    build_graph(start_node_index, &mut map, &mut graph);
    let start_pos = map.tiles[start_tile_index].pos;
    /* ..exmaple 2
    let (count,path) = astar(&graph,start_node_index,|finish| graph[finish].keys == Keys::A | Keys::B | Keys::C | Keys::D | Keys::E | Keys::F,|_| 1,|n| {
        (start_pos.x as i32 - graph[n].pos.x as i32).abs()+(start_pos.y as i32 -graph[n].pos.y as i32).abs()
    }).unwrap();
    */
    println!("done building graph");
    let (cost, path) = astar(
        &graph,
        start_node_index,
        |finish| graph[finish].keys.len() == 10,
        |e| *e.weight(),
        |n| {
            (start_pos.x as i32 - graph[n].pos.x as i32).abs()
                + (start_pos.y as i32 - graph[n].pos.y as i32).abs()
        },
    )
    .unwrap();

    println!("node count:{}", graph.node_count());
    println!("path cost:{}", cost);
    println!("Path len:{}", path.len());

    let node_index = NodeIndex::new(40); //start_node_index;
    let node = graph[node_index].clone();
    println!("{:?}:{:?}   {:?}", node_index, node.pos, node.keys);
    for node in graph.neighbors(node_index) {
        print!("{:?} -> {:?},", node, graph[node]);
    }
}
