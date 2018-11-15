pub fn solve(input: usize) -> usize {
    let mut buffer = Vec::with_capacity(2018);
    buffer.push(0);

    let mut idx = 0usize;
    for ii in 1..2018 {
        idx = (idx + input + 1) % ii;
        buffer.insert(idx, ii);
    }

    buffer[ (idx+1) % 2018 ]
}
