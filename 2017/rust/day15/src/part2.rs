pub fn solve(input: &(u64, u64)) -> u64 {
    let mut vals = *input;
    let mut cnt = 0;
    for _ in 0..5_000_000 {
        vals = gen_next(vals);

        if vals.0 & 0xffff == vals.1 & 0xffff {
            cnt += 1;
        }
    }

    cnt
}

fn gen_next(vals: (u64, u64)) -> (u64, u64) {
    let a = {
        let mut val = vals.0;
        loop {
            val = (val * 16807) % 2147483647;
            if val % 4 == 0 {
                break val;
            }
        }
    };

    let b = {
        let mut val = vals.1;
        loop {
            val = (val * 48271) % 2147483647;
            if val % 8 == 0 {
                break val;
            }
        }
    };

    (a, b)
}
