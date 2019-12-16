use std::collections::HashSet;
use utils::*;

fn main() {
    let mut file = read_file("input.txt");
    let l1 = file.nth(0).unwrap();
    let l1 = l1.split(',').map(|s| s.chars());
    let l2 = file.nth(0).unwrap();
    let l2 = l2.split(',').map(|s| s.chars());

    let mut l1_points_hash = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut l1_points = vec![(x, y)];
    for mut line in l1 {
        let dir = line.next().unwrap();
        let number = line.as_str().parse::<i32>().unwrap();
        for _ in 0..number {
            match dir {
                'R' => x += 1,
                'D' => y += 1,
                'U' => y -= 1,
                'L' => x -= 1,
                _ => panic!("invalid"),
            }
            l1_points.push((x, y));
            l1_points_hash.insert((x, y));
        }
    }

    let mut candidate_points = Vec::new();

    let mut min_dist = std::i32::MAX;
    x = 0;
    y = 0;
    let mut l2_points = vec![(x, y)];
    l2_points.push((x, y));
    for mut line in l2 {
        let dir = line.next().unwrap();
        let number = line.as_str().parse::<i32>().unwrap();
        for _ in 0..number {
            match dir {
                'R' => x += 1,
                'D' => y += 1,
                'U' => y -= 1,
                'L' => x -= 1,
                _ => panic!("invalid"),
            }
            l2_points.push((x, y));
            if l1_points_hash.contains(&(x, y)) {
                candidate_points.push((x, y)); // needed for part2
                if x.abs() + y.abs() < min_dist {
                    min_dist = x.abs() + y.abs();
                }
            }
        }
    }
    //answer to part1
    println!("distance:{}", min_dist);

    //Now compute steps for all candidate points
    let mut min_steps = std::usize::MAX;
    for (px, py) in candidate_points {
        let l1_steps = l1_points
            .iter()
            .take_while(|(x, y)| *x != px || *y != py)
            .count();
        let l2_steps = l2_points
            .iter()
            .take_while(|(x, y)| *x != px || *y != py)
            .count();
        let sum = l1_steps + l2_steps;
        if sum < min_steps {
            min_steps = sum;
        }
    }

    println!("steps:{}", min_steps);
}
