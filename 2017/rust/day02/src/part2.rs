use std::cmp;

pub fn solve(input: &Vec<String>) -> i32 {

    let mut sum = 0;

    for line in input {
        let numbers: Vec<i32> = line.split('\t').map(|x| x.parse::<i32>().unwrap()).collect();
        let len = numbers.len();

        for ii in 0..(len - 1) {
            for jj in (ii + 1)..len {
                let high = cmp::max(numbers[ii], numbers[jj]);
                let low = cmp::min(numbers[ii], numbers[jj]);

                if high % low == 0 {
                    sum += high / low;
                }
            }
        }
    }

    sum
}
