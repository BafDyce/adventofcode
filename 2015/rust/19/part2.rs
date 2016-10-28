// adventofcode - day 19
// part 2

use std::io::prelude::*;
use std::fs::File;

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
    println!("Advent of Code - day 19 | part 2");

    // import data
    let data = import_data();

    let (replacements, molecule) = parse_data(data);

    println!("Replacements possible:");
    for replacement in &replacements {
        replacement.print();
    }

    println!("\nMolecule: {}\n", molecule);

    let steps = dissolve_molecule(&molecule, &replacements, "e");

    println!("It took {} steps to create the molecule.", steps);
}

fn dissolve_molecule(base: &str, repls: &Vec<Replacement>, goal: &str) -> i32 {

    let mut mol = base.to_string();
    let mut ctr = 0;
    while mol != goal {
        for ref repl in repls {
            if mol.contains(&repl.to) {
                mol = replace_first(&mol, &repl.to, &repl.from);
                ctr += 1;
            }
        }
    }

    ctr
}

fn replace_first(string: &str, from: &str, to: &str) -> String {
    let matches: Vec<_> = string.match_indices(&from).collect();

    return if matches.len() > 0 {
        let (idx, _) = matches[0];

        // split the string into 3 strings:
        // a: substring BEFORE our element
        // _: the substring to replace
        // b: substring AFTER our element
        let (a, tmp) = string.split_at( idx );
        let (_, b) = tmp.split_at( from.len() );

        let mut result = a.to_string();
        result.push_str(&to);
        result.push_str(b);

        result
    } else {
        string.to_string()
    }
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
