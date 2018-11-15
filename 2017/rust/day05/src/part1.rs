pub fn solve(input: &Vec<i32>) -> i32 {
    let mut input = input.clone();
    let length = input.len() - 1;

    let mut ii = 0i32;
    let mut steps = 0;
    loop {
        let dist = input[ii as usize];
        input[ii as usize] += 1;

        ii += dist;
        if ii > length as i32 {
            break steps + 1;
        }

        steps += 1;
    }
}
