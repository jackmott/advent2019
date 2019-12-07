use std::collections::HashMap;
use std::collections::HashSet;
use utils::*;

//Todo, clean this crap up
/*
struct SuperComputer {
    memory:Vec<i64>,
    sp:usize,
    input_queue:Vec<i64>
}

impl SuperComputer {

}*/

fn run_computer(digits: &mut [i64],sp:usize,input1: i64, input2:i64) -> (usize,Option<i64>) {
    let mut input_count = 0;
    let mut i = sp;
    loop {
        if digits[i] == 99 {
            return (i,None);
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
                digits[digits[i + 1] as usize] =
                    if input_count == 0 {
                        input_count +=1;
                        input1
                    } else {
                        input2
                    };
                i += 2;
            } else if op_code == 4 {
                let result = Some(digits[digits[i + 1] as usize]);
                i += 2;
                return (i,result);
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

fn perm_helper(len:usize, nums: &mut [i64]) -> Vec<Vec<i64>> {
    let mut result : Vec<Vec<i64>> = Vec::new();
    if len == 1 {
        result.push(nums.iter().map(|x| *x).collect());
    } else {
        result.append(&mut perm_helper(len-1,nums));
        for i in 0 .. (len-1) {
            if len % 2 == 0 {
                nums.swap(i,len-1);
            } else {
                nums.swap(0,len-1);
            }
            result.append(&mut perm_helper(len-1,nums));
        }
    }
    result
}

fn perm(mut nums: Vec<i64>) -> Vec<Vec<i64>> {
    perm_helper(nums.len(),&mut nums)
}


fn main() {
    let digits: Vec<i64> = read_file("input.txt")
    .nth(0)
    .unwrap()
    .split(',')
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    // part 1
    let nums : Vec<i64> = vec![0,1,2,3,4];
    let perms = perm(nums);

    let mut max = -999999;
    for perm in perms {
        let mut digits1 = digits.clone();
        let mut digits2 = digits.clone();
        let mut digits3 = digits.clone();
        let mut digits4 = digits.clone();
        let mut digits5 = digits.clone();

        let (_,a) = run_computer(&mut digits1,0,perm[0],0);
        let (_,b) = run_computer(&mut digits2,0,perm[1],a.unwrap());
        let (_,c) = run_computer(&mut digits3,0,perm[2],b.unwrap());
        let (_,d) = run_computer(&mut digits4,0,perm[3],c.unwrap());
        let (_,e) = run_computer(&mut digits5,0,perm[4],d.unwrap());
        if e.unwrap() > max {
            max = e.unwrap();

        }

    }
    println!("max:{}",max);

    // part2
    let nums : Vec<i64> = vec![5,6,7,8,9];
    let perms = perm(nums);
    max = -9999999;
    for perm in perms {

        let mut digits1 = digits.clone();
        let mut digits2 = digits.clone();
        let mut digits3 = digits.clone();
        let mut digits4 = digits.clone();
        let mut digits5 = digits.clone();

        let (mut asp,a) = run_computer(&mut digits1,0,perm[0],0);
        let (mut bsp,b) = run_computer(&mut digits2,0,perm[1],a.unwrap());
        let (mut csp,c) = run_computer(&mut digits3,0,perm[2],b.unwrap());
        let (mut dsp,d) = run_computer(&mut digits4,0,perm[3],c.unwrap());
        let (mut esp,e) = run_computer(&mut digits5,0,perm[4],d.unwrap());
        let mut e = e.unwrap();

        loop {
            let (sp,a) = match run_computer(&mut digits1,asp,e,e) { (i,Some(n)) => (i,n), (_,None) => break};
            asp = sp;
            let (sp,b) = match run_computer(&mut digits2,bsp,a,a) { (i,Some(n)) => (i,n), (_,None) => break};
            bsp = sp;
            let (sp,c) = match run_computer(&mut digits3,csp,b,b) { (i,Some(n)) => (i,n), (_,None) => break};
            csp = sp;
            let (sp,d) = match run_computer(&mut digits4,dsp,c,c) { (i,Some(n)) => (i,n), (_,None) => break};
            dsp = sp;
            let (sp,temp_e) = match run_computer(&mut digits5,esp,d,d) { (i,Some(n)) => (i,n), (_,None) => break};
            e = temp_e;
            esp = sp;
        }
        if e > max { max = e;}


    }
    println!("max:{}",max);

}
