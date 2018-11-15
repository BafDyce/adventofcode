use std::collections::VecDeque;

pub fn solve(hashes: &Vec<String>) -> i32 {
    let mut grid: Vec<bool> = Vec::new();

    // fill grid
    for hash in hashes {
        for ch in hash.chars() {

            let bin = if ch.is_digit(10) {
                ch as u8 - '0' as u8
            } else if ch.is_digit(16) {
                ch as u8 - 'a'  as u8 + 10
            } else {
                panic!("Invalid hex char: {}", ch);
            };

            let binary = format!("{:4b}", bin);
            for ch in binary.chars() {
                let used = match ch {
                    '0' | ' ' => false,
                    '1' => true,
                    _   => panic!("Invalid binary char: {}", ch),
                };

                grid.push(used);
            }
        }
    }

    assert_eq!(grid.len(), 128*128);

    // count groups
    let mut groups = 0;
    for idx in 0..grid.len() {
        if ! grid[idx] {
            continue;
        }

        groups += 1;
        grid[idx] = false;
        let mut same_group = get_neighbor_list(idx);
        while same_group.len() > 0 {
            if let Some(nidx) = same_group.pop_front() {
                if grid[nidx] {
                    grid[nidx] = false;
                    same_group.append( &mut get_neighbor_list(nidx) );
                }
            }
        }
    }

    groups
}

fn get_neighbor_list(idx: usize) -> VecDeque<usize> {
    let mut neighbors = VecDeque::new();
    // left
    if idx % 128 > 0 {
        neighbors.push_back(idx - 1);
    }

    // right
    if idx % 128 != 127 {
        neighbors.push_back(idx + 1);
    }

    // up
    if idx > 127 {
        neighbors.push_back(idx - 128);
    }

    // down
    if idx / 128 < 127 {
        neighbors.push_back(idx + 128);
    }

    neighbors
}
