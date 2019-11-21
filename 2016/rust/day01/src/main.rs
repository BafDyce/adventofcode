extern crate aocutils;
mod part1;
mod part2;

#[derive(Clone)]
enum Facing {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Person {
    x: i32,
    y: i32,
    f: Facing,
}

impl Person {
    fn walk(&mut self, steps: i32) {
        match self.f {
            Facing::UP => self.x += steps,
            Facing::RIGHT => self.y += steps,
            Facing::DOWN => self.x -= steps,
            Facing::LEFT => self.y -= steps,
        };
    }

    fn turn(&mut self, dir: char) {
        self.f = match dir {
            'L' => match self.f {
                Facing::UP => Facing::LEFT,
                Facing::RIGHT => Facing::UP,
                Facing::DOWN => Facing::RIGHT,
                Facing::LEFT => Facing::DOWN,
            },
            'R' => match self.f {
                Facing::UP => Facing::RIGHT,
                Facing::RIGHT => Facing::DOWN,
                Facing::DOWN => Facing::LEFT,
                Facing::LEFT => Facing::UP,
            },
            _ => self.f.clone(),
        }
    }
}

fn main() {
    let day: i32 = 1;

    let input = aocutils::import(day, Some("puzzle1")).remove(0);
    let r1 = part1::solve(input.clone());
    let r2 = part2::solve(input);

    println!("Results: {} and {}", r1, r2);
}
