// adventofcode - day 13
// part 2


use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

struct Graph {
    nodes: Option<Vec<String>>,
    matrix: Option<Vec<Vec<i32>>>,
}

impl Graph {
    fn new(names: HashSet<String>) -> Graph {
        let mut graph = Graph{nodes: None, matrix: None};

        let size = names.len();

        graph.nodes = Some(Vec::with_capacity(size));
        match graph.nodes {
            Some(ref mut nodes) => {
                for name in names {
                    nodes.push(name.to_string());
                }
                nodes.sort();
            },
            None => {
                panic!("Failed to create graph!");
            }
        }

        graph.matrix = Some(Vec::<Vec<i32>>::with_capacity(size));
        match graph.matrix {
            Some(ref mut matrix) => {
                for ii in 0..size {
                    matrix.push(Vec::<i32>::with_capacity(size));
                    for _ in 0..size {
                        matrix[ii].push(0);
                    }
                }
            },
            None => {
                panic!("Failed to create graph!");
            }
        }

        graph
    }

    fn size(&self) -> usize {
        match self.nodes {
            Some(ref nodes) => nodes.len(),
            None => 0,
        }
    }

    #[allow(dead_code)]
    fn get_node_names(&self) -> Vec<String> {
        match self.nodes {
            Some(ref nodes) => nodes.clone(),
            None => Vec::<String>::new(),
        }
    }

    fn insert_edge(&mut self, src: &String, dest: &String, length: i32) {
        let src_idx = match self.nodes {
            Some(ref nodes) => match nodes.binary_search(src){
                Ok(x) => x,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                },
            },
            None => return,
        };
        let dst_idx = match self.nodes {
            Some(ref nodes) => match nodes.binary_search(dest){
                Ok(x) => x,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                },
            },
            None => return,
        };

        match self.matrix {
            Some(ref mut matrix) => {
                matrix[src_idx][dst_idx] = length;
            },
            None => return,
        }
    }

    fn calculate_happiness(&self, seated: Vec<usize>) -> (i32, Vec<usize>){

        if seated.len() == self.size() {
            return (self.calculate_happiness_of_table(&seated), seated);
        }

        let mut max_happiness = std::i32::MIN;
        let mut max_table = Vec::new();
        for ii in 0..self.size(){
            if ! seated.contains(&ii){
                let mut table = seated.clone();
                table.push(ii); // NOT table.flip() !
                let (happiness, table) = self.calculate_happiness(table);
                if happiness > max_happiness {
                    max_happiness = happiness;
                    max_table = table;
                }
            }
        }

        (max_happiness, max_table)
    }

    // bruteforce solution
    fn calculate_happiness_of_table(&self, table: &Vec<usize>) -> i32 {
        let mut seats = table.iter();
        let first = seats.next().unwrap();
        let mut from = first;
        let mut happiness = 0i32;
        loop {
            match seats.next() {
                Some(to) => {
                    happiness += self.get_happiness_of_seatmates(*from, *to);
                    from = to;
                },
                None => {
                    return happiness
                        + self.get_happiness_of_seatmates(*from, *first);
                },
            }
        }
    }

    fn get_happiness_of_seatmates(&self, a: usize, b: usize) -> i32 {
        match self.matrix {
            // happiness changes in both ways
            Some(ref matrix) => matrix[a][b] + matrix[b][a],
            None => 0,
        }
    }

    fn get_persons_name(&self, id: usize) -> String {
        match self.nodes {
            Some(ref persons) => persons[id].clone(),
            None => "N/A".to_string(),
        }
    }
}

fn main(){
    println!("Advent of Code - day 13 | part 2");

    // import data
    let data = import_data();
    let family = match parse_data(data){
        Some(x) => x,
        None => panic!("Couldn\'t parse data!"),
    };

    let table = Vec::new();
    let (happiness, table) = family.calculate_happiness(table);

    println!("Highest happiness possible: {}", happiness);
    for id in table {
        println!("{}", family.get_persons_name(id));
    }
}

fn parse_data(data: String) -> Option<Graph> {
    let mut all_names = HashSet::new();
    let mut connections =  Vec::<(String, String, i32)>::new();

    let me = "Fabian".to_string();

    all_names.insert(me.clone());

    for line in data.lines(){
        //print!("{} -> ", line);
        let (to, from, cost) = parse_line(line.to_string());
        //println!("{} to {} = {}", to, from, cost);

        all_names.insert(to.clone());
        connections.push( (to, from, cost) );
    }

    let mut graph = Graph::new(all_names.clone());

    for name in all_names {
        if name == me {
            continue;
        }

        graph.insert_edge(&me, &name, 0);
        graph.insert_edge(&name, &me, 0);
    }

    for (to, from, cost) in connections {
        graph.insert_edge(&to, &from, cost);
    }


    Some(graph)
}

fn parse_line(line: String) -> (String, String, i32) {
    let mut values = line.split(" would ")
                .flat_map(|s| s.split(" happiness units by sitting next to "))
                .map(|s| s.parse::<String>().unwrap())
                .collect::<Vec<String>>();

    let cost = if values[1].starts_with("lose") {
        -1
    } else {
        1
    } * &values[1][5..].parse::<i32>().unwrap();

    // remove the . at the end
    values[2].pop();

    (values[0].clone(), values[2].clone(), cost )
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/13.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

    // remove trailing \n
    data.pop();
	data
}
