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
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
}
impl Map {
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

    fn get_neighbors(&self, tile: Tile) -> Vec<&Tile> {
        Dir::iter()
            .filter_map(|dir| self.get_tile(tile.pos.dir(dir)))
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
    fn visit(&mut self, pos: Pos) {
        let index = self.get_index(pos);
        self.tiles[index].visited = true;
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

fn PortalStringToTile(s:&str) -> TileType {
    if s == "AA" {
        Start
    } else if s == "ZZ" {
        End
    } else {
        Portal(s.to_string())
    }
}

fn LoadHorizontalPortals(map: &mut Map) {
    for y in 0..map.h {
        for x in 0..map.w - 1 {
            let pos = Pos::new(x, y);
            match map.get_tile(pos).unwrap().tile_type {
                PortalPiece(c1) => {
                    let tile2 = map.get_tile(pos.right()).unwrap();
                    match tile2.tile_type {
                        PortalPiece(c2) => {
                            let s = c1.to_string() + &c2.to_string();
                            match map.get_tile(pos.right().right()) {
                                Some(tile) if tile.tile_type == Space => {
                                    let index = map.get_index(pos.right());
                                    map.tiles[index].tile_type = PortalStringToTile(&s);
                                    let index = map.get_index(pos);
                                    map.tiles[index].tile_type = Wall;
                                }
                                _ => {
                                    let index = map.get_index(pos.right());
                                    map.tiles[index].tile_type = Wall;
                                    let index = map.get_index(pos);
                                    map.tiles[index].tile_type = PortalStringToTile(&s);
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

fn LoadVerticalPortals(map: &mut Map) {
    for x in 0..map.w {
        for y in 0..map.h - 1 {
            let pos = Pos::new(x, y);
            match map.get_tile(pos).unwrap().tile_type {
                PortalPiece(c1) => {
                    let tile2 = map.get_tile(pos.down()).unwrap();
                    match tile2.tile_type {
                        PortalPiece(c2) => {
                            let s = c1.to_string() + &c2.to_string();
                            match map.get_tile(pos.down().down()) {
                                Some(tile) if tile.tile_type == Space => {
                                    let index = map.get_index(pos.down());
                                    map.tiles[index].tile_type = PortalStringToTile(&s);
                                    let index = map.get_index(pos);
                                    map.tiles[index].tile_type = Wall;
                                }
                                _ => {
                                    let index = map.get_index(pos.down());
                                    map.tiles[index].tile_type = Wall;
                                    let index = map.get_index(pos);
                                    map.tiles[index].tile_type = PortalStringToTile(&s);
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

fn main() {
    let mut w = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut y = 0;
    let mut portal_count = 0;
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
            match tile.tile_type {
                Portal(_) => portal_count += 1,
                _ => (),
            }
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
                visited: false,
            });
            x += 1;
        }
        println!("x:{} w:{}", x, w);
        y += 1;
    }
    let h = y as usize;
    let mut map = Map { w, h, tiles };
    map.print();

    LoadHorizontalPortals(&mut map);
    LoadVerticalPortals(&mut map);
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    map.print();
}
