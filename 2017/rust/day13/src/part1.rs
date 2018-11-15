pub fn solve(firewalls: &Vec<(i32, i32)>) -> i32 {
    let mut severity = 0;

    for &(pos, size) in firewalls {
        let period = (size - 1) * 2;
        let loc = (pos) % period;
        if loc == 0 {
            severity += pos * size;
        }
    }

    severity
}
