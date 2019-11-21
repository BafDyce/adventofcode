use super::*;
use std::collections::HashSet;

pub fn solve(input: String) -> i32 {
    let mut santa = Person {x:0, y:0, f: Facing::UP};
    let mut visited = HashSet::new();

    for direction in input.split(", ") {
        if let Some(dir) = direction.chars().nth(0) {
            santa.turn(dir);
            if let Ok(steps) = direction.split_at(1).1.parse::<i32>() {
                // we must consider each single step of santas walk
                for _ in 0 .. steps {
                    santa.walk(1);
                    let pos = (santa.x, santa.y);
                    if visited.contains(&pos) {
                    return santa.x.abs() + santa.y.abs();
                    } else {
                        visited.insert(pos);
                    }
                }
            }
        }
    }

    0
}
