use std::cmp::{max, min};
use utils::*;
use vector::*;

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn apply_gravity(positions: &[Vector3], velocities: &mut [Vector3]) {
    let x_normal = Vector3 { x: -1, y: 0, z: 0 };
    let y_normal = Vector3 { x: 0, y: -1, z: 0 };
    let z_normal = Vector3 { x: 0, y: 0, z: -1 };

    for i in 0..positions.len() {
        let base_moon = positions[i];
        let x_point = Vector3 {
            x: base_moon.x,
            y: 1,
            z: 1,
        };
        let y_point = Vector3 {
            x: 1,
            y: base_moon.y,
            z: 1,
        };
        let z_point = Vector3 {
            x: 1,
            y: 1,
            z: base_moon.z,
        };
        for j in 0..positions.len() {
            if i != j {
                let update_moon = positions[j];
                let xside = (update_moon - x_point).dot(x_normal);
                if xside.abs() != 0 {
                    velocities[j].x += xside / xside.abs();
                }

                let yside = (update_moon - y_point).dot(y_normal);
                if yside.abs() != 0 {
                    velocities[j].y += yside / yside.abs();
                }

                let zside = (update_moon - z_point).dot(z_normal);
                if zside.abs() != 0 {
                    velocities[j].z += zside / zside.abs();
                }
            }
        }
    }
}

fn update_positions(positions: &mut [Vector3], velocities: &[Vector3]) {
    positions
        .iter_mut()
        .zip(velocities.iter())
        .for_each(|(p, v)| *p = *p + *v);
}

fn state_equal(
    p1: &[Vector3],
    p2: &[Vector3],
    v1: &[Vector3],
    v2: &[Vector3],
) -> (bool, bool, bool) {
    let mut x = true;
    let mut y = true;
    let mut z = true;
    for i in 0..p1.len() {
        if p1[i].x != p2[i].x || v1[i].x != v2[i].x {
            x = false;
        }
        if p1[i].y != p2[i].y || v1[i].y != v2[i].y {
            y = false;
        }
        if p1[i].z != p2[i].z || v1[i].z != v2[i].z {
            z = false;
        }
    }
    (x, y, z)
}

fn main() {
    let positions_initial: Vec<Vector3> = read_file("input.txt")
        .map(|s| {
            let mut split = s.split(',');
            let xstr = split.next().unwrap().trim();
            let eq_pos = xstr.find('=').unwrap();
            let x = xstr[eq_pos + 1..].parse::<i64>().unwrap();

            let ystr = split.next().unwrap().trim();
            let eq_pos = ystr.find('=').unwrap();
            let y = ystr[eq_pos + 1..].parse::<i64>().unwrap();

            let zstr = split.next().unwrap().trim();
            let eq_pos = zstr.find('=').unwrap();
            let end_pos = zstr.find('>').unwrap();
            let z = zstr[eq_pos + 1..end_pos].parse::<i64>().unwrap();
            Vector3 { x, y, z }
        })
        .collect();

    let velocities_initial = vec![Vector3 { x: 0, y: 0, z: 0 }; positions_initial.len()];

    // part1
    let mut positions = positions_initial.clone();
    let mut velocities = velocities_initial.clone();

    for _ in 0..1000 {
        apply_gravity(&positions, &mut velocities);
        update_positions(&mut positions, &velocities);
    }

    let potential_energy = positions.iter().map(|p| p.x.abs() + p.y.abs() + p.z.abs());
    let kinetic_energy = velocities.iter().map(|v| v.x.abs() + v.y.abs() + v.z.abs());
    let total_energy: i64 = potential_energy
        .zip(kinetic_energy)
        .map(|(p, k)| p * k)
        .sum();

    println!("total energy:{}", total_energy);

    // part2
    let mut positions = positions_initial.clone();
    let mut velocities = velocities_initial.clone();
    let mut count = 1;
    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;
    // let mut set = HashSet::new();
    loop {
        apply_gravity(&positions, &mut velocities);
        update_positions(&mut positions, &velocities);
        let (x, y, z) = state_equal(
            &positions,
            &positions_initial,
            &velocities,
            &velocities_initial,
        );
        match x_period {
            None if x => x_period = Some(count),
            _ => (),
        }
        match y_period {
            None if y => y_period = Some(count),
            _ => (),
        }
        match z_period {
            None if z => z_period = Some(count),
            _ => (),
        }
        if x_period != None && y_period != None && z_period != None {
            break;
        }
        count += 1;
        if count % 100000 == 0 {
            println!("step:{}", count);
        }
    }
    let (x_period, y_period, z_period) = (x_period.unwrap(), y_period.unwrap(), z_period.unwrap());

    println!("x:{} y:{} z:{}", x_period, y_period, z_period);
    let lcm1 = lcm(x_period, y_period);
    let lcm2 = lcm(lcm1, z_period);
    println!("lcm2:{}", lcm2);
}
