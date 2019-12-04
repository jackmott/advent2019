use std::cell::{Ref, RefCell};
use std::time::Instant;

struct Digits<'a> {
    digits: &'a RefCell<Vec<u8>>,
    first: bool,
}

fn new_digits(num: i64) -> Vec<u8> {
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

fn is_valid1(digits: &[u8]) -> bool {
    let mut digits_count: [i32; 10] = [0; 10];
    for d in digits {
        digits_count[*d as usize] += 1;
    }
    digits_count.iter().any(|count| *count >= 2)
}

fn is_valid2(digits: &[u8]) -> bool {
    let mut digits_count: [i32; 10] = [0; 10];
    for d in digits {
        digits_count[*d as usize] += 1;
    }
    digits_count.iter().any(|count| *count == 2)
}
impl<'a> Iterator for Digits<'a> {
    type Item = Ref<'a, [u8]>;

    fn next(&mut self) -> Option<Ref<'a, [u8]>> {
        if self.first {
            self.first = false;
            return Some(Ref::map(self.digits.borrow(), Vec::as_slice));
        }
        let mut digits = self.digits.borrow_mut();
        let mut i = digits.len() - 1;
        loop {
            if digits[i] < 9 {
                digits[i] += 1;
                for j in i + 1..digits.len() {
                    digits[j] = digits[i];
                }
                drop(digits);
                return Some(Ref::map(self.digits.borrow(), Vec::as_slice));
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
    let now = Instant::now();
    let answer1 = Digits {
        digits: &RefCell::new(new_digits(388888)),
        first: true,
    }
    .filter(|d| is_valid1(d))
    .take_while(|d| to_num(d) <= 843167)
    .count();
    println!("answer1:{}", answer1);

    let answer2 = Digits {
        digits: &RefCell::new(new_digits(388888)),
        first: true,
    }
    .filter(|d| is_valid2(d))
    .take_while(|d| to_num(d) <= 843167)
    .count();
    println!("answer:{}", answer2);

    println!("{}", now.elapsed().as_millis());
}
