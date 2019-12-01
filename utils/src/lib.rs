use clipboard::*;
use std::fs::File;
use std::io::{Lines,prelude::*, BufReader};

pub fn read_file(file_name:&str)  -> Lines<BufReader<File>> {
    let file = File::open(file_name).expect("could not open file");
    let reader = BufReader::new(file);
    reader.lines()
}  

pub fn read_ints(file_name:&str) -> impl Iterator<Item = i64>{
    read_file(file_name).map(|line_result| {
        match line_result {
            Ok(line) => line.parse::<i64>().unwrap(),
            Err(_) => panic!("failed to read line")
        }
    })
} 

pub fn clip(s:String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(s.to_owned()).unwrap();
}