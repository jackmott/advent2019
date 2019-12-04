struct Digits {
    digits: Vec<u8>,
}
impl Digits {
    fn new(num: i32) -> Digits {
        let mut digits: Vec<u8> = Vec::new();
        let mut n = num;
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        digits.reverse();
        Digits { digits }
    }
    fn to_num(&self) -> i32 {
        let mut num = 0;
        let mut mul = 1;
        for i in (0..self.digits.len()).rev() {
            num = num + (self.digits[i] as i32) * mul;
            mul *= 10;
        }
        num
    }
    fn is_valid1(&self) -> bool {
        let mut digits_count: [i32; 10] = [0; 10];
        for d in &self.digits {
            digits_count[*d as usize] += 1;
        }
        digits_count.iter().any(|count| *count >= 2)
    }

    fn is_valid2(&self) -> bool {
        let mut digits_count: [i32; 10] = [0; 10];
        for d in &self.digits {
            digits_count[*d as usize] += 1;
        }
        digits_count.iter().any(|count| *count == 2)
    }
}

// todo - is there a way to not have to clone the array?
impl Iterator for Digits {
    type Item = Digits;

    fn next(&mut self) -> Option<Digits> {
        let clone = self.digits.clone();
        let digits = &mut self.digits;
        let mut i = digits.len() - 1;
        loop {
            if digits[i] < 9 {
                digits[i] += 1;
                for j in i + 1..digits.len() {
                    digits[j] = digits[i];
                }
                return Some(Digits { digits: clone });
            } else {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }
        None
    }
}

fn main() {
    let answer1 = Digits::new(388888)
        .filter(Digits::is_valid1)
        .take_while(|digits| digits.to_num() < 843167)
        .count();
    println!("answer1:{}", answer1);

    let answer2 = Digits::new(388888)
        .filter(Digits::is_valid2)
        .take_while(|digits| digits.to_num() < 843167)
        .count();
    println!("answer:{}", answer2);
}
