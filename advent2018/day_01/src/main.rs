use utils::*;
use std::collections::HashSet;

fn main() {
    let input : Vec<i64> = read_fromstr::<i64>("input.txt").collect();
    let result : i64 = input.iter().sum();
    println!("sum:{}",result);

    let mut nums = HashSet::new();
    let mut freq = 0;
    nums.insert(freq);
    'outer: loop {
        for n in &input {                    
            freq += n;            
            if nums.contains(&freq) {
                break 'outer;
            } else {                
                nums.insert(freq);
            }
        }
    }
    println!("freq:{}",freq);
}
