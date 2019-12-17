use intcomputer::*;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread;

use Robot::*;
#[derive(Copy, Debug, Clone, PartialEq)]
enum Robot {
    Up,
    Down,
    Left,
    Right,
}
use Tile::*;
#[derive(Copy, Debug, Clone, PartialEq)]
enum Tile {
    Space,
    Scaffold(Option<Robot>),
}
impl Tile {
    fn from_int(i: i64) -> Tile {
        let c = i as u8 as char;
        match c {
            '#' => Scaffold(None),
            '.' => Space,
            '^' => Scaffold(Some(Up)),
            '>' => Scaffold(Some(Right)),
            'v' => Scaffold(Some(Down)),
            '<' => Scaffold(Some(Left)),
            _ => panic!("unknown input {}", c),
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Scaffold(None) => "#",
            Space => ".",
            Scaffold(Some(Up)) => "^",
            Scaffold(Some(Right)) => ">",
            Scaffold(Some(Left)) => "<",
            Scaffold(Some(Down)) => "v",
        }
    }
}

fn is_intersection(x: usize, y: usize, map: &Vec<Vec<Tile>>) -> bool {
    let mut count = 0;
    match map[y][x] {
        Space => {
            return false;
        }
        _ => (),
    }
    if x != 0 {
        match map[y][x - 1] {
            Scaffold(_) => count += 1,
            _ => (),
        }
    }

    if x != map[0].len() - 1 {
        match map[y][x + 1] {
            Scaffold(_) => count += 1,
            _ => (),
        }
    }

    if y != 0 {
        match map[y - 1][x] {
            Scaffold(_) => count += 1,
            _ => (),
        }
    }
    if y != map.len() - 1 {
        match map[y + 1][x] {
            Scaffold(_) => count += 1,
            _ => (),
        }
    }

    count == 4
}

fn part1() {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let (_, computer_input) = channel();
    let (computer_output, camera_input) = channel();

    thread::spawn(move || {
        // Create a computer and run it
        let mut computer =
            SuperComputer::new("Robut".to_string(), digits, computer_output, computer_input);
        computer.run();
    });

    let mut map = Vec::new();
    map.push(Vec::new());
    loop {
        match camera_input.recv() {
            Ok(input) => {
                if input == 10 {
                    map.push(Vec::new());
                } else {
                    let len = map.len();
                    map[len - 1].push(Tile::from_int(input));
                }
            }
            Err(_) => break,
        }
    }

    // There are two bogus rows at the end so drop them
    map.pop();
    map.pop();

    let mut sum = 0;
    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if is_intersection(x, y, &map) {
                print!("O");
                count += 1;
                sum += x * y;
            } else {
                print!("{}", map[y][x].to_string());
            }
        }
        println!("");
    }

    println!("alignment parameter sum:{} count:{}", sum, count);
}

fn part2() {
    let mut digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    digits[0] = 2;

    let (robot_output, computer_input) = channel();
    let (computer_output, robot_input) = channel();

    let term = Term::new(robot_output, robot_input);

    thread::spawn(move || {
        // Create a computer and run it
        let mut computer =
            SuperComputer::new("Robut".to_string(), digits, computer_output, computer_input);
        computer.run();
    });

    // This program arrived at using a neural network made of meat
    term.send_string("A,A,B,C,B,A,C,B,C,A\n");
    term.send_string("L,6,R,12,L,6,L,8,L,8\n");
    term.send_string("L,6,R,12,R,8,L,8\n");
    term.send_string("L,4,L,4,L,6\n");
    term.send_string("n\n");


    // It seems like the computer outputs the map or something
    // before the number so keep receiver till the end
    let data = term.recv_till_disconnect();

    println!("dust:{}", data.last().unwrap());
}

fn main() {
    part1();
    part2();
}
