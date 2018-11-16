extern crate aocutils;
extern crate petgraph;

use std::collections::VecDeque;
use petgraph::prelude::*;
use petgraph::dot::Dot;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub xx: usize,
    pub yy: usize,
}

impl Position {
    pub fn new(xx: usize, yy: usize) -> Position {
        Position {
            xx,
            yy,
        }
    }

    pub fn move_up(&mut self) {
        self.xx -= 1;
    }

    pub fn move_down(&mut self) {
        self.xx += 1;
    }

    pub fn move_left(&mut self) {
        self.yy -= 1;
    }

    pub fn move_right(&mut self) {
        self.yy += 1;
    }

    pub fn get_pos_up(&self) -> Position {
        Position {
            xx: self.xx - 1,
            yy: self.yy,
        }
    }

    pub fn get_pos_down(&self) -> Position {
        Position {
            xx: self.xx + 1,
            yy: self.yy,
        }
    }

    pub fn get_pos_left(&self) -> Position {
        Position {
            xx: self.xx,
            yy: self.yy - 1,
        }
    }

    pub fn get_pos_right(&self) -> Position {
        Position {
            xx: self.xx,
            yy: self.yy + 1,
        }
    }
}

fn main() {
    let day: i32 = 24;

    let input = aocutils::import(day, Some("puzzle1"));

    let maze: Vec<Vec<char>> = input.into_iter()
        .map(|line| {
            line.chars().
                into_iter()
                .map(|cc| if cc == '.' { ' ' } else { cc } )
                .collect()
        })
        .collect();

    //for row in &maze {
    //    println!("{:?}", row);
    //}

    let mut positions = [None; 10];
    for (xx, row) in maze.iter().enumerate() {
        for (yy, cc) in row.iter().enumerate() {
            match cc {
                '0' => positions[0] = Some(Position::new(xx, yy)),
                '1' => positions[1] = Some(Position::new(xx, yy)),
                '2' => positions[2] = Some(Position::new(xx, yy)),
                '3' => positions[3] = Some(Position::new(xx, yy)),
                '4' => positions[4] = Some(Position::new(xx, yy)),
                '5' => positions[5] = Some(Position::new(xx, yy)),
                '6' => positions[6] = Some(Position::new(xx, yy)),
                '7' => positions[7] = Some(Position::new(xx, yy)),
                '8' => positions[8] = Some(Position::new(xx, yy)),
                '9' => positions[9] = Some(Position::new(xx, yy)),
                _ => {}
            }
        }
    }

    //println!("{:#?}", positions);

    let mut maze_graph = Graph::new_undirected();
    let mut graph_nodes = Vec::new();
    for cc in 0..=9 {
        graph_nodes.push(maze_graph.add_node(cc));
    }

    for (idx_start, pos_start) in positions.into_iter().enumerate() {
        if let Some(pos_start) = pos_start {
            let neighbors = find_neighbors(&maze, pos_start);
            //println!("Found neighbors for {:?}: {:?}", pos_start, neighbors);
            for (idx_end, weight) in neighbors.into_iter().enumerate() {
                if idx_end < idx_start {
                    continue;
                }

                if let Some(weight) = weight {
                    maze_graph.add_edge(graph_nodes[idx_start], graph_nodes[idx_end], weight.to_owned());
                }

            }
        }
    }


    //println!("{:?}", maze_graph);
    //for edge in maze_graph.edge_references() {
    //    println!("{:?}", edge);
    //}

    println!("{:?}", Dot::new(&maze_graph));
    println!("\nUse this output to create a graph via `dot -Tpng graph.txt > graph.png` and solve graphically.")
}

fn find_neighbors(maze: &Vec<Vec<char>>, start: &Position) -> [Option<usize>; 10] {
    let mut neighbors = [None; 10];

    let mut search_queue = VecDeque::new();
    search_queue.push_back((start.get_pos_up(), 1));
    search_queue.push_back((start.get_pos_down(), 1));
    search_queue.push_back((start.get_pos_left(), 1));
    search_queue.push_back((start.get_pos_right(), 1));

    let mut queued = Vec::new();
    queued.push(start.to_owned());
    while let Some((pos, distance)) = search_queue.pop_front() {
        match maze[pos.xx][pos.yy] {
            '#' => {},
            ' ' => {
                [
                    pos.get_pos_up(),
                    pos.get_pos_down(),
                    pos.get_pos_left(),
                    pos.get_pos_right(),
                ].into_iter()
                    .for_each(|&next_pos| {
                        if maze[next_pos.xx][next_pos.yy] != '#'
                        && !queued.contains(&next_pos) {
                            search_queue.push_back((next_pos, distance + 1));
                            queued.push(next_pos);
                        }
                    });
            },
            value @ '0'...'9' => {
                let idx: usize = value.to_digit(10).unwrap() as usize;
                if neighbors[idx].is_none() {
                    neighbors[idx] = Some(distance);

                    [
                        pos.get_pos_up(),
                        pos.get_pos_down(),
                        pos.get_pos_left(),
                        pos.get_pos_right(),
                    ].into_iter()
                    .for_each(|&next_pos| {
                        if maze[next_pos.xx][next_pos.yy] != '#'
                        && !queued.contains(&next_pos) {
                            search_queue.push_back((next_pos, distance + 1));
                            queued.push(next_pos);
                        }
                    });
                }
            },
            _ => {}
        }
    }

    neighbors
}
