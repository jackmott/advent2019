use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::VecDeque;
use utils::*;
use std::ops::BitOr;

struct Map {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
}
impl Map {
    fn get_neighbors(&self,tile:Tile) -> Vec<&Tile> {
        let mut result = Vec::new();
        match self.get_tile(tile.pos.up()) {
            Some(tile) => result.push(tile),
            None => ()
        }
        match self.get_tile(tile.pos.down()) {
            Some(tile) => result.push(tile),
            None => ()
        }
        match self.get_tile(tile.pos.left()) {
            Some(tile) => result.push(tile),
            None => ()
        }
        match self.get_tile(tile.pos.right()) {
            Some(tile) => result.push(tile),
            None => ()
        }
        result
    }

    fn get_tile(&self, pos: Pos) -> Option<&Tile> {
        let index = pos.y * self.w as i32 + pos.x;
        if index < 0 || index >= self.tiles.len() as i32 {
            None 
        } else {
            Some(&self.tiles[index as usize])
        }
    }

    fn get_index(&self, pos: Pos) -> usize {
        pos.y as usize * self.w + pos.x as usize
    }
    fn from_index(&self,index:usize) -> Pos {
        Pos {
            x: (index%self.w) as i32,
            y: (index/self.w) as i32
        }
    }
    fn visit(&mut self, pos: Pos) {
        let index = self.get_index(pos);
        self.tiles[index].visited = true;
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
struct Pos {
    x: i32,
    y: i32,
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


fn main() {
    let mut w = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut y = 0;
    let mut portal_count = 0;
    for line in read_file("input.txt") {
        w = line.len();
        let mut x = 0;
        for c in line.chars() {
            let tile = Tile::from_char(c, Pos { x:x as i32, y:y as i32});
            match tile.tile_type {
                Portal(_) => portal_count += 1,
                _ => (),
            }
            tiles.push(tile);
            x += 1;
        }
        y += 1;
    }
    let h = y as usize;
    let mut map = Map {w,h,tiles};
    for y in 0 .. h {
        for x in 0..2 {
            let pos = Pos {x:x as i32,y:y as i32};
            let tile1 = map.get_tile(pos).unwrap();
            match tile1.tile_type {
                PortalPiece(c1) => {
                    let tile2 = map.get_tile(pos.right()).unwrap();
                    match tile2.tile_type {
                        PortalPiece(c2) => {
                            let s = c1.to_string()+&c2.to_string();
                            map.tiles[map.get_index(pos)].tile_type = Wall;
                            map.tiles[map.get_index(pos.right())].tile_type = Portal(s);
                        }
                        _ => panic!("bad input portal")
                    }
                }
                _ => ()
            }
        }
    }   




}
