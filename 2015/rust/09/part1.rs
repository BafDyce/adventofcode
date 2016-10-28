// adventofcode - day 9
// part 1


use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

struct Graph {
    nodes: Option<Vec<String>>,
    matrix: Option<Vec<Vec<i32>>>,
}

impl Graph {
    fn new(names: HashSet<&str>) -> Graph {
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
                matrix[dst_idx][src_idx] = length;
            },
            None => return,
        }
    }

    // bruteforce solution
    fn calculate_path(&self, visited: Vec<usize>) -> (i32, Vec<usize>){

        if visited.len() == self.size() {
            return (self.calculate_cost_of_path(&visited), visited);
        }

        let mut min_cost = std::i32::MAX;
        let mut min_path = Vec::new();
        for ii in 0..self.size(){
            if ! visited.contains(&ii){
                let mut path = visited.clone();
                path.push(ii);
                let (cost, path) = self.calculate_path(path);
                if cost < min_cost {
                    min_cost = cost;
                    min_path = path;
                }
            }
        }

        (min_cost, min_path)
    }

    fn calculate_cost_of_path(&self, path: &Vec<usize>) -> i32 {
        let mut locations = path.iter();
        let mut from = locations.next().unwrap();
        let mut cost = 0i32;
        loop {
            match locations.next() {
                Some(to) => {
                    cost += self.get_edge_cost(*from, *to);
                    from = to;
                },
                None => return cost,
            }
        }
    }

    fn get_edge_cost(&self, from: usize, to: usize) -> i32 {
        match self.matrix {
            Some(ref matrix) => matrix[from][to],
            None => 0,
        }
    }
}

fn main(){
    println!("Advent of Code - day 9 | part 1");

    // import data
    let data = import_data();
    let graph = match parse_data(data){
        Some(x) => x,
        None => panic!("Couldn\'t parse data!"),
    };

    //println!("Graph has the following nodes ({}):", graph.size());
    //for name in graph.get_node_names() {
    //    println!("{}", name);
    //}

    let path = Vec::new();
    let (cost, path) = graph.calculate_path(path);

    println!("Shortest path costs: {}", cost);
    for location in path {
        println!("{}", location);
    }
}

fn parse_data(data: String) -> Option<Graph> {
    let mut all_names = HashSet::new();

    // first: scan data for names
    for line in data.lines(){
        let names = line.split(" to ").flat_map(|s| s.split(" = ")).take(2);
        for name in names {
            all_names.insert(name);
        }
    }

    let mut graph = Graph::new(all_names);

    for line in data.lines(){
        let info = line.split(" to ")
                                    .flat_map(|s| s.split(" = "))
                                    .map(|s| s.parse::<String>().unwrap())
                                    .collect::<Vec<String>>();

        let length = info[2].parse::<i32>().unwrap();
        graph.insert_edge(&info[0], &info[1], length);
    }

    Some(graph)
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/09.txt") {
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
