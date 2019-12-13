use intcomputer::*;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::mpsc::SendError;

fn main() -> Result<(), SendError<i64>> {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // PART 1
    let (input_s, input_r) = channel();
    let (output_s, output_r) = channel();
    input_s.send(1)?;
    let mut computer = SuperComputer::new("Computer".to_string(), digits, output_s, input_r);
    computer.run();

    loop {
        match output_r.try_recv() {
            Ok(o) => println!("{}", o),
            Err(_) => break,
        }
    }
    Ok(())
}
