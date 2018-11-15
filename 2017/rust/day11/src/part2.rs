use super::*;

pub fn solve(input: &String) -> i32 {
    let mut pos = Pos {row: 0, col: 0};
    let mut max_dist = 0;
    for direction in input.split(",") {
        pos.step(direction);
        let dist = pos.distance(Pos { row: 0, col: 0});
        if dist > max_dist {
            max_dist = dist;
        }
    }

    max_dist
}
