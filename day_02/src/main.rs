use utils::*;

struct SuperComputer {
    digits:Vec<i64>,
}

impl SuperComputer {
    fn new(digits:&Vec<i64>) -> SuperComputer {
        SuperComputer {
            digits:digits.clone(),
        }
    }

    fn run(&mut self) -> i64 {
        for i in (0..self.digits.len()).step_by(4) {
            if self.digits[i] == 99 {
                break;
            } else {
                let p1 = self.digits[i+1];
                let p2 = self.digits[i+2];
                let p3 = self.digits[i+3];
                let a = self.digits[p1 as usize];
                let b = self.digits[p2 as usize];
                self.digits[p3 as usize] = match self.digits[i] {
                    1 => a+b,
                    2 => a*b,
                    _ => panic!("error")
                };
            }
        }
        self.digits[0]
    }
}

fn part1(digits:&Vec<i64>) {
    let r = SuperComputer::new(digits).run();
    println!("{}",r);
    clip(format!("{}",r));
}


fn part2(digits:&Vec<i64>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut digits = digits.clone();
            digits[1] = noun;
            digits[2] = verb;
            if SuperComputer::new(&digits).run() == 19690720 {
                println!("{}",100*noun+verb);
                clip(format!("{}",100*noun+verb));
            }
        }
    }

}

fn main() {
    let digits: Vec<i64> =  read_file("input.txt")
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    part1(&digits);
    part2(&digits);
}