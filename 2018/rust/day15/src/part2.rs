use super::*;

type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut field = input.clone();

    let elves = {
        let mut elves = Vec::new();
        for (xx, row) in field.iter().enumerate() {
            for (yy, cell) in row.iter().enumerate() {
                if let Field::Unit(unit) = cell {
                    if let UnitType::Elf = unit.utype {
                        elves.push(Location2D::new(xx as isize, yy as isize))
                    }
                }
            }
        }
        elves
    };

    //println!("elves: {:?}", elves);

    for power in 4 .. {
        for pos in elves.iter() {
            if let Field::Unit(ref mut elf) = field[pos.xx_as_usize()][pos.yy_as_usize()] {
                elf.attack_power = power;
            }
        }

        let (result, elves_survived) = part1::solve(&field.clone(), config);
        //println!("{:2} -> {} ({} survied elves)", power, result, elves_survived);
        if elves_survived == elves.len() {
            //println!("Ended with a power of {}", power);
            return result;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config)
    }

    #[test]
    fn example0_2() {
        assert_eq!(solve_example("example0_2"), 4988);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_example("example2"), 31284);
    }

    #[test]
    fn example3() {
        assert_eq!(solve_example("example3"), 3478);
    }

    #[test]
    fn example4() {
        assert_eq!(solve_example("example4"), 6474);
    }

    #[test]
    fn example5() {
        assert_eq!(solve_example("example5"), 1140);
    }
}
