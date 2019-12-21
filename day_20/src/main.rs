use bitflags::*;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::VecDeque;
use utils::*;
use std::ops::BitOr;

#[derive(Copy, Clone, Debug, PartialEq)]
struct PortalId(u64);
impl PortalId {
    fn from_chars(c1:char,c2:char) -> PortalId{
        let i1 = c1 as u64 - 'A' as u64;
        let i2 = c2 as u64 - 'A' as u64;
        let bitflag = 1 << i1+(i2+26);
        PortalId(bitflag)
    }

    fn empty() -> PortalId {
        PortalId(0)
    }

    fn contains(&self,id:PortalId) -> u64 {
        self.0 & id.0
    }
}

impl BitOr for PortalId {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        PortalId(self.0 | rhs.0)
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
    Portal(PortalId),
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
            _ if c >= 'A' && c <= 'Z' => Portal(PortalId::empty()),
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
            let tile = Tile::from_char(c, Pos { x, y });
            match tile.tile_type {
                Portal(_) => portal_count += 1,
                _ => (),
            }
            tiles.push(tile);
            x += 1;
        }
        y += 1;
    }
    println!("portal_count:{}",portal_count);
    let portalId1 = PortalId::from_chars('A','A');
    println!("portalId1:{:?}",portalId1);
    let portalId2 = PortalId::from_chars('A','B');
    println!("portalId2:{:?}",portalId2);
    let portalId2 = PortalId::from_chars('A','C');
    println!("portalId2:{:?}",portalId2);
    let portalId3 = PortalId::from_chars('B','A');
    println!("portalId3:{:?}",portalId3);
    let portalId4 = PortalId::from_chars('Z','Z');
    println!("portalId4:{:?}",portalId4);
    let portalId4 = PortalId::from_chars('A','Z');
    println!("portalId4:{:?}",portalId4);
    let portalId4 = PortalId::from_chars('B','Z');
    println!("portalId4:{:?}",portalId4);

    let test = PortalId::from_chars('A','A') |  PortalId::from_chars('A','B');
    println!("AA {:b}",PortalId::from_chars('A','A').0);
    println!("AB {:b}",PortalId::from_chars('A','B').0);
    println!("AA {:b}",PortalId::from_chars('Z','Z').0);
    println!("AB {:b}",test.contains(PortalId::from_chars('B','A')));
    println!("test contains: {}",test.contains(PortalId::from_chars('A','A')));
    println!("test contains: {}",test.contains(PortalId::from_chars('A','B')));
    println!("test contains: {}",test.contains(PortalId::from_chars('B','A')));




}
