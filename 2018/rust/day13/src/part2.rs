use super::*;

type OutputType = super::part1::OutputType;
//type OutputType = i64; // <-- IF part 2 needs a different output

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut tracks: InfiniteGrid<Field> = InfiniteGrid::new();

    for (ii, line) in input.into_iter().enumerate() {
        let xx = ii as isize;
        for (jj, cc) in line.into_iter().enumerate() {
            let yy = jj as isize;
            //println!("[{}][{}] = {}", xx, yy, cc);
            //let yy = ii as isize;
            //let xx = jj as isize;
            match cc {
                cc @ '|'
                | cc @ '+'
                | cc @ '-'
                | cc @ '/'
                | cc @ '\\' => {
                    tracks.set_value(&Location2D::new(xx, yy), Field::NoCart(*cc));
                },
                '>' => {
                    let cart = Cart::new_with_particle(Particle2D::new(xx, yy, Direction2D::Right));
                    tracks.set_value(&Location2D::new(xx, yy), Field::Cart(cart, '-'));
                }
                '<' => {
                    let cart = Cart::new_with_particle(Particle2D::new(xx, yy, Direction2D::Left));
                    tracks.set_value(&Location2D::new(xx, yy), Field::Cart(cart, '-'));
                }
                '^' => {
                    let cart = Cart::new_with_particle(Particle2D::new(xx, yy, Direction2D::Up));
                    tracks.set_value(&Location2D::new(xx, yy), Field::Cart(cart, '|'));
                }
                'v' => {
                    let cart = Cart::new_with_particle(Particle2D::new(xx, yy, Direction2D::Down));
                    tracks.set_value(&Location2D::new(xx, yy), Field::Cart(cart, '|'));
                }
                _ => {}
            }
        }
    }

    //println!("==== START PART 2 =====");
    //fancy_print(&tracks);
    //enter_to_continue();

    loop {
        let mut positions = Vec::new();
        for (pos, field) in tracks.iter() {
            match field {
                Field::Cart(_cart, _) => {
                    positions.push(pos.to_owned());
                    //println!("{:?}", cart);
                }
                _ => {}
            }
        }

        if positions.len() == 1 {
            break positions[0].to_owned();
        }

        let mut carts = Vec::new();
        for pos in &positions {
            let entry = tracks.get_value_ref(pos);
            let entry_clone = entry.clone();
            if let Field::Cart(mut cart, _old) = entry_clone {
                carts.push(cart);
            }
        }
        carts.sort_unstable_by(|aa, bb| aa.get_pos().cmp(&bb.get_pos()));

        for mut cart in carts {
            let old_pos = cart.get_pos();

            let still_alive = {
                let entry = tracks.get_value_ref(&old_pos);
                match &entry {
                    Field::Cart(_, _) => true,
                    _ => false,
                }
            };

            if still_alive {
                cart.walker.step_forward();
                {
                    let entry = tracks.get_value_ref(&cart.get_pos());
                    let new = match entry {
                        Field::Cart(_other, track) => {
                            // carts crash -> remove
                            Field::NoCart(*track)
                        },
                        Field::NoCart(cc) => {
                            match cc {
                                '+' => cart.turn(),
                                '/' => match cart.get_dir() {
                                    Direction2D::Up     => cart.walker.turn_to_right(),
                                    Direction2D::Right  => cart.walker.turn_to_up(),
                                    Direction2D::Down   => cart.walker.turn_to_left(),
                                    Direction2D::Left   => cart.walker.turn_to_down(),
                                }
                                '\\' => match cart.get_dir() {
                                    Direction2D::Up     => cart.walker.turn_to_left(),
                                    Direction2D::Right  => cart.walker.turn_to_down(),
                                    Direction2D::Down   => cart.walker.turn_to_right(),
                                    Direction2D::Left   => cart.walker.turn_to_up(),
                                }
                                _ => {}
                            }
                            Field::Cart(cart, *cc)
                        }
                    };

                    *entry = new;
                }

                // now remove the old cart entry
                let entry = tracks.get_value_ref(&old_pos);
                *entry = match entry.clone() {
                    Field::Cart(_, track) => Field::NoCart(track),
                    other => other,
                }
            }
        }
    }
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
        assert_eq!(solve_example("example2"), Location2D::new(4, 6));
    }
}
