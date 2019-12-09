use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread;
use std::collections::VecDeque;
use utils::*;

#[allow(dead_code)]
struct SuperComputer {
    name: String,
    digits: Vec<i64>,
    sp: usize,
    rb: i64,
    output_channel: Sender<i64>,
    input_channel: Receiver<i64>,
    last_output: Option<i64>,
}
use ParameterMode::*;
#[derive(Debug)]
enum ParameterMode {
    Value,
    Pointer,
    Relative,
}
impl From<u8> for ParameterMode {
    fn from(n: u8) -> Self {
        match n {
            0 => Pointer,
            1 => Value,
            2 => Relative,
            _ => panic!("invalid parameter mode"),
        }
    }
}
#[derive(Debug)]
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
    RelativeBase
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
            9 => RelativeBase,
            99 => Halt,
            _ => panic!("invalid op code"),
        }
    }
}

impl SuperComputer {
    fn new(
        name: String,
        mut digits: Vec<i64>,
        output_channel: Sender<i64>,
        input_channel: Receiver<i64>,
    ) -> SuperComputer {
        // add ram
        for _ in 0 .. digits.len()*10 {
            digits.push(0);
        }
        SuperComputer {
            name,
            digits,
            sp: 0,
            rb: 0,
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
                Add | Mul | LessThan | Equals |
                JumpIfTrue | JumpIfFalse => 2,
                Input | Output | RelativeBase => 1,
                Halt => 0,
            };

            let mut input_params = Vec::new();
            for i in 0..num_params {
                let value = self.digits[self.sp + i + 1];
                match param_modes[i] {
                    Pointer => input_params.push(self.digits[value as usize]),
                    Relative => input_params.push(self.digits[(self.rb+value) as usize]),
                    Value => input_params.push(value),
                }
            }

            match op_code {
                Halt => {
                    //println!("{} Halting",self.name);
                    break;
                }
                Add => {
                    let offset = match param_modes[2] { Relative => self.rb, _ => 0 };
                    let write_address = (self.digits[self.sp + 3] + offset) as usize;
                    self.digits[write_address] = input_params[0] + input_params[1];
                    self.sp += 4;
                }
                Mul => {
                    let offset = match param_modes[2] { Relative => self.rb, _ => 0 };
                    let write_address = (self.digits[self.sp + 3] + offset) as usize;
                    self.digits[write_address] = input_params[0] * input_params[1];
                    self.sp += 4;
                }
                Input => {
                    let offset = match param_modes[0] { Relative => self.rb, _ => 0 };
                    let write_address = (self.digits[self.sp + 1] + offset) as usize;
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
                    let offset = match param_modes[2] { Relative => self.rb, _ => 0 };
                    let write_address = (self.digits[self.sp + 3] + offset) as usize;
                    self.digits[write_address] = if input_params[0] < input_params[1] {
                        1
                    } else {
                        0
                    };
                    self.sp += 4;
                }
                Equals => {
                    let offset = match param_modes[2] { Relative => self.rb, _ => 0 };
                    let write_address = (self.digits[self.sp + 3] + offset) as usize;
                    self.digits[write_address] = if input_params[0] == input_params[1] {
                        1
                    } else {
                        0
                    };
                    self.sp += 4;
                }
                RelativeBase => {
                    self.rb += input_params[0];
                    self.sp += 2;
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



fn main() -> Result<(), SendError<i64>> {
    let digits: Vec<i64> = read_file("input.txt")
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // PART 1
    let (input_s, input_r) = channel();
    let (output_s, output_r) = channel();
    input_s.send(2)?;
    let mut computer = SuperComputer::new("Computer".to_string(), digits, output_s, input_r);
    computer.run();

    loop {
        match output_r.recv() {
            Ok(o) => println!("{}",o),
            Err(_) => break,
        }
    }
    println!("last output:{}",computer.last_output.unwrap());
    Ok(())
}
