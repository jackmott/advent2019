use intcomputer::*;
use std::collections::HashSet;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::SendError;
use std::thread;

use Dir::*;
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn from_index(index: usize, w: usize) -> Pos {
        Pos {
            x: index % w,
            y: index / w,
        }
    }

    fn to_index(&self, w: usize) -> usize {
        self.y * w + self.x
    }
}
struct Robot {
    hull_width: usize,
    pos: Pos,
    hull: Vec<i64>,
    dir: Dir,
    paint_log: HashSet<usize>,
}
impl Robot {
    fn new(start_color: i64, w: usize, h: usize) -> Robot {
        let mut robot = Robot {
            hull_width: w,
            pos: Pos { x: w / 2, y: h / 2 },
            hull: vec![0; w * h],
            dir: Up,
            paint_log: HashSet::new(),
        };
        let index = robot.pos.to_index(w);
        robot.hull[index] = start_color;
        robot
    }
    fn get_current_color(&self) -> i64 {
        let index = self.pos.to_index(self.hull_width);
        self.hull[index]
    }
    fn paint(&mut self, color: i64) {
        let index = self.pos.to_index(self.hull_width);
        self.paint_log.insert(index);
        self.hull[index] = color;
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

    let w = 200;
    let h = 200;
    let mut robot = Robot::new(1, w, h);
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
        match robot_output.send(robot.get_current_color()) {
            Ok(_) => (),
            Err(_) => break,
        }

        let color_to_paint = match robot_input.recv() {
            Ok(color) => color,
            Err(_) => break,
        };

        robot.paint(color_to_paint);

        match robot_input.recv() {
            Ok(dir) => robot.turn(dir),
            Err(_) => break,
        };
        robot.next();
    }
    println!("{}", robot.paint_log.len());

    // Part 2
    for y in 0..h {
        for x in 0..w {
            let pos = Pos { x, y };
            match robot.hull[pos.to_index(w)] {
                1 => print!("#"),
                0 => print!(" "),
                _ => panic!("err"),
            }
        }
        println!("");
    }

    Ok(())
}
