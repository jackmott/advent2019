use utils::*;

fn run_computer(digits: &mut Vec<i64>) -> i64 {
    for i in (0..digits.len()).step_by(4) {
        if digits[i] == 99 {
            break;
        } else {
            let pos_in1 = digits[i + 1] as usize;
            let pos_in2 = digits[i + 2] as usize;
            let a = digits[pos_in1];
            let b = digits[pos_in2];
            let pos_out = digits[i + 3] as usize;
            digits[pos_out] = 
                if digits[i] == 1 {
                    a + b
                } else if digits[i] == 2 {
                    a * b
                } else {
                    panic!("error");
                };
        }
    }
    digits[0]
}

fn part1() {
    let mut digits: Vec<i64> =  read_file("input.txt")
        .nth(0)
        .unwrap()        
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let r = run_computer(&mut digits);
    println!("{}",r);
    clip(format!("{}",r));
}


fn part2() {
    let original_digits: Vec<i64> =  read_file("input.txt")
    .nth(0)
    .unwrap()        
    .split(',')
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut digits = original_digits.clone();
            digits[1] = noun;
            digits[2] = verb;           
            if run_computer(&mut digits) == 19690720 {
                println!("{} {}",noun,verb);
                clip(format!("{}",100*noun+verb));
            }
        }
    }

}

fn main() {
    part1();
    part2();
}