pub fn solve(input: &(u64, u64)) -> u64 {
    let mut vals = *input;
    let mut cnt = 0;
    for _ in 0..40_000_000 {
        vals = gen_next(vals);

        if vals.0 & 0xffff == vals.1 & 0xffff {
            cnt += 1;
        }
    }

    cnt
}

fn gen_next(vals: (u64, u64)) -> (u64, u64) {
    ( (vals.0 * 16807) % 2147483647, (vals.1 * 48271) % 2147483647, )
}
