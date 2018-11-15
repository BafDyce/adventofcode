pub fn solve(input: usize) -> usize {
    let mut idx = 0usize;

    let mut idx0 = 0;
    let mut val_after_0 = 1;
    let mut val_at_idx_0 = 0;

    for ii in 1..50_000_001 {
        idx = (idx + input + 1) % ii;

        if idx == idx0 {
            val_after_0 = ii;
        }

        if idx == 0 {
            val_at_idx_0 = ii;
        }

        if idx <= idx0 {
            idx0 += 1;
        }
    }

    if idx0 == 50_000_000 {
        val_at_idx_0
    } else {
        val_after_0
    }
}
