pub fn solve(firewalls: &Vec<(i32, i32)>) -> i32 {
    for start in 0.. {
        let mut hit = false;

        for &(pos, size) in firewalls {
            let period = (size - 1) * 2;
            let loc = (pos + start) % period;
            if loc == 0 {
                hit = true;
                break;
            }
        }

        if ! hit {
            return start;
        }
    }

    0
}
