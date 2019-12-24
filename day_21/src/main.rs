use intcomputer::*;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread;



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
        let mut computer =
            SuperComputer::new("Drone".to_string(), digits, computer_output, computer_input);

        computer.run();

    });

    term.send_stringln("NOT A J");
    term.send_stringln("NOT C T");
    term.send_stringln("OR T J");
    term.send_stringln("AND D J");
    term.send_stringln("WALK");

    println!("{}",term.recv_one_line());
    println!("{}",term.recv_one_line());
    println!("{}",term.recv_one_line());
    println!("{}",term.recv_one_line());
    println!("result:{}",term.recv());

}


fn part2() {
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
        let mut computer =
            SuperComputer::new("Drone".to_string(), digits, computer_output, computer_input);

        computer.run();

    });



    term.send_stringln("NOT C T");
    term.send_stringln("AND D T");
    term.send_stringln("NOT A J");
    term.send_stringln("OR T J");
    term.send_stringln("RUN");

    for line in term.recv_stringln() {
        println!("{}",line);
    }
/*
    println!("{}",term.recv_one_line());
    println!("{}",term.recv_one_line());
    println!("{}",term.recv_one_line());
    println!("{}",term.recv_one_line());
    println!("result:{}",term.recv());*/

}

fn main() {
//    part1();
part2();

}
