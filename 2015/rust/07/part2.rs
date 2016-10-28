// adventofcode - day 7
// part 2

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::cell::RefCell;

enum Gate {
    // Strings are the names of the input wire(s)
    Not(String),
    And(String, String),
    FixedAnd(String),       // for ANDs with a '1'
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    Redirect(String),
    Const(u16),
}

impl Gate {
    // long list of functions which parse the input and create the right gates
    fn new(desc: &mut &str) -> Gate {
        if desc.contains("NOT") {
            Gate::new_not(desc)
        } else if desc.contains("AND") {
            if desc.starts_with("1"){
                Gate::new_fixed_and(desc)
            } else {
                Gate::new_and(desc)
            }
        } else if desc.contains("OR") {
            Gate::new_or(desc)
        } else if desc.contains("RSHIFT") {
            Gate::new_rshift(desc)
        } else if desc.contains("LSHIFT") {
            Gate::new_lshift(desc)
        } else {
            Gate::new_redirect_or_const(desc)
        }
    }

    fn new_not(desc: &mut &str) -> Gate {
        // skip the "NOT "
        let tmp = &desc[4..];

        let x = tmp.split(" -> ").collect::<Vec<&str>>();

        *desc = x[1];
        Gate::Not(x[0].to_string())
    }

    fn new_fixed_and(desc: &mut &str) -> Gate {
        // skip the "1 AND "
        let tmp = &desc[6..];

        let x = tmp.split(" -> ").collect::<Vec<&str>>();

        *desc = x[1];
        Gate::FixedAnd(x[0].to_string())
    }

    fn new_and(desc: &mut &str) -> Gate {
        let x = desc.split(" AND ")
                    .flat_map(|s| s.split(" -> "))
                    .collect::<Vec<&str>>();

        *desc = x[2];
        Gate::And(x[0].to_string(), x[1].to_string())
    }

    fn new_or(desc: &mut &str) -> Gate {
        let x = desc.split(" OR ")
                    .flat_map(|s| s.split(" -> "))
                    .collect::<Vec<&str>>();

        *desc = x[2];
        Gate::Or(x[0].to_string(), x[1].to_string())
    }

    fn new_rshift(desc: &mut &str) -> Gate {
        let x = desc.split(" RSHIFT ")
                    .flat_map(|s| s.split(" -> "))
                    .collect::<Vec<&str>>();

        *desc = x[2];
        Gate::Rshift(x[0].to_string(), x[1].parse::<u16>().unwrap())
    }

    fn new_lshift(desc: &mut &str) -> Gate {
        let x = desc.split(" LSHIFT ")
                    .flat_map(|s| s.split(" -> "))
                    .collect::<Vec<&str>>();

        *desc = x[2];
        Gate::Lshift(x[0].to_string(), x[1].parse::<u16>().unwrap())
    }

    fn new_redirect_or_const(desc: &mut &str) -> Gate {
        let x = desc.split(" -> ").collect::<Vec<&str>>();

        *desc = x[1];
        match x[0].parse::<u16>() {
            Ok(x) => Gate::Const(x),
            Err(_) => Gate::Redirect(x[0].to_string())
        }
    }

