use intcomputer::*;
use std::fs;
use std::sync::mpsc::channel;

fn part1(digits: Vec<i64>) {
    let (_, input_r) = channel();
    let (output_s, _) = channel();
    let mut computer = SuperComputer::new("Computer".to_string(), digits, output_s, input_r);
    computer.run();
    println!("{}", computer.digits[0]);
}

fn part2(digits: Vec<i64>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut digits = digits.clone();
            digits[1] = noun;
            digits[2] = verb;
            let (_, input_r) = channel();
            let (output_s, _) = channel();
            let mut computer =
                SuperComputer::new("Computer".to_string(), digits, output_s, input_r);
            computer.run();
            if computer.digits[0] == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}

fn main() {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    part1(digits.clone());
    part2(digits.clone());
}
