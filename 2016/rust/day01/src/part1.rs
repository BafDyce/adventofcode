use super::*;

pub fn solve(input: String) -> i32 {
    let mut santa = Person {x:0, y:0, f: Facing::UP};

    for direction in input.split(", ") {
        if let Some(dir) = direction.chars().nth(0) {
            santa.turn(dir);
            if let Ok(steps) = direction.split_at(1).1.parse::<i32>() {
                santa.walk(steps);
            }
        }
    }

    santa.x.abs() + santa.y.abs()
}
