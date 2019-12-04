use std::collections::HashMap;
use utils::*;

#[derive(Debug, Copy, Clone)]
struct Claim {
    x: i32,
    y: i32,
    id: i32,
    w: i32,
    h: i32,
}

fn main() {
    let claims: Vec<Claim> = read_file("input.txt")
        .map(|s| {
            let first_space = s.find(' ').unwrap();
            let id = s[1..first_space].parse::<i32>().unwrap();
            let at = s.find('@').unwrap();
            let colon = s.find(':').unwrap();
            let mut positions = s[at + 2..colon].split(',');
            let x = positions.next().unwrap().parse::<i32>().unwrap();
            let y = positions.next().unwrap().parse::<i32>().unwrap();
            let mut size = s[colon + 2..].split('x');
            let w = size.next().unwrap().parse::<i32>().unwrap();
            let h = size.next().unwrap().parse::<i32>().unwrap();
            Claim { x, y, id, w, h }
        })
        .collect();

    let mut point_count: Vec<Vec<i32>> = vec![vec![]; 1000 * 1000];

    for claim in &claims {
        for y in claim.y..claim.y + claim.h {
            for x in claim.x..claim.x + claim.w {
                point_count[(y * 1000 + x) as usize].push(claim.id);
            }
        }
    }

    let result = point_count.iter().filter(|claims| claims.len() > 1).count();
    println!("result:{}", result);
    let winner = claims
        .iter()
        .filter(|claim| {
            !point_count
                .iter()
                .filter(|claims| claims.len() > 1)
                .any(|claim_ids| claim_ids.iter().any(|claim_id| *claim_id == claim.id))
        })
        .nth(0)
        .unwrap();

    println!("winner:{}", winner.id);
}
