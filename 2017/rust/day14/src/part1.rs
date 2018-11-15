pub fn solve(hashes: &Vec<String>) -> i32 {
    let mut used_total = 0;

    for hash in hashes {
        let mut used = 0;
        for ch in hash.chars() {
            used += match ch {
                '0' => 0,
                '1' | '2' | '4' | '8' => 1,
                '3' | '5' | '6' | '9' | 'a' | 'c' => 2,
                '7' | 'b' | 'd' | 'e' => 3,
                'f' => 4,
                _ => 0
            };
        }

        used_total += used;
    }

    used_total
}
