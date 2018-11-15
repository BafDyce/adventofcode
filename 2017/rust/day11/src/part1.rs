use super::*;

pub fn solve(input: &String) -> i32 {
    let mut pos = Pos {row: 0, col: 0};
    for direction in input.split(",") {
        pos.step(direction);
    }

    pos.distance( Pos {row: 0, col: 0})
}
