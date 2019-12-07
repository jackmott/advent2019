use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread;
use utils::*;

struct SuperComputer {
    name: String,
    digits: Vec<i64>,
    sp: usize,
    output_channel: Sender<i64>,
    input_channel: Receiver<i64>,
    last_output: Option<i64>,
}
use ParameterMode::*;
enum ParameterMode {
    Value,
    Pointer,
}
impl From<u8> for ParameterMode {
    fn from(n: u8) -> Self {
        match n {
            0 => Pointer,
            1 => Value,
            _ => panic!("invalid parameter mode"),
        }
    }
}

enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}
use OpCode::*;
impl From<u8> for OpCode {
    fn from(n: u8) -> Self {
        match n {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            99 => Halt,
            _ => panic!("invalid op code"),
        }
    }
}

impl SuperComputer {
    fn new(
        name: String,
        digits: Vec<i64>,
        output_channel: Sender<i64>,
        input_channel: Receiver<i64>,
    ) -> SuperComputer {
        SuperComputer {
            name,
            digits,
            sp: 0,
            output_channel,
            input_channel,
            last_output: None,
        }
    }

    fn run(&mut self) {
        loop {
            let instruction = to_digits(self.digits[self.sp]);

            // Extract op code from instruction
            let op_code_num = if instruction.len() == 1 {
                instruction[0] as i64
            } else {
                to_num(&instruction[instruction.len() - 2..])
            };
            let op_code = OpCode::from(op_code_num as u8);

            //set param modes
            let mut param_modes = [Pointer, Pointer, Pointer];
            if instruction.len() > 2 {
                param_modes[0] = ParameterMode::from(instruction[instruction.len() - 3]);
            }
            if instruction.len() > 3 {
                param_modes[1] = ParameterMode::from(instruction[instruction.len() - 4]);
            }
            if instruction.len() > 4 {
                param_modes[2] = ParameterMode::from(instruction[instruction.len() - 5]);
            }

            let num_params = match op_code {
                Add | Mul | LessThan | Equals | JumpIfTrue | JumpIfFalse => 2,
                Input | Output => 1,
                Halt => 0,
            };
            let mut input_params = Vec::new();
            for i in 0..num_params {
                let value = self.digits[self.sp + i + 1];
                match param_modes[i] {
                    Pointer => input_params.push(self.digits[value as usize]),
                    Value => input_params.push(value),
                }
            }

            match op_code {
                Halt => {
                    //println!("{} Halting",self.name);
                    break;
                }
                Add => {
                    let write_address = self.digits[self.sp + 3] as usize;
                    self.digits[write_address] = input_params[0] + input_params[1];
                    self.sp += 4;
                }
                Mul => {
                    let write_address = self.digits[self.sp + 3] as usize;
                    self.digits[write_address] = input_params[0] * input_params[1];
                    self.sp += 4;
                }
                Input => {
                    let write_address = self.digits[self.sp + 1] as usize;
                    match self.input_channel.recv() {
                        Ok(input) => {
                            self.digits[write_address] = input;
                            self.sp += 2;
                        }
                        Err(_) => {
                            //println!("Input channel dead on {}",self.name);
                            break;
                        }
                    }
                }
                Output => match self.output_channel.send(input_params[0]) {
                    Ok(_) => {
                        self.last_output = Some(input_params[0]);
                        self.sp += 2;
                    }
                    Err(_) => {
                        self.last_output = Some(input_params[0]);
                        //println!("Output channel dead on {}",self.name);
                        break;
                    }
                },
                JumpIfTrue => {
                    if input_params[0] != 0 {
                        self.sp = input_params[1] as usize;
                    } else {
                        self.sp += 3;
                    }
                }
                JumpIfFalse => {
                    if input_params[0] == 0 {
                        self.sp = input_params[1] as usize;
                    } else {
                        self.sp += 3;
                    }
                }
                LessThan => {
                    let write_address = self.digits[self.sp + 3] as usize;
                    self.digits[write_address] = if input_params[0] < input_params[1] {
                        1
                    } else {
                        0
                    };
                    self.sp += 4;
                }
                Equals => {
                    let write_address = self.digits[self.sp + 3] as usize;
                    self.digits[write_address] = if input_params[0] == input_params[1] {
                        1
                    } else {
                        0
                    };
                    self.sp += 4;
                }
            }
        }
    }
}

