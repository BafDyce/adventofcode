pub fn solve(input: &Vec<i32>) -> i32 {
    let mut input = input.clone();
    let length = input.len() - 1;

    let mut ii = 0i32;
    let mut steps = 0;
    loop {
        let dist = input[ii as usize];
        if dist >= 3 {
            input[ii as usize] -= 1;
        } else {
            input[ii as usize] += 1;
        };

        ii += dist;

        if ii > length as i32 {
            break steps + 1;
        }

        while ii < 0 {
            ii += length as i32;
        }

        steps += 1;
    }
}
