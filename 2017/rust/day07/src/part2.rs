use regex::Regex;
use std::collections::HashMap;

// WARNING: part2 implementation is NOT useable for different inputs!!
pub fn solve(input: &Vec<String>) -> i32 {
    let mut tower_lookup: HashMap<String, (i32, Vec<String>)> = HashMap::new();

    let re = Regex::new(
        r"(?P<name>^\w*) \((?P<weight>\d*)\)(?: -> (?P<others>.*))?").unwrap();

    for line in input {
        let things = re.captures(line).unwrap();

        let name = (&things["name"]).to_string();
        let weight = &things["weight"].to_string().parse::<i32>().unwrap();

        let others = match things.name("others") {
            Some(_) => {
                let others: Vec<String> = things["others"].split(", ").map(|s| s.to_string()).collect();
                others
            },
            None => Vec::new(),
        };

        tower_lookup.insert(name, (weight.clone(), others));
    }

    let root = super::part1::solve(input);
    find_unbalanced(&root, &tower_lookup);

    if let Some(tuple) = tower_lookup.get("egbzge") {
        println!("{:?}", tuple);
    }

    0
}

fn find_unbalanced(name: &String, lookup: &HashMap<String, (i32, Vec<String>)>) {
    if let Some(&(_, ref others)) = lookup.get(name) {
        let mut weights: Vec<i32> = Vec::new();
        let mut descriptions: Vec<(String, i32)> = Vec::new();
        for other in others {
            let weight = calc_tower_weight(other, lookup);
            weights.push(weight);
            descriptions.push((other.clone(), weight));
        }

        weights.sort();
        weights.dedup();

        if weights.len() > 1 {
            println!("name: {}, subtowers: {:?}", name, descriptions);
        }

        for other in others {
            find_unbalanced(other, lookup);
        }
    }
}

fn calc_tower_weight(name: &String, lookup: &HashMap<String, (i32, Vec<String>)>) -> i32 {
    if let Some(&(weight, ref others)) = lookup.get(name) {
        let mut sum = weight;
        for other in others {
            sum += calc_tower_weight(other, lookup);
        }

        return sum;
    }

    0
}
