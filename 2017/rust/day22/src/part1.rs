use super::*;

pub fn solve(input: &[String]) -> i32 {
    let mut grid: HashMap<Position, bool> = HashMap::new();

    let mut xx = 0 - (input.len() as i32 / 2);
    for line in input {
        //println!("xx = {}", xx);
        let mut yy = 0 - (line.len() as i32 / 2);
        for cc in line.chars() {
            //println!("  yy = {} --> {}", yy, cc);
            grid.insert(Position::new(xx, yy), cc == '#');
            yy += 1;
        }
        xx += 1;
    }

    let mut virus = Virus::new();
    let mut counter = 0;

    for __ in 1..=10000 {
        let infected = grid.entry(virus.get_pos()).or_insert(false);
        if *infected {
            virus.turn_right();
        } else {
            virus.turn_left();
            counter += 1;
        }
        *infected = !*infected;
        //println!("steps = {:2} | infections = {:2} | {:?}", ii, counter, virus.get_pos());
        virus.forward();
    }

    counter
}
