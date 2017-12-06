use std::collections::HashMap;

pub fn solve(input: &Vec<i32>) -> i32 {
    let mut checker: HashMap<Vec<i32>, bool> = HashMap::new();
    let mut memory = input.clone();
    let length = memory.len();

    let mut count = 0;
    loop {
        let mut max = memory.clone();
        max.sort();
        max.reverse();
        let max = max.remove(0);

        for ii in 0..length {
            if memory[ii] == max {
                memory[ii] = 0;
                let mut memories = max;
                let mut jj = ii;
                while memories > 0 {
                    jj += 1;
                    jj %= length;
                    memory[jj] += 1;
                    memories -= 1;
                }
                break;
            }
        }
        count += 1;

        if checker.contains_key(&memory.clone()) {
            break count;
        }

        checker.insert(memory.clone(), true);
    }
}