    // compute the value of this gate, depending on its type
    fn compute_value(&mut self, wires: &HashMap<String, RefCell<Wire>>) -> u16 {
        match *self {
            Gate::Not(ref wname) => {
                let x = wname.to_string();
                //println!("Trying to borrow {}", x);
                let mut wire = match wires.get(&x) {
                    Some(elem) => elem.borrow_mut(),
                    None => panic!("No wire with name \"{}\" found!", wname),
                };

                ! wire.get_value(&wires)
            },
            Gate::And(ref wn1, ref wn2) => {
                let tmp;

                // we need to create an extra block here, so that w1 has a
                // shorter lifetime and dies right after the block again.
                // as a result, it is free again and available for a new
                // recursive borrow
                {
                    let x = wn1.to_string();
                    //println!("Trying to borrow {}", x);
                    let mut w1 = match wires.get(&x) {
                        Some(elem) => elem.borrow_mut(),
                        None => panic!("No wire with name \"{}\" found!", wn1),
                    };

                    tmp = w1.get_value(&wires);
                }

                let x = wn2.to_string();
                //println!("Trying to borrow {}", x);
                let mut w2 = match wires.get(&x) {
                    Some(elem) => elem.borrow_mut(),
                    None => panic!("No wire with name \"{}\" found!", wn2),
                };

                tmp & w2.get_value(&wires)
            },
            Gate::FixedAnd(ref wname) => {
                let x = wname.to_string();
                //println!("Trying to borrow {}", x);
                let mut wire = match wires.get(&x) {
                    Some(elem) => elem.borrow_mut(),
                    None => panic!("No wire with name \"{}\" found!", wname),
                };

                wire.get_value(&wires) & 1
            },
            Gate::Or(ref wn1, ref wn2) => {
                let tmp;

                {
                    let x = wn1.to_string();
                    //println!("Trying to borrow {}", x);
                    let mut w1 = match wires.get(&x) {
                        Some(elem) => elem.borrow_mut(),
                        None => panic!("No wire with name \"{}\" found!", wn1),
                    };

                    tmp = w1.get_value(&wires);
                }

                let x = wn2.to_string();
                //println!("Trying to borrow {}", x);
                let mut w2 = match wires.get(&x) {
                    Some(elem) => elem.borrow_mut(),
                    None => panic!("No wire with name \"{}\" found!", wn2),
                };

                tmp | w2.get_value(&wires)
            },
            Gate::Lshift(ref wname, bits) => {
                let x = wname.to_string();
                //println!("Trying to borrow {}", x);
                let mut wire = match wires.get(&x) {
                    Some(elem) => elem.borrow_mut(),
                    None => panic!("No wire with name \"{}\" found!", wname),
                };

                wire.get_value(&wires) << bits
            },
            Gate::Rshift(ref wname, bits) => {
                let x = wname.to_string();
                //println!("Trying to borrow {}", x);
                let mut wire = match wires.get(&x) {
                    Some(elem) => elem.borrow_mut(),
                    None => panic!("No wire with name \"{}\" found!", wname),
                };

                wire.get_value(&wires) >> bits
            },
            Gate::Redirect(ref wname) => {
                let x = wname.to_string();
                //println!("Trying to borrow {}", x);
                let mut wire = match wires.get(&x) {
                    Some(elem) => elem.borrow_mut(),
                    None => panic!("No wire with name \"{}\" found!", wname),
                };

                wire.get_value(&wires)
            },
            Gate::Const(value) => {
                value
            },
        }
    }
}

// each Wire stores:
// - its name (which is also its key in the hashmap)
// - the gate this wire is connected to
// - value of this gate (if it's already computed)
struct Wire {
    #[allow(dead_code)]
    id: String,
    gate: Gate,
    value: Option<u16>,
}

impl Wire {
    fn new(desc: &mut &str) -> Wire {
        Wire{ gate: Gate::new( desc), value: None, id: desc.to_string()}
    }

    fn get_value(&mut self, list: &HashMap<String, RefCell<Wire>>) -> u16 {
        match self.value {
            Some(val) => val,
            None => {
                let val = self.gate.compute_value(list);
                self.value = Some(val);
                val
            }
        }
    }
}

fn main(){
    println!("Advent of Code - day 7 | part 2");

    // import data
    let data = import_data();
    let wires = create_hashmap_from_data(&data);

    let mut a = match wires.get("a") {
        Some(elem) => elem.borrow_mut(),
        None => panic!("No wire with name \"a\" found!"),
    };

    let value = a.get_value(&wires);
    println!("Value of a: {}. Now setting b to that value.", value);

    // part 2 specific code
    let mut wires2 = create_hashmap_from_data(&data);

    // extra block, so that tmp dies again
    {
        wires2.remove("b");
        let tmp = RefCell::new( Wire{id: "b".to_string(),
                                    gate: Gate::Const(value),
                                    value: Some(value) } );
        wires2.insert("b".to_string(), tmp);
    }

    let mut a = match wires2.get("a") {
        Some(elem) => elem.borrow_mut(),
        None => panic!("No wire with name \"a\" found!"),
    };
    println!("Value of a: {}", a.get_value(&wires2));
}

fn create_hashmap_from_data(data: &String) -> HashMap<String, RefCell<Wire>> {
    let mut wires = HashMap::new();

    for mut line in data.lines(){
        let wire = Wire::new(&mut line);

        let x = RefCell::new(wire);

        // line now contains the id of the wire -> use it as key for the hashmap
        wires.insert(line.to_string(), x);
    }

    wires
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/07.txt") {
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
