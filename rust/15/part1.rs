// adventofcode - day 15
// part 1

use std::io::prelude::*;
use std::fs::File;

struct Ingredient {
    #[allow(dead_code)]
    name: String,
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    #[allow(dead_code)]
    calories: i32,
}

fn main(){
    println!("Advent of Code - day 15 | part 1");

    // import data
    let data = import_data();

    let mut ingredients = Vec::new();
    for line in data.lines(){
        ingredients.push( parse_line(line) );
    }

    let mut teaspoons = Vec::with_capacity(ingredients.len());
    for _ in 0..ingredients.len(){
        teaspoons.push(0);
    }

    let mut max_score = 0;
    for ii in 0..101 {
        teaspoons[0] = ii;
        for jj in 0.. 101 - ii {
            teaspoons[1] = jj;
            for kk in 0 .. 101 - (ii + jj) {
                teaspoons[2] = kk;
                let ll = 100 - (ii + kk + jj);
                    teaspoons[3] = ll;
                    let score = calculate_recipe(&ingredients, &teaspoons);
                    if score > max_score {
                        max_score = score;
                    }
            }
        }
    }

    println!("Maximal score: {}", max_score);

}

fn calculate_recipe(ingredients: &Vec<Ingredient>, teaspoons: &Vec<i32>) -> i32{

    let mut capacity = 0;
    let mut durability = 0;
    let mut flavour = 0;
    let mut texture = 0;
    for ii in 0..ingredients.len() {
        capacity += ingredients[ii].capacity * teaspoons[ii];
        durability += ingredients[ii].durability * teaspoons[ii];
        flavour += ingredients[ii].flavour * teaspoons[ii];
        texture += ingredients[ii].texture * teaspoons[ii];
    }

    if capacity <= 0 || durability <= 0 || flavour <= 0 || texture <= 0 {
        return 0;
    }

    capacity * durability * flavour * texture
}

fn parse_line(line: &str) -> Ingredient {

    let properties = line.split(": capacity ")
                        .map(|s| s.parse::<String>().unwrap())
                        .collect::<Vec<String>>();
    let name = properties[0].clone();

    let properties = properties[1].split(", durability ")
                        .flat_map(|s| s.split(", flavor "))
                        .flat_map(|s| s.split(", texture "))
                        .flat_map(|s| s.split(", calories "))
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

    let capacity = properties[0];
    let durability = properties[1];
    let flavour = properties[2];
    let texture = properties[3];
    let calories = properties[4];

    Ingredient{ name: name,
                capacity: capacity,
                durability: durability,
                flavour: flavour,
                texture: texture,
                calories: calories}
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/15.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

	data
}
