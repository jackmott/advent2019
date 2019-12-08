use std::fs;

const BLACK: u8 = 0;
const ALPHA: u8 = 2;
const W: usize = 25;
const H: usize = 6;

fn main() {
    let pixels: Vec<u8> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();

    let layers: Vec<&[u8]> = pixels.chunks(W * H).collect();

    let min_zero_layer = layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|&v| *v == 0).count())
        .unwrap();

    let one_count = min_zero_layer.iter().filter(|&v| *v == 1).count();
    let two_count = min_zero_layer.iter().filter(|&v| *v == 2).count();

    // Part 1
    println!("{}", one_count * two_count);
    println!("");

    // Part 2
    let mut image = Vec::new();
    for i in 0..W * H {
        let pixel = layers.iter().find(|layer| layer[i] != ALPHA).unwrap()[i];
        image.push(pixel);
    }

    for y in 0..H {
        for x in 0..W {
            if image[y * W + x] == BLACK {
                print!(" ");
            } else {
                print!("X");
            }
        }
        println!("");
    }
}
