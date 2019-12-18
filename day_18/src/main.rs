use bitflags::*;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use utils::*;

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
    fn get_tile(&self, pos: Pos) -> Option<(&Tile,usize)> {
        if pos.x as usize >= self.w || pos.y as usize >= self.h {
            None
        } else {
            let index = pos.y * self.w + pos.x;
            Some((&self.tiles[index],index))
        }
    }
    fn from_index(&self, index: usize) -> Pos {
        let x = index % self.w;
        let y = index / self.w;
        Pos {
            x,
            y
        }
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
    Door(Keys),
    Key(Keys),
    Start,
}
impl TileType {
    fn is_open(&self, keys: Keys) -> bool {
        match self {
            Space | Start | Key(_) => true,
            Door(key) => keys.contains(*key),
            Wall => false,
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
    visited_keys: HashMap<Keys,bool>,
}
impl Tile {
    fn from_char(c: char, pos: Pos) -> Tile {
        Tile {
            tile_type: TileType::from_char(c),
            pos,
            visited_keys: HashMap::new()
        }
    }

    fn is_open(&self, keys: Keys) -> bool {
        self.tile_type.is_open(keys)
    }

    fn visit(&mut self, keys: Keys) {
        self.visited_keys.insert(keys,true);
    }

    fn is_visited(&self, keys: Keys) -> bool {
        if self.visited_keys.contains_key(&keys) {
            self.visited_keys[&keys]
        } else {
            false
        }
    }


}

#[derive(Copy, Clone, Debug)]
struct NodeData {
    pos: Pos,
    keys: Keys,
}
impl NodeData {
    fn from_tile(tile: &Tile, keys: Keys) -> NodeData {
        NodeData {
            pos: tile.pos,
            keys,
        }
    }
}

fn get_neighbors(pos: Pos, map: &Map, keys: Keys) -> Vec<usize> {
    let mut result = Vec::new();
    let up = map.get_tile(pos.up());
    match up {
        Some((up,index)) if up.is_open(keys) => result.push(index),
        _ => (),
    }

    let down = map.get_tile(pos.down());
    match down {
        Some((down,index)) if down.is_open(keys) => result.push(index),
        _ => (),
    }

    let left = map.get_tile(pos.left());
    match left {
        Some((left,index)) if left.is_open(keys) => result.push(index),
        _ => (),
    }

    let right = map.get_tile(pos.right());
    match right {
        Some((right,index)) if right.is_open(keys) => result.push(index),
        _ => (),
    }

    result
}

fn build_graph(
    tile_index:usize,
    node_index: NodeIndex<usize>,
    map: &mut Map,
    keys: Keys,
    graph: &mut Graph<NodeData, u8, Undirected, usize>,
) {
    let data = graph[node_index];
    let neighbors = get_neighbors(data.pos, map, keys);
    for neighbor in neighbors {
        if !map.tiles[neighbor].is_visited(keys) {
            map.tiles[neighbor].visit(keys);
            let new_keys =
                match map.tiles[neighbor].tile_type {
                    Key(new_key) => {
                        keys | new_key
                    }
                    _ => keys
                };
            println!("keys{}",new_keys.bits());
            let neighbor_index = graph.add_node(NodeData::from_tile(&map.tiles[neighbor], new_keys));
            graph.update_edge(node_index, neighbor_index, 1);
            build_graph(neighbor,neighbor_index,map,new_keys,graph);
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

    let keys = Keys::empty();
    let mut graph = Graph::<NodeData, u8, Undirected, usize>::default();
    let start_node_index = graph.add_node(NodeData::from_tile(&map.tiles[start_tile_index], keys));
    map.tiles[start_tile_index].visit(Keys::empty());

    build_graph(start_tile_index,start_node_index,&mut map,keys,&mut graph);
    println!("node count:{}",graph.node_count());


}
