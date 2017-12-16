use super::*;

use std::collections::HashMap;

pub fn solve(moves: Vec<Move>, dancers: String) -> String {
    let mut performance = Performance::new(dancers, moves);
    let mut positions: HashMap<String, i32> = HashMap::new();
    let mut cnt = 0;

    positions.insert(performance.get_position(), cnt);
    let (circle_start, circle_length) = loop {
        performance.dance();
        cnt += 1;
        let position = performance.get_position();
        if let Some(pos) = positions.get(&position) {
            break (pos.clone(), cnt - pos);
        }

        positions.insert(position, cnt);
    };

    let remaining = (1_000_000_000 - circle_start) % circle_length;
    for _ in 0..remaining {
        performance.dance();
    }

    performance.get_position()
}
