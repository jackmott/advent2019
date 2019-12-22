use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use utils::*;
const MAX_DEPTH: i32 = 25;

struct Map {
    start: Pos,
    end: Pos,
    tiles: Vec<Tile>,
    inner_portals: HashMap<String, Pos>,
    outer_portals: HashMap<String, Pos>,
    w: usize,
    h: usize,
}
impl Map {
    fn new(tiles: Vec<Tile>, w: usize, h: usize) -> Map {
        //todo - redesign so we don't put in dummy values for start and end up front
        let mut map = Map {
            start: Pos { x: 0, y: 0 },
            end: Pos { x: 0, y: 0 },
            tiles,
            inner_portals: HashMap::new(),
            outer_portals: HashMap::new(),
            w,
            h,
        };
        map.load_horiz_portals();
        map.load_vert_portals();

        let start = map
            .tiles
            .iter()
            .find(|tile| tile.tile_type == Start)
            .unwrap()
            .pos;
        let end = map
            .tiles
            .iter()
            .find(|tile| tile.tile_type == End)
            .unwrap()
            .pos;

        map.start = start;
        map.end = end;

        map.inner_portals = map
            .tiles
            .iter()
            .filter_map(|tile| match &tile.tile_type {
                InnerPortal(s) => Some((s.to_string(), tile.pos)),
                _ => None,
            })
            .collect();

        map.outer_portals = map
            .tiles
            .iter()
            .filter_map(|tile| match &tile.tile_type {
                OuterPortal(s) => Some((s.to_string(), tile.pos)),
                _ => None,
            })
            .collect();

        debug_assert!(map.outer_portals.len() == map.inner_portals.len());
        map
    }

    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                match self
                    .get_tile(Pos {
                        x: x as i32,
                        y: y as i32,
                    })
                    .unwrap()
                    .tile_type
                {
                    Wall => print!("#"),
                    Space => print!("."),
                    InnerPortal(_) => print!("I"),
                    OuterPortal(_) => print!("O"),
                    PortalPiece(c) => print!("{}", c),
                    Start => print!("S"),
                    End => print!("E"),
                }
            }
            println!("");
        }
    }

    fn get_neighbors(&self, pos: Pos, depth: i32) -> Vec<Pos> {
        Dir::iter()
            .filter_map(|dir| self.get_tile(pos.dir(dir)))
            .filter(|tile| match tile.tile_type {
                Start | End if depth == 0 => true,
                InnerPortal(_) if depth <= MAX_DEPTH => true,
                OuterPortal(_) if depth > 0 => true,
                Space => true,
                _ => false,
            })
            .map(|tile| match &tile.tile_type {
                Start | End | Space => tile.pos,
                InnerPortal(s) => {
                    println!(
                        "at pos {:?} found portal {} goes to {:?}",
                        pos, s, self.outer_portals[s]
                    );
                    self.outer_portals[s]
                }
                OuterPortal(s) => {
                    println!(
                        "at pos {:?} found portal {} goes to {:?}",
                        pos, s, self.inner_portals[s]
                    );
                    self.inner_portals[s]
                }
                _ => panic!("previous filtering has gone bad"),
            })
            .collect()
    }

    fn get_tile(&self, pos: Pos) -> Option<&Tile> {
        let index = pos.y * self.w as i32 + pos.x;
        if pos.x < 0 || pos.x >= self.w as i32 || pos.y < 0 || pos.y >= self.h as i32 {
            None
        } else {
            Some(&self.tiles[index as usize])
        }
    }

    fn get_index(&self, pos: Pos) -> usize {
        pos.y as usize * self.w + pos.x as usize
    }

    fn portal_string_to_tile_horiz(&self, s: &str, pos: Pos) -> TileType {
        if s == "AA" {
            Start
        } else if s == "ZZ" {
            End
        } else if pos.x < 2 || pos.x >= self.w as i32 - 2 {
            OuterPortal(s.to_string())
        } else {
            InnerPortal(s.to_string())
        }
    }

    fn portal_string_to_tile_vert(&self, s: &str, pos: Pos) -> TileType {
        if s == "AA" {
            Start
        } else if s == "ZZ" {
            End
        } else if pos.y < 2 || pos.y >= self.h as i32 - 2 {
            OuterPortal(s.to_string())
        } else {
            InnerPortal(s.to_string())
        }
    }

    fn load_horiz_portals(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w - 1 {
                let pos = Pos::new(x, y);
                match self.get_tile(pos).unwrap().tile_type {
                    PortalPiece(c1) => {
                        let tile2 = self.get_tile(pos.right()).unwrap();
                        match tile2.tile_type {
                            PortalPiece(c2) => {
                                let s = c1.to_string() + &c2.to_string();
                                match self.get_tile(pos.right().right()) {
                                    Some(tile) if tile.tile_type == Space => {
                                        let index = self.get_index(pos.right().right());
                                        self.tiles[index].tile_type =
                                            self.portal_string_to_tile_horiz(&s, pos);
                                        let index = self.get_index(pos.right());
                                        self.tiles[index].tile_type = Wall;
                                        let index = self.get_index(pos);
                                        self.tiles[index].tile_type = Wall;
                                    }
                                    _ => {
                                        let index = self.get_index(pos.right());
                                        self.tiles[index].tile_type = Wall;
                                        let index = self.get_index(pos);
                                        self.tiles[index].tile_type = Wall;
                                        let index = self.get_index(pos.left());
                                        self.tiles[index].tile_type =
                                            self.portal_string_to_tile_horiz(&s, pos);
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn load_vert_portals(&mut self) {
        for x in 0..self.w {
            for y in 0..self.h - 1 {
                let pos = Pos::new(x, y);
                match self.get_tile(pos).unwrap().tile_type {
                    PortalPiece(c1) => {
                        let tile2 = self.get_tile(pos.down()).unwrap();
                        match tile2.tile_type {
                            PortalPiece(c2) => {
                                let s = c1.to_string() + &c2.to_string();
                                match self.get_tile(pos.down().down()) {
                                    Some(tile) if tile.tile_type == Space => {
                                        let index = self.get_index(pos.down().down());
                                        self.tiles[index].tile_type =
                                            self.portal_string_to_tile_vert(&s, pos);
                                        let index = self.get_index(pos.down());
                                        self.tiles[index].tile_type = Wall;
                                        let index = self.get_index(pos);
                                        self.tiles[index].tile_type = Wall;
                                    }
                                    _ => {
                                        let index = self.get_index(pos.down());
                                        self.tiles[index].tile_type = Wall;
                                        let index = self.get_index(pos);
                                        self.tiles[index].tile_type = Wall;
                                        let index = self.get_index(pos.up());
                                        self.tiles[index].tile_type =
                                            self.portal_string_to_tile_vert(&s, pos);
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
use Dir::*;
#[derive(EnumIter, Display, Copy, Clone, Debug, PartialEq, Hash, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {
            x: x as i32,
            y: y as i32,
        }
    }
    fn dir(&self, dir: Dir) -> Pos {
        match dir {
            Up => self.up(),
            Down => self.down(),
            Left => self.left(),
            Right => self.right(),
        }
    }
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
#[derive(Clone, Debug, PartialEq)]
enum TileType {
    Space,
    Wall,
    InnerPortal(String),
    OuterPortal(String),
    PortalPiece(char),
    Start,
    End,
}
impl TileType {
    fn from_char(c: char) -> TileType {
        match c {
            _ if c >= 'A' && c <= 'Z' => PortalPiece(c),
            '.' => Space,
            '#' => Wall,
            ' ' => Wall,
            _ => panic!("invalid tile:{}", c),
        }
    }
}

#[derive(Clone, Debug)]
struct Tile {
    tile_type: TileType,
    pos: Pos,
}
impl Tile {
    fn from_char(c: char, pos: Pos) -> Tile {
        Tile {
            tile_type: TileType::from_char(c),
            pos,
        }
    }
}

struct NodeData {
    pos: Pos,
}
impl NodeData {
    fn new(pos: Pos) -> NodeData {
        NodeData { pos }
    }
}

type MapGraph = Graph<NodeData, i32, Undirected, usize>;
fn build_graph(
    index: NodeIndex<usize>,
    map: &Map,
    graph: &mut MapGraph,
    pos_to_node: &mut HashMap<Pos, NodeIndex<usize>>,
) {
    let pos = graph[index].pos;

    for next_pos in map.get_neighbors(pos, 0) {
        let tile = map.get_tile(next_pos).unwrap();
        // make the portal edge cost 2
        let edge_cost = match tile.tile_type {
            OuterPortal(_) | InnerPortal(_) => 2,
            _ => 1,
        };
        match pos_to_node.get(&next_pos) {
            Some(next_index) => {
                let _ = graph.update_edge(index, *next_index, edge_cost);
            }
            None => {
                let next_index = graph.add_node(NodeData::new(next_pos));
                let _ = graph.update_edge(index, next_index, edge_cost);
                pos_to_node.insert(next_pos, next_index);
                build_graph(next_index, map, graph, pos_to_node);
            }
        }
    }
}

struct InceptionNodeData {
    pos: Pos,
}
impl InceptionNodeData {
    fn new(pos: Pos) -> InceptionNodeData {
        InceptionNodeData { pos }
    }
}

type InceptionMapGraph = Graph<InceptionNodeData, i32, Undirected, usize>;
fn build_inception_graph(
    stack: &mut Vec<(NodeIndex<usize>, i32)>,
    map: &Map,
    graph: &mut InceptionMapGraph,
    pos_to_node: &mut HashMap<(Pos, i32), NodeIndex<usize>>,
) {
    while stack.len() > 0 {
        let (index, depth) = stack.pop().unwrap();

        let pos = graph[index].pos;

        for next_pos in map.get_neighbors(pos, depth) {
            let tile = map.get_tile(next_pos).unwrap();
            // make the portal edge cost 2
            let edge_cost = match tile.tile_type {
                OuterPortal(_) | InnerPortal(_) => 2,
                _ => 1,
            };
            //This is backwards becase we are lookinga the other side
            let depth_change: i32 = match tile.tile_type {
                InnerPortal(_) => -1,
                OuterPortal(_) => 1,
                _ => 0,
            };

            match pos_to_node.get(&(next_pos, depth)) {
                Some(next_index) => {
                    let _ = graph.update_edge(index, *next_index, edge_cost);
                }
                None => {
                    let next_index = graph.add_node(InceptionNodeData::new(next_pos));
                    let _ = graph.update_edge(index, next_index, edge_cost);
                    pos_to_node.insert((next_pos, depth), next_index);
                    stack.push((next_index, depth + depth_change));
                }
            }
        }
    }
}

fn main() {
    let mut w = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut y = 0;
    let mut lines = Vec::new();
    for line in read_file("input.txt") {
        if line.len() > w {
            w = line.len()
        }
        lines.push(line);
    }
    println!("w:{}", w);
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            let tile = Tile::from_char(
                c,
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
            );
            tiles.push(tile);
            x += 1;
        }
        while x < w {
            tiles.push(Tile {
                pos: Pos {
                    x: x as i32,
                    y: y as i32,
                },
                tile_type: Wall,
            });
            x += 1;
        }
        println!("x:{} w:{}", x, w);
        y += 1;
    }
    let h = y as usize;
    let mut map = Map::new(tiles, w, h);
    map.print();

    let mut graph: MapGraph = Graph::default();
    let start_index = graph.add_node(NodeData::new(map.start));
    let mut pos_to_node = HashMap::new();
    pos_to_node.insert(map.start, start_index);
    build_graph(start_index, &mut map, &mut graph, &mut pos_to_node);
    println!("graph node count:{}", graph.node_count());
    println!("start:{:?} end:{:?}", map.start, map.end);

    let (cost, _) = astar(
        &graph,
        start_index,
        |finish| graph[finish].pos == map.end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();

    println!("part 1 cost:{}", cost);

    let mut graph: InceptionMapGraph = Graph::default();
    let start_index = graph.add_node(InceptionNodeData::new(map.start));
    let mut pos_to_node = HashMap::new();
    pos_to_node.insert((map.start, 0), start_index);
    let mut stack = Vec::new();
    stack.push((start_index, 0));
    build_inception_graph(&mut stack, &mut map, &mut graph, &mut pos_to_node);
    println!("graph node count:{}", graph.node_count());
    println!("start:{:?} end:{:?}", map.start, map.end);

    let (cost, _) = astar(
        &graph,
        start_index,
        |finish| graph[finish].pos == map.end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();

    println!("part 2 cost:{}", cost);
}
