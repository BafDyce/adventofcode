use super::*;

pub fn solve(grid: Vec<Vec<char>>) -> String {

    let mut start = None;
    for (nn, cc) in grid[0].iter().enumerate() {
        if *cc != ' ' {
            start = Some(nn.to_owned());
            break;
        }
    }

    if let Some(start) = start {
        let mut packet = Packet {
            grid: grid,
            xx: 0,
            yy: start,
            dir: Direction::Down,
            letters: Vec::new(),
            steps: 1,
        };

        loop {
            if packet.step() {
                break;
            }
        }

        packet.letters.into_iter().collect()
    } else {
        panic!("Invalid input!")
    }
}
