use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use utils::*;

struct SuperComputer {
    digits: Vec<i64>,
    sp: usize,
    output_channel: Sender<i64>,
    input_channel: Receiver<i64>,
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
        digits: Vec<i64>,
        output_channel: Sender<i64>,
        input_channel: Receiver<i64>,
    ) -> SuperComputer {
        SuperComputer {
            digits: digits,
            sp: 0,
            output_channel,
            input_channel,
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
                Halt => break,
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
                        Err(_) => break,
                    }
                }
                Output => match self.output_channel.send(input_params[0]) {
                    Ok(_) => self.sp += 2,
                    Err(_) => break,
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
                    self.digits[write_address] =
                        if input_params[0] < input_params[1] {
                            1
                        } else {
                            0
                        };
                    self.sp += 4;
                }
                Equals => {
                    let write_address = self.digits[self.sp + 3] as usize;
                    self.digits[write_address] =
                        if input_params[0] == input_params[1] {
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

fn main() {
    let digits: Vec<i64> = read_file("input.txt")
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let (input_s,input_r) = channel();
    let (output_s,output_r) = channel();
    thread::spawn( move || {
        SuperComputer::new(digits,output_s,input_r).run();
    });

    let _ = input_s.send(5);
    loop {
        match output_r.recv() {
            Ok(output) => println!("output:{}",output),
            Err(_) => break
        }
    }
}