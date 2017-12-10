pub fn solve(input: &Vec<usize>) -> i32 {
    let mut numbers: Vec<i32> = (0..256i32).collect();
    // E X A M P L E
    //let mut numbers: Vec<i32> = (0..5i32).collect();

    let numberlen = numbers.len();

    //println!("numbers: {:?}", numbers);
    //println!("lengths: {:?}", input);

    let mut current = 0;
    let mut skip_size = 0;
    for length in input {
        //println!("=> current = {} | skip_size = {} | length = {}", current, skip_size, length);
        let mut selection: Vec<i32> = numbers
                                        .iter()
                                        .cycle()
                                        .skip(current)
                                        .take(*length)
                                        .map(|x| *x)
                                        .collect();
        //println!("==> selection: {:?}", selection);
        selection.reverse();

        let curlen = current + length;
        let selection_iter = selection.iter();
        numbers = if curlen >= numbers.len() {
            // selection wrapped
            let not_selected = numbers
                                .iter()
                                .cycle()
                                .skip(curlen)
                                .take( numbers.len() - length );

            let tmp: Vec<i32> = not_selected
                                .chain(selection_iter)
                                .map(|x| *x)
                                .collect();
            tmp.iter()
                .cycle()
                .skip( numbers.len() - (curlen % numbers.len()))
                .take(numbers.len())
                .map(|x| *x)
                .collect()
        } else {
            // selection did not wrap
            let start = numbers.iter().take(current);
            let end = numbers.iter().skip(curlen);
            start.chain(selection_iter).chain(end).map(|x| *x).collect()
        };

        //println!("==> numbers after modifictation: {:?}", numbers);
        assert_eq!(numbers.len(), numberlen);

        current += length + skip_size;
        skip_size += 1;
    }

    numbers[0] * numbers[1]
}
