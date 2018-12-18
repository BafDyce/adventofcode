use super::*;
use std::collections::HashMap;

pub type OutputType = isize;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> (OutputType, usize) {
    let mut field = input.clone();
    //print_field(&field);

    let mut next_id = 1;
    let mut unit_lookup = HashMap::new();
    for row in field.iter_mut() {
        for cell in row.iter_mut() {
            if let Field::Unit(ref mut unit) = cell {
                unit_lookup.insert(next_id, unit.utype.to_owned());
                unit.id = next_id;
                next_id += 1;
            }
        }
    }

    let mut round_counter = 0;
    loop {
        /*
        println!("BEGIN TURN {}:", round_counter + 1);
        print_field(&field);

        for (xx, row) in field.iter().enumerate() {
            for (yy, cell) in row.iter().enumerate() {
                if let Field::Unit(unit) = cell {
                    println!("Unit @ {}/{}: {:?}", xx, yy, unit);
                }
            }
        }
        */

        let mut units = Vec::new();
        let mut elf_counter = 0;
        let mut goblin_counter = 0;
        for (xx, row) in field.iter().enumerate() {
            for (yy, cell) in row.iter().enumerate() {
                if let Field::Unit(unit) = cell {
                    units.push( (Location2D::new(xx as isize, yy as isize), unit.id) );
                    match unit.utype {
                        UnitType::Elf => elf_counter += 1,
                        UnitType::Goblin => goblin_counter += 1,
                    }
                }
            }
        }

        let number_of_units = units.len();
        let mut processed_units = Vec::new();
        let mut died_units = Vec::new();
        for (unit, uid) in units {
            if died_units.contains(&uid) {
                // Skip this unit if it already died
                //println!("Skipping unit with id {}", uid);
                continue;
            }

            match process_cell(&mut field, unit, round_counter) {
                None => {},
                Some(id) => {
                    match unit_lookup.get(&id) {
                        None => println!("ERROR: Received invalid unit id"),
                        Some(UnitType::Elf)     => elf_counter -= 1,
                        Some(UnitType::Goblin)  => goblin_counter -= 1,
                    }
                    died_units.push(id);
                }
            }
            processed_units.push(uid);

            if elf_counter == 0 || goblin_counter == 0 {
                break;
            }
        }

        if elf_counter == 0 || goblin_counter == 0 {
            //println!("number_of_units: {}", number_of_units);
            //println!("processed_units: {:?}", processed_units);
            //println!("died_units: {:?}", died_units);

            let mut unit_check: Vec<_> = processed_units.iter().chain(died_units.iter()).collect();
            unit_check.sort();
            unit_check.dedup();
            if unit_check.len() == number_of_units {
                round_counter += 1;
            }

            let mut remaining_hp = 0;
            for row in field.iter() {
                for cell in row.iter() {
                    if let Field::Unit(unit) = cell {
                        remaining_hp += unit.hp;
                    }
                }
            }
            //println!("Battle ended after {} rounds with {} HP left", round_counter, remaining_hp);
            //println!("Survived elves: {} | Goblins survived: {}", elf_counter, goblin_counter);
            return (round_counter * remaining_hp, elf_counter);
        }

        round_counter += 1;
        //println!("AFTER {} TURNS:", round_counter);
        //print_field(&field);
        /*
        for (xx, row) in field.iter().enumerate() {
            for (yy, cell) in row.iter().enumerate() {
                if let Field::Unit(unit) = cell {
                    println!("Unit @ {}/{}: {:?}", xx, yy, unit);
                }
            }
        }*/

        //enter_to_continue();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config).0
    }

    #[test]
    fn example0_2() {
        assert_eq!(solve_example("example0_2"), 27730);
    }

    #[test]
    fn example1() {
        assert_eq!(solve_example("example1"), 36334);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_example("example2"), 39514);
    }

    #[test]
    fn example3() {
        assert_eq!(solve_example("example3"), 27755);
    }

    #[test]
    fn example4() {
        assert_eq!(solve_example("example4"), 28944);
    }

    #[test]
    fn example5() {
        assert_eq!(solve_example("example5"), 18740);
    }
}
