use super::*;

use regex::Regex;
use std::collections::HashMap;

use part1::*;

static INFINITY: usize = 9999999;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

fn tool_options(ff: Field) -> [Tool; 2] {
    match ff.ctype {
        Ctype::Rocky => [Tool::ClimbingGear, Tool::Torch],
        Ctype::Wet => [Tool::ClimbingGear, Tool::Neither],
        Ctype::Narrow => [Tool::Torch, Tool::Neither],
    }
}

fn switch_necessary(aa: Field, bb: Field) -> bool {
    let tt = tool_options(aa);
    let uu = tool_options(bb);

    ! tt.into_iter().any(|tool| uu.contains(&tool))
}

pub fn get_field(cave: &mut Cave, xx: usize, yy: usize) -> Field {
    if let Some(field) = cave.fields.get( &(xx, yy) ) {
        return field.to_owned();
    }

    // else
    // calculate field
    let geologic_index = if ( xx == 0 && yy == 0 ) || ( xx == cave.target.0 && yy == cave.target.1) {
        0
    } else if yy == 0 {
        xx * 16807
    } else if xx == 0 {
        yy * 48271
    } else {
        get_field(cave, xx - 1, yy).erosion_level * get_field(cave, xx, yy - 1).erosion_level
    };

    let erosion_level = (geologic_index + cave.depth) % 20183;
    let field = Field {
        erosion_level: erosion_level,
        ctype: match erosion_level % 3 {
            0 => Ctype::Rocky,
            1 => Ctype::Wet,
            2 => Ctype::Narrow,
            _ => panic!(" modulo 3 > 2 oO"),
        }
    };

    cave.fields.insert((xx, yy), field);
    field
}

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut cave = Cave {
        depth: input.depth,
        target: (input.target.xx_as_usize(), input.target.yy_as_usize() ),
        fields: HashMap::new(),
    };

    // dijkstra attempt
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    struct Node {
        // shortest distances to this node for different tools
        // [torch, climbing-gear, neither]
        distances: [usize; 3],
        visited: bool,
    };

    impl Node {
        fn new() -> Node {
            Node {
                distances: [INFINITY; 3],
                visited: false,
            }
        }
    }

    // actually, i should split this up into `visited` and `unvisited`, I guess..
    let mut nodes = HashMap::new();
    nodes.insert( (0,0), Node {
        distances: [0, 7, 7], // also add switching costs
        visited: false,
    });

    let mut solution_candidate = 0;
    loop {
        let current: Option<((usize, usize), Node)> = nodes.iter()
        .filter(|(_, node)| !node.visited)
        .min_by(|aa, bb| {
            usize::cmp(aa.1.distances.iter().min().unwrap(), bb.1.distances.iter().min().unwrap())
        })
        // satisfy borrow checker
        .map(|(aa, bb)| (aa.to_owned(), bb.to_owned()));

        if let Some(current) = current {
            let xx = (current.0).0;
            let yy = (current.0).1;

            if xx == cave.target.0 && yy == cave.target.1 {
                println!("{:?} (dont forget to add +7 if necessary)", nodes.get( &(xx, yy) ));

                let distances: Vec<usize> = nodes.get( &(xx, yy) )
                    .unwrap()
                    .distances
                    .iter()
                    .enumerate()
                    // add switching costs for non-torch items
                    .map(|(ii, dist)| dist + if ii == 0 { 0 } else { 7 })
                    .filter(|&dist| dist < INFINITY)
                    .collect();


                if distances.len() == 2 {
                    return distances.iter().min().unwrap().to_owned();
                } else if distances.len() == 1 {
                    solution_candidate = distances[0].to_owned();
                    println!("Found potential solution: {}. Continuing search though.", solution_candidate);
                    let entry = nodes.entry( (xx, yy) ).or_insert( Node::new());
                    entry.visited = true;
                    continue;
                }

            } else if xx >= cave.target.0 + 120 || yy >= cave.target.1 + 120 {
                // don't go too far away
                let entry = nodes.entry( (xx, yy) ).or_insert( Node::new());
                entry.visited = true;
                continue;
            }

            let mut neighbors = Vec::new();
            // up
            if xx > 0 {
                neighbors.push((xx - 1, yy));
            }
            // left
            if yy > 0 {
                neighbors.push((xx, yy - 1));
            }
            // right
            neighbors.push((xx, yy + 1));
            // down
            neighbors.push((xx + 1, yy));

            for pos in neighbors {
                let next = nodes.entry( pos ).or_insert( Node::new());
                //if !next.visited {
                    for (ii, tool) in [Tool::Torch, Tool::ClimbingGear, Tool::Neither].into_iter().enumerate() {
                        if tool_options({get_field(&mut cave, xx, yy)}).contains(&tool)
                        && tool_options({get_field(&mut cave, pos.0, pos.1)}).contains(&tool) {
                            //let cost_0 = current.distances[0] + 1 + if ii == 0 { 0 } else { 7 };
                            let best_cost = [
                                // current cost foor tool x
                                //                     + 1 (walking distance to new field)
                                //                         + switching cost (if necessary)
                                current.1.distances[0] + 1 + if ii == 0 { 0 } else { 7 },
                                current.1.distances[1] + 1 + if ii == 1 { 0 } else { 7 },
                                current.1.distances[2] + 1 + if ii == 2 { 0 } else { 7 },
                            ].into_iter().min().unwrap().to_owned();

                            //next.distances[ii] = usize::min(next.distances[ii], best_cost);
                            if best_cost < next.distances[ii] {
                                next.distances[ii] = best_cost;
                                // mark the node as unvisited again, so that it is re-evaluated
                                // in case the new cost is better than the old one (might be caused
                                // due to the switching cost)
                                next.visited = false;
                            }
                        }
                    }
                //}
            }

            // mark visited
            let entry = nodes.entry( (xx, yy) ).or_insert( Node::new());
            entry.visited = true;
        } else {
            break;
        }
    }

    solution_candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), OutputType::default());
    }
}
