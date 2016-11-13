// adventofcode - day 19
// part 1

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

struct Replacement {
    from: String,
    to: String,
}

impl Replacement {
    fn print(&self) {
        println!("Replacement: {} -> {}", self.from, self.to);
    }
}

fn main(){
    println!("Advent of Code - day 19 | part 1");

    // import data
    let data = import_data();

    let (replacements, molecule) = parse_data(data);

    println!("Possible replacements:");
    for replacement in &replacements {
        replacement.print();
    }

    println!("\nMolecule: {}\n", molecule);

    let mut results = HashSet::new();
    for replacement in replacements {
        // iterate over all occurences of the original element
        for (idx, _) in molecule.match_indices(&replacement.from) {
            // split the molecule into 3 strings:
            // a: substring BEFORE our element
            // _: our element
            // b: substring AFTER our element
            let (a, tmp) = molecule.split_at( idx );
            let (_, c) = tmp.split_at( replacement.from.len() );

            let mut mol = a.to_string();
            mol.push_str(&replacement.to);
            mol.push_str(c);

            // insert it into a HashSet (multiple creations of the same
            // molecule will be ignored)
            results.insert(mol);
        }
    }

    println!("Number of possible distinct alternations: {}", results.len());
}

fn parse_data(data: String) -> (Vec<Replacement>, String) {

    let mut lines = data.lines();

    let mut replacements = Vec::new();
    loop {
        let line = lines.next().unwrap();
        // an empty line marks the end of the replacements, next line will be
        // our molecule
        if line == "" {
            break;
        }

        let replacement = string_to_replacement(line);
        replacements.push( replacement );
    }

    let molecule = lines.next().unwrap();

    ( replacements, molecule.parse::<String>().unwrap() )
}

fn string_to_replacement(string: &str) -> Replacement {
    let properties = string.split(" => ")
                            .map(|s| s.parse::<String>().unwrap())
                            .collect::<Vec<String>>();

    Replacement { from: properties[0].clone(), to: properties[1].clone() }
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/19.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

    data.pop();
    data
}
