use intcomputer::*;
use std::sync::mpsc::channel;
use std::thread;
use std::fs;

fn main() {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let (input_s, input_r) = channel();
    let (output_s, output_r) = channel();
    thread::spawn(move || {
        SuperComputer::new("Computer".to_string(), digits, output_s, input_r).run();
    });

    let _ = input_s.send(1);
    loop {
        match output_r.recv() {
            Ok(output) => println!("output:{}", output),
            Err(_) => break,
        }
    }
}
