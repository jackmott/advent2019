use std::fs;
use utils::*;

fn part1() {
    let mut digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();

    let base_pattern = vec![0, 1, 0, -1];
    let mut digits_b = digits.clone();
    let mut sum_b = false; //so I can swap vecs rather than make a new one each time
    for _ in 0..100 {
        for i in 0..digits.len() - 1 {
            let mut sum = 0;
            //minor optimization, start at i because it zeros out
            for j in i..digits.len() {
                let pattern_digit = base_pattern[((j + 1) / (i + 1)) % 4];

                if sum_b {
                    sum += digits_b[j] * pattern_digit;
                } else {
                    sum += digits[j] * pattern_digit;
                }
            }
            sum = sum.abs();
            if sum >= 10 {
                sum = sum % 10;
            }
            if sum_b {
                digits[i] = sum;
            } else {
                digits_b[i] = sum;
            }
        }
        sum_b = !sum_b;
    }
    if !sum_b {
        println!("digits:{:?}", digits);
    } else {
        println!("digits:{:?}", digits_b);
    }
}

fn part2() {
    let original_digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();

    let mut expanded_digits = Vec::new();
    for i in 0..10000 * original_digits.len() {
        expanded_digits.push(original_digits[i % original_digits.len()]);
    }

    let offset = 5977567;
    let digits = &mut expanded_digits[offset..];
    // This pattern only works on the 2nd half of the array
    // But lo and behold, the offset is in the 2nd half
    for _ in 0..100 {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }

    println!("{:?}", &digits[0..8]);
}

fn main() {
    part1();
    part2();
}
