pub fn solve(input: &Vec<String>) -> i32 {

    let mut sum = 0;

    for line in input {
        let mut numbers: Vec<i32> = line.split('\t').map(|x| x.parse::<i32>().unwrap()).collect();
        numbers.sort();

        if let Some(min) = numbers.first() {
            if let Some(max) = numbers.last() {
                sum += max - min;
            }
        }
    }

    sum
}
