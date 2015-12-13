// adventofcode - day 12
// part 2

use std::io::prelude::*;
use std::fs::File;

enum Element {
    Root(i32),
    Array(i32),
    Object(i32, bool),
    Str(String),
}

impl Element {
    fn add_char(&mut self, ch: char) -> bool {
        match *self {
            // add a char only if it's really a string
            Element::Str(ref mut string) => {
                string.push(ch);
                true
            },
            _ => false,
        }
    }

    fn add_value(&mut self, value: i32) {
        match *self {
            // add values only to Root, Array, or Object
            Element::Root(ref mut val)
            | Element::Array(ref mut val)
            | Element::Object(ref mut val, _) => {
                *val += value;
            },
            _ => {},
        }
    }

    fn set_red(&mut self) {
        match *self {
            // only Objects care for the "red"-attribute
            Element::Object(_, ref mut red) => {
                *red = true;
            },
            _ => {},
        }
    }

    fn get_value(&self) -> i32 {
        match *self {
            // Root and Array can return their value without any problems
            Element::Root(ref val)
            | Element::Array(ref val) => {
                *val
            }
            // Objects containing at least one "red" must not count towards the
            // sum
            Element::Object(ref val, ref red) => {
                if *red {
                    0
                } else {
                    *val
                }
            },
            _ => 0,
        }
    }

    fn is_red_string(&self) -> bool {
        match *self {
            // check whether a string is "red"
            Element::Str(ref string) => {
                string == "red"
            },
            _ => false,
        }
    }
}

fn main(){
    println!("Advent of Code - day 12 | part 2");

    // import data
    let data = import_data();

    // create the element stack and populate it with our root element
    let mut record = Vec::<Element>::new();
    record.push( Element::Root(0) );

    // stores the last character which was parsed
    let mut last: char = '\x00';
    // stores the multiplier of the current number. either 1 or -1
    let mut multiplier = 1i32;
    // stores the current value, in case of multi-digit values
    let mut tmp_val = 0i32;

    // loop through ALL the CHARS
    for ch in data.chars(){
        match ch {
            // add new elements accordingly
            '[' => {
                record.push( Element::Array(0) );
            },
            '{' => {
                record.push( Element::Object(0, false) );
            },
            // wenn an element (array or object) is closed, we need to remove
            // it from the stack and move it's value over to the next outer
            // element
            ']' | '}' => {
                match record.pop() {
                    Some(mut element) => {
                        match record.last_mut() {
                            Some(outer) => {
                                // first, in case we also finished reading a
                                // numer, we need to add it
                                if last.is_digit(10) {
                                    let value = tmp_val * multiplier;
                                    tmp_val = 0;
                                    multiplier = 1;

                                    element.add_value(value);
                                }
                                outer.add_value( element.get_value() );
                            },
                            None => panic!("Got a problem!"),
                        }
                    },
                    None => panic!("This should never happen!"),
                }
            },
            // beginning or end of string
            '"' => {
                // check whether this opened or closed a string
                match record.pop() {
                    Some(element) => {
                        match element {
                            // it was a string
                            e @ Element::Str(_) => {
                                // get reference to outer element
                                if e.is_red_string() {
                                    match record.last_mut(){
                                        Some(element) => element.set_red(),
                                        None => panic!("Ooops!"),
                                    }
                                }
                            },
                            // it was anything but a string (= this is the
                            // beginning of a new one) -> add back the element
                            // and also add a new string element
                            _ => {
                                record.push(element);
                                record.push( Element::Str(String::new()));
                            },
                        };
                    },
                    None => panic!("Santa, we got a problem!"),
                };
            },
            // digit, keep track of current number (in case it's multi-digit)
            '0'...'9' => {
                if last == '-' {
                    multiplier = -1;
                }
                tmp_val = tmp_val * 10 + match ch.to_string().parse::<i32>() {
                    Ok(x) => x,
                    Err(e) => panic!("Help! {}", e),
                };
            },
            // we just finished encountering a number -> save it
            _ if last.is_digit(10) => {
                let value = tmp_val * multiplier;
                tmp_val = 0;
                multiplier = 1;

                match record.last_mut() {
                    Some(element) => element.add_value(value),
                    None => panic!("Uhh Ohh :("),
                };
            }
            // everything else
            _ => {
                // if we're currently in a string, we need to add the character
                match record.last_mut() {
                    Some(element) => element.add_char(ch),
                    None => panic!("All aboard the panic train!"),
                };
            },
        }

        last = ch;
    }

    match record.pop() {
        Some(root) => {
            match root {
                Element::Root(_) => println!("Sum: {}", root.get_value()),
                _ => println!("Thats no Root-Element! oO :("),
            };
        },
        None => println!("No Root Element found! oO :("),
    };

}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/12.txt") {
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
