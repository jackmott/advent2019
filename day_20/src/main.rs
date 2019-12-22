use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::BitOr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use utils::*;

struct Map {
    start: Pos,
    end: Pos,
    tiles: Vec<Tile>,
    portal_map: HashMap<String, (Option<Pos>,Option<Pos>)>,
    w: usize,
    h: usize,
}
impl Map {
    fn new(tiles: Vec<Tile>, w: usize, h: usize) -> Map {

        //todo - redesign so we don't put in dummy values for start and end up front
        let mut map = Map {
            start:Pos{x:0,y:0},
            end:Pos{x:0,y:0},
            tiles,
            portal_map: HashMap::new(),
            w,
            h,
        };
        map.LoadHorizontalPortals();
        map.LoadVerticalPortals();

        let start = map.tiles
        .iter()
        .find(|tile| tile.tile_type == Start)
        .unwrap()
        .pos;
        let end = map.tiles
        .iter()
        .find(|tile| tile.tile_type == End)
        .unwrap()
        .pos;

        map.start = start;
        map.end = end;


        let portals: Vec<&Tile> = map
            .tiles
            .iter()
            .filter(|tile| match &tile.tile_type {
                Portal(_) => true,
                _ => false,
            })
            .collect();
        let portals = portals
            .iter()
            .map(|tile1| {
                println!("trying to find {:?}", tile1);
                let portal1_string = match &tile1.tile_type {
                    Portal(s) => s,
                    _ => panic!("wat"),
                };
                let tile2 = portals
                    .iter()
                    .find(|tile2| {
                        let portal2_string = match &tile2.tile_type {
                            Portal(s) => s,
                            _ => panic!("wat"),
                        };
                        tile2.pos != tile1.pos && portal1_string == portal2_string
                    })
                    .unwrap();
                (portal1_string.to_string(), tile2.pos)
            });
        let mut portal_map : HashMap<String,(Option<Pos>,Option<Pos>)> = HashMap::new();
        for (portal_string,pos) in portals {
            match portal_map.get(&portal_string) {
                Some((a,b)) => {
                    let a = *a;
                    let b = *b;
                    if a.is_some() {
                        portal_map.insert(portal_string,(a,Some(pos)));
                    } else {
                        portal_map.insert(portal_string,(b,Some(pos)));
                    }
                }
                None => { let _ = portal_map.insert(portal_string,(Some(pos),None));},
            };
        }
        map.portal_map = portal_map;
        println!("portal map len:{}", map.portal_map.len());
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
                    Portal(_) => print!("P"),
                    PortalPiece(c) => print!("{}", c),
                    Start => print!("S"),
                    End => print!("E"),
                }
            }
            println!("");
        }
    }

    fn get_only_adjacent_space(&self, pos: Pos) -> Pos {
        let mut iter = Dir::iter()
            .filter_map(|dir| self.get_tile(pos.dir(dir)))
            .filter(|tile| match tile.tile_type {
                Space => true,
                _ => false,
            });
        let result = iter.nth(0).unwrap();
        debug_assert!(iter.count() == 0);
        result.pos
    }

    fn get_neighbors(&self, pos: Pos) -> Vec<Pos> {
        Dir::iter()
            .filter_map(|dir| self.get_tile(pos.dir(dir)))
            .filter(|tile| match tile.tile_type {
                Space | Portal(_) | Start | End => true,
                _ => false,
            })
            .map(|tile| match &tile.tile_type {
                Start | End | Space => tile.pos,
                Portal(s) => {
                    println!("getting a portal naybor at pos {:?} named {} going to pos:",pos,s);
                    let (portal1,portal2)  = self.portal_map[s];
                    let portal1 = portal1.unwrap();
                    let portal2 = portal2.unwrap();
                    if portal1 != pos {
                        println!("{:?}",portal1);
                        portal1
                    } else {
                        println!("{:?}",portal2);
                        portal2
                    }
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
    fn from_index(&self, index: usize) -> Pos {
        Pos {
            x: (index % self.w) as i32,
            y: (index / self.w) as i32,
        }
    }

    fn PortalStringToTile(s: &str) -> TileType {
        if s == "AA" {
            Start
        } else if s == "ZZ" {
            End
        } else {
            Portal(s.to_string())
        }
    }

    fn LoadHorizontalPortals(&mut self) {
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
                                        self.tiles[index].tile_type = Map::PortalStringToTile(&s);
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
                                        self.tiles[index].tile_type = Map::PortalStringToTile(&s);
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

    fn LoadVerticalPortals(&mut self) {
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
                                        self.tiles[index].tile_type = Map::PortalStringToTile(&s);
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
                                        self.tiles[index].tile_type = Map::PortalStringToTile(&s);
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
    Portal(String),
    PortalPiece(char),
    Start,
    End,
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
    fn is_open(&self) -> bool {
        self.tile_type.is_open()
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

    for next_pos in map.get_neighbors(pos) {
        match pos_to_node.get(&next_pos) {
            Some(next_index) => {
                let _ = graph.update_edge(index, *next_index, 1);
            }
            None => {
                let next_index = graph.add_node(NodeData::new(next_pos));
                let _ = graph.update_edge(index, next_index, 1);
                pos_to_node.insert(next_pos, next_index);
                build_graph(next_index, map, graph, pos_to_node);
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
    pos_to_node.insert(map.start,start_index);
    build_graph(start_index,&mut map,&mut graph,&mut pos_to_node);
    println!("graph node count:{}",graph.node_count());
    println!("start:{:?} end:{:?}",map.start,map.end);

    let (cost,path) = astar(
        &graph,
        start_index,
        |finish| graph[finish].pos == map.end,
        |_| 1,
        |n| {
                let start_pos = graph[n].pos;
                let end_pos = map.end;
                (start_pos.x as i32 - end_pos.x as i32).abs()
                    + (start_pos.y as i32 - end_pos.y as i32).abs()

        },
    ).unwrap();

    println!("cost:{}",cost);

}
