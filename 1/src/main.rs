use utils::*;

fn calc_fuel(total:i64,mass:i64) -> i64 {
    let fuel = mass/3-2;
    if fuel <= 0 {
        total
    } else {
        calc_fuel(total+fuel,fuel)
    }
}

fn main() {
    let result : i64 = read_fromstr::<i64>("input.txt").map(|mass| mass/3-2).sum();
    println!("{}",result);
    let result : i64 = read_fromstr("input.txt").map(|mass| calc_fuel(0,mass)).sum();
    println!("{}",result);
    clip(result.to_string());
}
