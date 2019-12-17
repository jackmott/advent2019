use intcomputer::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::SendError;
use std::thread;

use PaintColor::*;
#[derive(Copy,Debug,Clone)]
enum PaintColor {
    Black,
    White,
}
impl PaintColor {
    fn from_int(i:i64) -> PaintColor{
        match i {
            0 => Black,
            1 => White,
            _ => panic!("color not supported"),
        }
    }
    fn to_int(&self) -> i64 {
        match self {
            Black => 0,
            White => 1,
        }
    }
}

use Dir::*;
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash,Copy,Debug,Clone)]
struct Pos {
    x: i64,
    y: i64,
}

struct Robot {
    pos: Pos,
    dir: Dir,
    // By using a hashmap here, we don't have to know the bounds of the hull
    // or allocate extra memory
    // and it simplifies getting the count of painted pieces
    paint_map: HashMap<Pos,PaintColor>,
}
impl Robot {
    fn new(start_color: PaintColor) -> Robot {
        let mut robot = Robot {
            pos: Pos { x: 0, y: 0 },
            dir: Up,
            paint_map: HashMap::new(),
        };
        robot.paint_map.insert(robot.pos,start_color);
        robot
    }
    fn get_current_color(&self) -> PaintColor {
        match self.paint_map.get(&self.pos) {
            Some(color) => *color,
            None => Black
        }
    }
    fn paint(&mut self, color: PaintColor) {
        self.paint_map.insert(self.pos,color);
    }
    fn turn(&mut self, d: i64) {
        let new_dir = match self.dir {
            Up => {
                if d == 0 {
                    Left
                } else {
                    Right
                }
            }
            Down => {
                if d == 0 {
                    Right
                } else {
                    Left
                }
            }
            Left => {
                if d == 0 {
                    Down
                } else {
                    Up
                }
            }
            Right => {
                if d == 0 {
                    Up
                } else {
                    Down
                }
            }
        };
        self.dir = new_dir;
    }
    fn next(&mut self) {
        let new_pos = match self.dir {
            Up => Pos {
                x: self.pos.x,
                y: self.pos.y - 1,
            },
            Down => Pos {
                x: self.pos.x,
                y: self.pos.y + 1,
            },
            Left => Pos {
                x: self.pos.x - 1,
                y: self.pos.y,
            },
            Right => Pos {
                x: self.pos.x + 1,
                y: self.pos.y,
            },
        };
        self.pos = new_pos;
    }
}

fn main() -> Result<(), SendError<i64>> {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut robot = Robot::new(White);
    // PART 1
    let (robot_output, computer_input) = channel();
    let (computer_output, robot_input) = channel();
    thread::spawn(move || {
        // Create a computer and run it
        let mut computer = SuperComputer::new(
            "Computer".to_string(),
            digits,
            computer_output,
            computer_input,
        );
        computer.run();
    });

    loop {
        match robot_output.send(robot.get_current_color().to_int()) {
            Ok(_) => (),
            Err(_) => break,
        }

        let color_to_paint = match robot_input.recv() {
            Ok(color) => PaintColor::from_int(color),
            Err(_) => break,
        };

        robot.paint(color_to_paint);

        match robot_input.recv() {
            Ok(dir) => robot.turn(dir),
            Err(_) => break,
        };
        robot.next();
    }
    println!("{}", robot.paint_map.len());

    // HashMap simplifies a lot of things but complicates printing the image
    let (min_x,max_x) = robot.paint_map.keys().map(|pos| pos.x).minmax().into_option().unwrap();
    let (min_y,max_y) = robot.paint_map.keys().map(|pos| pos.y).minmax().into_option().unwrap();
    for y in min_y .. max_y+1 {
        for x in min_x .. max_x+1 {
            let color =
                match robot.paint_map.get(&Pos { x: x, y: y}) {
                    Some(color) => *color,
                    None => Black,
                };
            match color {
                Black => print!(" "),
                White => print!("#"),
            }
        }
        println!("");
    }

    Ok(())
}
