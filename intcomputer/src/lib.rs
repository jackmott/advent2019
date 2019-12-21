use std::sync::mpsc::{Receiver, Sender,TryRecvError};


pub struct Term {
    sender:Sender<i64>,
    pub receiver:Receiver<i64>,
}
impl Term {

    pub fn new(sender:Sender<i64>,receiver:Receiver<i64>) -> Term {
        Term {
            sender,
            receiver
        }
    }
    pub fn send(&self,data:i64) {
        match self.sender.send(data) {
            Ok(_) => (),
            Err(err) => panic!("error sending:{}",err),
        }
    }

    pub fn recv(&self) -> i64 {
        match self.receiver.recv() {
            Ok(data) => data,
            _ => panic!("tried to recv but channel dead"),
        }
    }

    pub fn recv_till_block(&self) -> std::vec::IntoIter<i64> {
        let mut result = Vec::new();
        loop {
            match self.receiver.try_recv() {
                Ok(data) => result.push(data),
                Err(err) => match err {
                    TryRecvError::Empty => return result.into_iter(),
                    TryRecvError::Disconnected => panic!("terminal dead"),
                }
            }
        }
    }

    pub fn recv_till_disconnect(&self) -> std::vec::IntoIter<i64>  {
        let mut result = Vec::new();
        loop {
            match self.receiver.recv() {
                Ok(data) => result.push(data),
                Err(_) => return result.into_iter()
            }
        }
    }

    pub fn send_string(&self,s:&str) {
        for c in s.chars() {
            let data = c as i64;
            println!("sending:{} as {}",c,data);
            self.send(data);
        }
    }
}



// Feature in progress
use Command::*;
pub enum Command {
    Reset(Vec<i64>),
    Quit,
}

#[allow(dead_code)]
pub struct SuperComputer {
    pub name: String,
    pub digits: Vec<i64>,
    sp: usize,
    rb: i64,
    output_channel: Sender<i64>,
    input_channel: Receiver<i64>,
    command_channel: Option<Receiver<Command>>,
    pub last_output: Option<i64>,
}


use ParameterMode::*;
#[derive(Debug,Copy,Clone)]
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
        digits: Vec<i64>,
        output_channel: Sender<i64>,
        input_channel: Receiver<i64>,
    ) -> SuperComputer {
        SuperComputer::new_command(name,digits,output_channel,input_channel,None)
    }

    pub fn new_command(
        name: String,
        mut digits: Vec<i64>,
        output_channel: Sender<i64>,
        input_channel: Receiver<i64>,
        command_channel: Option<Receiver<Command>>,
    ) -> SuperComputer {
        // add ram
        digits.resize(digits.len()*10,0);
        SuperComputer {
            name,
            digits,
            sp: 0,
            rb: 0,
            output_channel,
            input_channel,
            last_output: None,
            command_channel: command_channel
        }
    }

    pub fn reset(&mut self,digits:&Vec<i64>) {
        let mut digits = digits.clone();
        digits.resize(digits.len()*10,0);
        self.sp = 0;
        self.rb = 0;
        self.digits = digits;
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
                    if let Some(command_channel) = &self.command_channel {
                        match command_channel.recv() {
                            Ok(command) => {
                                match command {
                                    Reset(digits) => {
                                        self.sp = 0;
                                        self.rb = 0;
                                        self.digits = digits.clone();
                                    }
                                    Quit => break
                                }
                            }
                            Err(_) => break
                        }
                    }
                    else {
                        break;
                    }
                }
                Add => {
                    let write_address = (self.digits[self.sp + 3] + self.get_offset(param_modes[2])) as usize;
                    self.digits[write_address] = input_params[0] + input_params[1];
                    self.sp += 4;
                }
                Mul => {
                    let write_address = (self.digits[self.sp + 3] + self.get_offset(param_modes[2])) as usize;
                    self.digits[write_address] = input_params[0] * input_params[1];
                    self.sp += 4;
                }
                Input => {
                    let write_address = (self.digits[self.sp + 1] + self.get_offset(param_modes[0])) as usize;
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
                        //println!("Computer {} sent: {}",self.name,input_params[0]);
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
                    let write_address = (self.digits[self.sp + 3] + self.get_offset(param_modes[2])) as usize;
                    self.digits[write_address] = if input_params[0] < input_params[1] {
                        1
                    } else {
                        0
                    };
                    self.sp += 4;
                }
                Equals => {
                    let write_address = (self.digits[self.sp + 3] + self.get_offset(param_modes[2])) as usize;
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

    fn get_offset(&self, mode: ParameterMode) -> i64 {
        match mode {
            Relative => self.rb,
            _ => 0,
        }
    }
}
