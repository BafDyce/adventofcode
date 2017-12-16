use super::*;

pub fn solve(moves: Vec<Move>, dancers: String) -> String {
    let mut performance = Performance::new(dancers, moves);
    performance.dance();
    performance.get_position()
}
