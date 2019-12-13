use intcomputer::*;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::SendError;
use std::sync::mpsc::TryRecvError;
use std::thread;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

use Tile::*;
#[derive(PartialEq, Copy, Debug, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}
impl Tile {
    fn from_int(i: i64) -> Tile {
        match i {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            _ => panic!("invalid tile:{}", i),
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Empty => " ",
            Wall => "#",
            Block => "B",
            HorizontalPaddle => "_",
            Ball => "*",
        }
    }
}

fn main() -> Result<(), SendError<i64>> {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // PART 1
    let (game_output, computer_input) = channel();
    let (computer_output, game_input) = channel();
    thread::spawn(move || {
        // Create a computer and run it
        let mut computer =
            SuperComputer::new("Gamer".to_string(), digits, computer_output, computer_input);
        computer.run();
    });

    let mut tiles = HashMap::new();
    let mut score = 0;
    let mut paddle_pos = Pos { x: 0, y: 0 };
    let mut ball_pos = Pos { x: 0, y: 0 };

    loop {
        let x = match game_input.recv() {
            Ok(x) => x, // x
            Err(err) => break,
        };
        let y = match game_input.recv() {
            Ok(y) => y, // y
            Err(err) => {
                println!("err on y:{}", err);
                break;
            }
        };
        let pos = Pos { x, y };
        match game_input.recv() {
            Ok(input) => {
                if x != -1 {
                    let tile = Tile::from_int(input);
                    println!("{:?}", tile);
                    if tile == Ball {
                        ball_pos = pos;
                        println!("ball:{:?}", ball_pos);
                        let output = if paddle_pos.x < ball_pos.x {
                            1
                        } else if paddle_pos.x > ball_pos.x {
                            -1
                        } else {
                            0
                        };
                        //paddle_pos.x += output;
                        println!("sending output{}", output);
                        match game_output.send(output) {
                            Ok(_) => continue,
                            Err(err) => {
                                // println!("err:{}",err);
                                break;
                            }
                        }
                    } else if tile == HorizontalPaddle {
                        paddle_pos = pos;
                        println!("padl:{:?}", paddle_pos);
                    }
                    tiles.insert(Pos { x, y }, tile);
                } else {
                    score = input;
                    println!("score:{}", score);

                }
            }
            Err(err) => {
                println!("err on input:{}", err);
                break;
            }
        }
    }
    println!("{}", tiles.values().filter(|tile| **tile == Block).count());
    Ok(())
}
