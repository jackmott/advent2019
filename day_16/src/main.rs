use std::fs;
use utils::*;

fn part1() {
    let mut digits: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();

    let base_pattern = vec![0, 1, 0, -1];
    for _ in 0..100 {
        for i in 0..digits.len() - 1 {
            digits[i] = digits
                .iter()
                .enumerate()
                .skip(i) //optimization, since all will be * 0
                .map(|(j, d)| d * base_pattern[((j + 1) / (i + 1)) % 4])
                .sum::<i64>()
                .abs()
                % 10;
        }
    }

    println!("part1:{:?}", &digits[0..8]);
}

fn part2() {
    let input_chars: Vec<char> = fs::read_to_string("input.txt").unwrap().chars().collect();

    let offset = input_chars
        .iter()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut digits: Vec<i64> = input_chars
        .iter()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .cycle()
        .skip(offset)
        .take(10000 * input_chars.len() - offset) //slightly faster to skip then take, than vice versa
        .collect();

    // This pattern only works on the 2nd half of the array
    // But lo and behold, the offset is in the 2nd half
    for _ in 0..100 {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }

    println!("part2:{:?}", &digits[0..8]);
}

fn main() {
    part1();
    part2();
}
