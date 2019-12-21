use intcomputer::*;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread;

fn will_it_fit(map: Vec<Vec<u8>>,w:usize,h:usize, square_size:usize) -> Option<(usize,usize)> {
    for map_y in 0.. h-square_size {
        for map_x in 0 .. w-square_size {
            let mut fit = true;
            'loopy: for square_y in 0 .. square_size {
                let y = map_y + square_y;
                for square_x in 0 .. square_size
                {
                    let x = map_x + square_x;
                    if map[y][x] == 0 {
                        fit = false;
                        break 'loopy;
                    }
                }
            }
            if fit {
                return Some((map_x,map_y))
            }
        }
    }
    None
}

fn part1() {
    let mut digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let (drone_output, computer_input) = channel();
    let (computer_output, drone_input) = channel();

    let term = Term::new(drone_output, drone_input);

    thread::spawn(move || {
        // Create a computer and run it
        let dclone = digits.clone();
        let mut computer =
            SuperComputer::new("Drone".to_string(), digits, computer_output, computer_input);
        loop {
            computer.run();
            computer.reset(&dclone);
        }
    });

    let w :usize = 2000;
    let h :usize = 2000;

    let mut map : Vec<Vec<u8>> = Vec::new();
    let mut count = 0;
    for y in 0 .. h {
        map.push(Vec::new());
        for x in 0 .. w {
            term.send(x as i64);
            term.send(y as i64);
            let result = term.recv();
            map[y as usize].push(result as u8);
            count += result;
        }
    }

    println!("count:{}",count);

    match will_it_fit(map,w,h,100) {
        Some((x,y)) => println!("fit:{},{}",x,y),
        None => println!("dunna fit")
    }






}

fn main() {
    part1();

}
