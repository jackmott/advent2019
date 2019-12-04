use utils::*;
use std::collections::HashMap;



fn find_diff_count_one(boxes:&Vec<String>) -> (String,String) {
    for b1 in boxes.iter() {
        for b2 in boxes.iter() {
            if b1.chars().zip(b2.chars()).filter(|(a,b)| a!=b).count() == 1 {
                return (b1.to_string(),b2.to_string());
            }
        }       
    }
    panic!("none found");
}

fn main() {
    
    let boxes : Vec<String>  = read_file("input.txt").collect();
    
    let mut two_count = 0;
    let mut three_count = 0;
    
    for b in boxes.iter() {      
        let mut letter_counts : HashMap<char,i32> = HashMap::new();
        for c in b.chars() {
            if letter_counts.contains_key(&c) {
                let v = letter_counts[&c];
                letter_counts.insert(c,v+1);
            } else {
                letter_counts.insert(c,1);
            }  
        }
        if letter_counts.values().any(|&n| n == 2) {
            two_count += 1;
        }
        if letter_counts.values().any(|&n| n == 3) {
            three_count +=1;
        }       
    }
    println!("checksum:{}",two_count*three_count);

    let (a,b) = find_diff_count_one(&boxes);
    a.chars().zip(b.chars()).for_each(|(a,b)| {
        if a == b {
            print!("{}",a);
        }
    });
    
           
}
