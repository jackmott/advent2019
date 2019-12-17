use utils::*;
use vector::*;

fn is_in_the_way(a: Vector2, b: Vector2, c: Vector2) -> bool {
    let on_line = (c.x - a.x) / (b.x - a.x) - (c.y - a.y) / (b.y - a.y);
    if on_line.abs() > 0.00001 {
        return false;
    }
    let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

    if c.x < min_x || c.x > max_x || c.y < min_y || c.y > max_y {
        return false;
    }
    true
}

fn can_see(map: &Vec<bool>, w: usize, p1: usize, p2: usize) -> bool {
    !map.iter()
        .enumerate()
        .filter(|(p3, &is_asteroid)| is_asteroid && *p3 != p1 && *p3 != p2)
        .map(|(p3, _)| {
            let a = Vector2::from_index(p1, w);
            let b = Vector2::from_index(p2, w);
            let c = Vector2::from_index(p3, w);
            is_in_the_way(a, b, c)
        })
        .any(|is_in_the_way| is_in_the_way)
}

fn get_visible(map: &Vec<bool>, w: usize, p1: usize) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter(|(i, is_asteroid)| **is_asteroid && *i != p1 && can_see(&map, w, p1, *i))
        .map(|(i, _)| i)
        .collect()
}

fn count_visible(map: &Vec<bool>, w: usize, p1: usize) -> usize {
    map.iter()
        .enumerate()
        .filter(|(i, is_asteroid)| **is_asteroid && *i != p1 && can_see(&map, w, p1, *i))
        .count()
}

fn get_station(map: &Vec<bool>, w: usize) -> (usize, usize) {
    map.iter()
        .enumerate()
        .map(|(i, is_asteroid)| {
            if *is_asteroid {
                (i, count_visible(&map, w, i))
            } else {
                (i, 0)
            }
        })
        .max_by_key(|(_, count)| *count)
        .unwrap()
}

fn print_map(map: &Vec<bool>, w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let index = y * w + x;
            if map[index] == false {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}

fn main() {
    let input = read_file("input.txt");

    let mut asteroid_map = Vec::new();
    let mut w: usize = 0;
    for line in input {
        w = line.len();
        for c in line.chars() {
            asteroid_map.push(match c {
                '.' => false,
                '#' => true,
                _ => panic!("invalid input"),
            });
        }
    }
    let (station_index, count) = get_station(&asteroid_map, w);
    println!("part1:{}", count);

    let mut destroyed_asteroids: Vec<usize> = Vec::new();
    loop {
        println!("--------------");
        let mut visible_asteroids = get_visible(&asteroid_map, w, station_index);
        for a in &visible_asteroids {
            asteroid_map[*a] = false;
        }
        let station = Vector2::from_index(station_index, w);
        println!("station:{:?}", station);
        visible_asteroids.sort_by(|a, b| {
            let asteroid1 = Vector2::from_index(*a, w);
            let asteroid2 = Vector2::from_index(*b, w);
            station
                .angle_from_vertical(asteroid1)
                .partial_cmp(&station.angle_from_vertical(asteroid2))
                .unwrap()
        });
        destroyed_asteroids.append(&mut visible_asteroids);
        if destroyed_asteroids.len() >= 200 {
            let index = destroyed_asteroids[199];
            let v = Vector2::from_index(index, w);
            println!("{:?},{}", v, v.x * 100.0 + v.y);
            return;
        }
    }
}
