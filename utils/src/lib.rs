use clipboard::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

// Return an iterator over each line of a file
pub fn read_file(file_name:&str)  -> impl Iterator<Item = String> {
    let file = File::open(file_name).expect("could not open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.expect("failed to read line from file"))
    
}  

// Load a file and return an iterator of parseable types, eg a file with an int on each line
pub fn read_fromstr<T>(file_name:&str) -> impl Iterator<Item = T>
where T: FromStr, <T as FromStr>::Err : std::fmt::Debug
{
    read_file(file_name).map(|line| line.parse::<T>().expect(&format!("unable to parse {}",line)))
} 

/// Put a string on the system clipboard
pub fn clip(s:String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(s.to_owned()).unwrap();
}