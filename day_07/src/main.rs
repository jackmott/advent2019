use intcomputer::*;
use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread;
use std::fs;

fn perm_helper(len: usize, nums: &mut [i64]) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = Vec::new();
    if len == 1 {
        result.push(nums.iter().map(|x| *x).collect());
    } else {
        result.append(&mut perm_helper(len - 1, nums));
        for i in 0..(len - 1) {
            if len % 2 == 0 {
                nums.swap(i, len - 1);
            } else {
                nums.swap(0, len - 1);
            }
            result.append(&mut perm_helper(len - 1, nums));
        }
    }
    result
}

fn perm(mut nums: Vec<i64>) -> Vec<Vec<i64>> {
    perm_helper(nums.len(), &mut nums)
}

fn main() -> Result<(), SendError<i64>> {
    let digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // PART 1

    // Phase Settings
    let nums: Vec<i64> = vec![0, 1, 2, 3, 4];
    let perms = perm(nums);

    let mut max = -999999;
    // For each permutation of phase settings
    for perm in perms {
        // Start off the computation with perm[0] and 0 as inputs
        let (a_send, a_in) = channel();
        a_send.send(perm[0])?;
        a_send.send(0)?;

        // Build up input and output channels for each Amplifier
        let mut out_channels = VecDeque::<Sender<i64>>::new();
        let mut recv_channels = VecDeque::<Receiver<i64>>::new();
        recv_channels.push_back(a_in);
        for _ in 0..perm.len() {
            let (out, recv) = channel();
            out_channels.push_back(out);
            recv_channels.push_back(recv);
        }
        // Spawn a thread for each amplifier
        for i in 0..perm.len() {
            let out = out_channels.pop_front().unwrap();
            // Send the phase setting into the input of the NEXT machine
            if i + 1 < perm.len() {
                out.send(perm[i + 1])?;
            }
            // Set the input channel to the receiver of the PREVIOUS machine's output
            let recv = recv_channels.pop_front().unwrap();
            let digits_clone = digits.clone();
            // Spawn a thread and run the computer
            thread::spawn(move || {
                SuperComputer::new(i.to_string(), digits_clone, out, recv).run();
            });
        }
        // Get the output from the last am,plifier
        match recv_channels.pop_back().unwrap().recv() {
            Ok(last_amplifier_output) => {
                if last_amplifier_output > max {
                    max = last_amplifier_output;
                }
            }
            Err(_) => panic!("we broke"),
        }
    }
    println!("max:{}", max);

    // PART2

    // Phase settings
    let nums: Vec<i64> = vec![5, 6, 7, 8, 9];
    let perms = perm(nums);
    max = -9999999;

    // For each permutattion of phase settings
    for perm in perms {
        //Set up a channel to send the final output of the final amplifier
        let (result_send, result_recv) = channel();

        //Setup output and input channels for each amplifier
        let mut out_channels = VecDeque::<Sender<i64>>::new();
        let mut recv_channels = VecDeque::<Receiver<i64>>::new();
        for i in 0..perm.len() {
            let (out, recv) = channel();
            // For the last machine, we put its recv channel at the front
            // So that they become the input for the first machine
            // And we send the starting input so that will go to the first machine
            if i == perm.len() - 1 {
                out.send(perm[0])?;
                out.send(0)?;
                recv_channels.push_front(recv);
            } else {
                out.send(perm[i + 1])?;
                recv_channels.push_back(recv);
            }
            out_channels.push_back(out);
        }
        // Add the result channel to the end of the out channels
        out_channels.push_back(result_send);

        for i in 0..perm.len() {
            let digits_clone = digits.clone();
            let out = out_channels.pop_front().unwrap();
            // If we are on the last machine, get the result sending channel out
            // So we can move it into the thread
            let result_sender = if i == perm.len() - 1 {
                Some(out_channels.pop_front().unwrap())
            } else {
                None
            };
            let recv = recv_channels.pop_front().unwrap();
            // Spawn a thread for each amplifier
            thread::spawn(move || {
                // Create a computer and run it
                let mut computer = SuperComputer::new(i.to_string(), digits_clone, out, recv);
                computer.run();
                // If we have a final result sender, send the last output of this cmplifier
                match result_sender {
                    Some(sender) => {
                        let _ = sender.send(computer.last_output.unwrap());
                    }
                    None => (),
                }
            });
        }

        // Keep looping on the result sending channel until it dies
        // keep track of the max
        loop {
            match result_recv.recv() {
                Ok(last_machine_output) => {
                    if last_machine_output > max {
                        max = last_machine_output
                    }
                }
                Err(_) => break,
            }
        }
    }

    println!("max:{}", max);

    Ok(())
}
