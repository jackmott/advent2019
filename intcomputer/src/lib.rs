use std::sync::mpsc::{Receiver, Sender};

#[allow(dead_code)]
pub struct SuperComputer {
    pub name: String,
    pub digits: Vec<i64>,
    sp: usize,
    rb: i64,
    output_channel: Sender<i64>,
    input_channel: Receiver<i64>,
    pub last_output: Option<i64>,
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
    RelativeBase,
}
use OpCode::*;
impl From<i64> for OpCode {
    fn from(n: i64) -> Self {
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
    pub fn new(
        name: String,
        mut digits: Vec<i64>,
        output_channel: Sender<i64>,
        input_channel: Receiver<i64>,
    ) -> SuperComputer {
        // add ram
        for _ in 0..digits.len() * 2 {
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

    pub fn run(&mut self) {
        loop {
            let instruction = self.digits[self.sp];
            let op_code = OpCode::from(instruction % 100);

            //set param modes
            let param_modes = [
                ParameterMode::from((instruction / 100 % 10) as u8),
                ParameterMode::from((instruction / 100 / 10 % 10) as u8),
                ParameterMode::from((instruction / 100 / 10 / 10 % 10) as u8),
            ];

            let num_params = match op_code {
                Add | Mul | LessThan | Equals | JumpIfTrue | JumpIfFalse => 2,
                Input | Output | RelativeBase => 1,
                Halt => 0,
            };

            let mut input_params = Vec::new();
            for i in 0..num_params {
                let value = self.digits[self.sp + i + 1];
                match param_modes[i] {
                    Pointer => input_params.push(self.digits[value as usize]),
                    Relative => input_params.push(self.digits[(self.rb + value) as usize]),
                    Value => input_params.push(value),
                }
            }

            match op_code {
                Halt => {
                    //println!("{} Halting",self.name);
                    break;
                }
                Add => {
                    let offset = match param_modes[2] {
                        Relative => self.rb,
                        _ => 0,
                    };
                    let write_address = (self.digits[self.sp + 3] + offset) as usize;
                    self.digits[write_address] = input_params[0] + input_params[1];
                    self.sp += 4;
                }
                Mul => {
                    let offset = match param_modes[2] {
                        Relative => self.rb,
                        _ => 0,
                    };
                    let write_address = (self.digits[self.sp + 3] + offset) as usize;
                    self.digits[write_address] = input_params[0] * input_params[1];
                    self.sp += 4;
                }
                Input => {
                    let offset = match param_modes[0] {
                        Relative => self.rb,
                        _ => 0,
                    };
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
                    let offset = match param_modes[2] {
                        Relative => self.rb,
                        _ => 0,
                    };
                    let write_address = (self.digits[self.sp + 3] + offset) as usize;
                    self.digits[write_address] = if input_params[0] < input_params[1] {
                        1
                    } else {
                        0
                    };
                    self.sp += 4;
                }
                Equals => {
                    let offset = match param_modes[2] {
                        Relative => self.rb,
                        _ => 0,
                    };
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
