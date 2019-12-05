use utils::*;
fn run_computer(digits: &mut [i64], input: i64) -> i64 {
    let mut i = 0;
    loop {
        if digits[i] == 99 {
            break;
        } else {
            let instruction = to_digits(digits[i]);

            // Extract op code from instruction
            let op_code = if instruction.len() == 1 {
                digits[i]
            } else {
                to_num(&instruction[instruction.len() - 2..])
            };

            //set param modes
            let mut params = [0, 0, 0];
            if instruction.len() > 2 {
                params[0] = instruction[instruction.len() - 3];
            }
            if instruction.len() > 3 {
                params[1] = instruction[instruction.len() - 4];
            }
            if instruction.len() > 4 {
                params[2] = instruction[instruction.len() - 5];
            }

            if op_code == 1 {
                let a = if params[0] == 0 {
                    digits[digits[i + 1] as usize]
                } else {
                    digits[i + 1]
                };
                let b = if params[1] == 0 {
                    digits[digits[i + 2] as usize]
                } else {
                    digits[i + 2]
                };
                digits[digits[i + 3] as usize] = a + b;
                i += 4;
            } else if op_code == 2 {
                let a = if params[0] == 0 {
                    digits[digits[i + 1] as usize]
                } else {
                    digits[i + 1]
                };
                let b = if params[1] == 0 {
                    digits[digits[i + 2] as usize]
                } else {
                    digits[i + 2]
                };
                digits[digits[i + 3] as usize] = a * b;
                i += 4;
            } else if op_code == 3 {
                digits[digits[i + 1] as usize] = input;
                i += 2;
            } else if op_code == 4 {
                println!("output:{}", digits[digits[i + 1] as usize]);
                i += 2;
            } else if op_code == 5 {
                let jump = if params[0] == 0 {
                    digits[digits[i + 1] as usize]
                } else {
                    digits[i + 1]
                };
                let i_ptr = if params[1] == 0 {
                    digits[digits[i + 2] as usize]
                } else {
                    digits[i + 2]
                };
                if jump != 0 {
                    i = i_ptr as usize;
                } else {
                    i += 3;
                }
            } else if op_code == 6 {
                let jump = if params[0] == 0 {
                    digits[digits[i + 1] as usize]
                } else {
                    digits[i + 1]
                };
                let i_ptr = if params[1] == 0 {
                    digits[digits[i + 2] as usize]
                } else {
                    digits[i + 2]
                };
                if jump == 0 {
                    i = i_ptr as usize;
                } else {
                    i += 3;
                }
            } else if op_code == 7 {
                let a = if params[0] == 0 {
                    digits[digits[i + 1] as usize]
                } else {
                    digits[i + 1]
                };
                let b = if params[1] == 0 {
                    digits[digits[i + 2] as usize]
                } else {
                    digits[i + 2]
                };
                digits[digits[i + 3] as usize] = if a < b { 1 } else { 0 };
                i += 4;
            } else if op_code == 8 {
                let a = if params[0] == 0 {
                    digits[digits[i + 1] as usize]
                } else {
                    digits[i + 1]
                };
                let b = if params[1] == 0 {
                    digits[digits[i + 2] as usize]
                } else {
                    digits[i + 2]
                };
                digits[digits[i + 3] as usize] = if a == b { 1 } else { 0 };
                i += 4;
            } else {
                panic!(format!("error:{}", op_code));
            };
        }
    }
    digits[0]
}

fn to_digits(num: i64) -> Vec<u8> {
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

fn main() {
    let mut digits: Vec<i64> = read_file("input.txt")
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut digits_clone = digits.clone();

    // part1
    run_computer(&mut digits[..], 1);
    // part2
    run_computer(&mut digits_clone[..], 5);
}
