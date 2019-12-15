use std::collections::HashMap;
use utils::*;

#[derive(Clone, Debug)]
struct Reaction {
    input_amount: u64,
    name: String,
    output_amount: u64,
}

#[derive(Clone, Debug)]
struct Chemical {
    name: String,
    produces: Vec<Reaction>,
}

// Recursively determine the amount needed of a given chemical to produce fuel amount of fuel
fn amount_needed(chemical: &str, graph: &HashMap<String, Chemical>, fuel: u64) -> u64 {
    let chemical = &graph[chemical];
    let mut total = 0;
    for reaction in &chemical.produces {
        let needed = if reaction.name == "FUEL" {
            //base case
            reaction.input_amount * fuel
        } else {
            let units_needed = (amount_needed(&reaction.name, graph, fuel) as f64
                / reaction.output_amount as f64)
                .ceil() as u64;
            reaction.input_amount * units_needed
        };
        total += needed;
    }
    total
}

fn main() {
    let mut graph = HashMap::new();
    for s in read_file("input.txt") {
        let mut parts = s.split("=>");
        let reagents = parts.next().unwrap().trim().split(',').map(|r| {
            let mut r = r.trim().split(' ');
            let numstr = r.next().unwrap().trim();
            let amount = numstr.parse::<u64>().unwrap();
            let chemical = r.next().unwrap().trim();
            (amount, chemical.to_string())
        });

        let mut result = parts.next().unwrap().trim().split(' ');
        let output_amount = result.next().unwrap().trim().parse::<u64>().unwrap();
        let output_chemical = result.next().unwrap().trim().to_string();

        reagents.for_each(|(input_amount, name)| {
            let reaction = Reaction {
                input_amount,
                name: output_chemical.to_string(),
                output_amount,
            };
            match graph.get_mut(&name) {
                None => {
                    graph.insert(
                        name.to_string(),
                        Chemical {
                            name: name.to_string(),
                            produces: vec![reaction],
                        },
                    );
                }
                Some(chemical) => chemical.produces.push(reaction),
            }
        });
    }

    for chemical in graph.values() {
        println!("{:?}", chemical);
    }

    //Part1
    println!("Amount needed:{}", amount_needed("ORE", &graph, 1));

    // Part2 binary search for the answer
    // Probably this could be done more directly but this was faster to figure out
    let trillion = 1000000000000;
    let mut low = 0;
    let mut high = trillion;
    loop {
        let m = (low + high) / 2;
        let needed = amount_needed("ORE", &graph, m);
        // Because we won't get a whole number answer, we check two cases
        if needed == trillion
            || (needed < trillion && amount_needed("ORE", &graph, m + 1) > trillion)
        {
            println!("fuel:{}", m);
            break;
        } else if needed > trillion {
            high = m - 1;
        } else if needed < trillion {
            low = m + 1;
        }
    }
}