fn to_digits(num: i64) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn to_num(digits: &[u8]) -> i64 {
    let mut num = 0;
    let mut mul = 1;
    for i in (0..digits.len()).rev() {
        num = num + (digits[i] as i64) * mul;
        mul *= 10;
    }
    num
}

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
    let digits: Vec<i64> = read_file("input.txt")
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // part 1
    let nums: Vec<i64> = vec![0, 1, 2, 3, 4];
    let perms = perm(nums);

    let mut max = -999999;
    for perm in perms {
        let digits_a = digits.clone();
        let digits_b = digits.clone();
        let digits_c = digits.clone();
        let digits_d = digits.clone();
        let digits_e = digits.clone();

        let (a_send, a_in) = channel();
        a_send.send(perm[0])?;
        a_send.send(0)?;
        let (a_out, a_recv) = channel();
        a_out.send(perm[1])?;
        thread::spawn(move || {
            SuperComputer::new("A".to_string(), digits_a, a_out, a_in).run();
        });

        let (b_out, b_recv) = channel();
        b_out.send(perm[2])?;
        thread::spawn(move || {
            SuperComputer::new("B".to_string(), digits_b, b_out, a_recv).run();
        });

        let (c_out, c_recv) = channel();
        c_out.send(perm[3])?;
        thread::spawn(move || {
            SuperComputer::new("C".to_string(), digits_c, c_out, b_recv).run();
        });

        let (d_out, d_recv) = channel();
        d_out.send(perm[4])?;
        thread::spawn(move || {
            SuperComputer::new("D".to_string(), digits_d, d_out, c_recv).run();
        });

        let (e_out, e_recv) = channel();
        thread::spawn(move || {
            SuperComputer::new("E".to_string(), digits_e, e_out, d_recv).run();
        });

        match e_recv.recv() {
            Ok(e) => {
                if e > max {
                    max = e
                }
            }
            Err(_) => panic!("we broke"),
        }
    }
    println!("max:{}", max);

    // part2
    let nums: Vec<i64> = vec![5, 6, 7, 8, 9];
    let perms = perm(nums);
    max = -9999999;

    for perm in perms {
        let digits_a = digits.clone();
        let digits_b = digits.clone();
        let digits_c = digits.clone();
        let digits_d = digits.clone();
        let digits_e = digits.clone();

        let (result_send, result_recv) = channel();
        let (a_out, a_recv) = channel();
        a_out.send(perm[1])?;
        let (b_out, b_recv) = channel();
        b_out.send(perm[2])?;
        let (c_out, c_recv) = channel();
        c_out.send(perm[3])?;
        let (d_out, d_recv) = channel();
        d_out.send(perm[4])?;
        let (e_out, e_recv) = channel();
        e_out.send(perm[0])?;
        e_out.send(0)?;

        thread::spawn(move || {
            SuperComputer::new("A".to_string(), digits_a, a_out, e_recv).run();
        });
        thread::spawn(move || {
            SuperComputer::new("B".to_string(), digits_b, b_out, a_recv).run();
        });
        thread::spawn(move || {
            SuperComputer::new("C".to_string(), digits_c, c_out, b_recv).run();
        });
        thread::spawn(move || {
            SuperComputer::new("D".to_string(), digits_d, d_out, c_recv).run();
        });
        thread::spawn(move || {
            let mut e = SuperComputer::new("E".to_string(), digits_e, e_out, d_recv);
            e.run();
            let _ = result_send.send(e.last_output.unwrap());
        });

        loop {
            match result_recv.recv() {
                Ok(e) => {
                    if e > max {
                        max = e
                    }
                }
                Err(_) => break,
            }
        }
    }

    println!("max:{}", max);

    Ok(())
}
