// adventofcode - day 16
// part 1

use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

struct Aunt {
    id: i32,
    children: i32,
    cats: i32,
    samoyeds: i32,
    pomeranians: i32,
    akitas: i32,
    vizslas: i32,
    goldfish: i32,
    trees: i32,
    cars: i32,
    perfumes: i32,
}

impl Aunt {
    fn new(id: i32) -> Aunt {
        Aunt{
            id: id,
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1,
        }
    }

    fn matches(&self, other: &Aunt) -> bool {
        if (self.children == -1 || self.children == other.children)
            && (self.cats == -1 || self.cats == other.cats)
            && (self.samoyeds == -1 || self.samoyeds == other.samoyeds)
            && (self.pomeranians == -1 || self.pomeranians == other.pomeranians)
            && (self.akitas == -1 || self.akitas == other.akitas)
            && (self.vizslas == -1 || self.vizslas == other.vizslas)
            && (self.goldfish == -1 || self.goldfish == other.goldfish)
            && (self.trees == -1 || self.trees == other.trees)
            && (self.cars == -1 || self.cars == other.cars)
            && (self.perfumes == -1 || self.perfumes == other.perfumes) {
                return true;
            }

        false
    }

    fn print(&self) {
        print!("Sue {}: ", self.id);
        print!("{} children, ", self.children);
        print!("{} cats, ", self.cats);
        print!("{} samoyeds, ", self.samoyeds);
        print!("{} pomeranians, ", self.pomeranians);
        print!("{} akitas, ", self.akitas);
        print!("{} vizslas, ", self.vizslas);
        print!("{} goldfish, ", self.goldfish);
        print!("{} trees, ", self.trees);
        print!("{} cars, ", self.cars);
        println!("{} perfumes", self.perfumes);
    }
}

fn main(){
    println!("Advent of Code - day 16 | part 1");

    // import data
    let data = import_data();

    let mut aunts = Vec::new();

    let giver = Aunt{
        id: 0,
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    let mut linecount = 1i32;
    for mut line in data.lines(){
        let aunt = parse_line(&mut line, linecount);

        if aunt.matches(&giver) {
            aunts.push( aunt );
        }

        linecount += 1;
    }

    let matches = aunts.len();

    for aunt in aunts {
        aunt.print();
    }

    println!("Found {} matching aunts.", matches);

}

fn parse_line(line: &mut &str, number: i32) -> Aunt {
    let mut x = "Sue ".to_string();
    x.push_str( &number.to_string() );
    x.push_str(": ");
    eat(line, &x);

    let mut aunt = Aunt::new(number);

    let values = line.split(", ").collect::<Vec<_>>();

    for mut value in values {
        if eat(&mut value, "children: ") {
            aunt.children = i32::from_str(value).unwrap();
        } else if eat(&mut value, "cats: ") {
            aunt.cats = i32::from_str(value).unwrap();
        } else if eat(&mut value, "samoyeds: ") {
            aunt.samoyeds = i32::from_str(value).unwrap();
        } else if eat(&mut value, "pomeranians: ") {
            aunt.pomeranians = i32::from_str(value).unwrap();
        } else if eat(&mut value, "akitas: ") {
            aunt.akitas = i32::from_str(value).unwrap();
        } else if eat(&mut value, "vizslas: ") {
            aunt.vizslas = i32::from_str(value).unwrap();
        } else if eat(&mut value, "goldfish: ") {
            aunt.goldfish = i32::from_str(value).unwrap();
        } else if eat(&mut value, "trees: ") {
            aunt.trees = i32::from_str(value).unwrap();
        } else if eat(&mut value, "cars: ") {
            aunt.cars = i32::from_str(value).unwrap();
        } else if eat(&mut value, "perfumes: ") {
            aunt.perfumes = i32::from_str(value).unwrap();
        }
    }

    aunt
}

fn eat(s: &mut &str, expect: &str) -> bool {
    if s.starts_with(expect) {
        *s = &s[expect.len()..];
        true
    } else {
        false
    }
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/16.txt") {
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
