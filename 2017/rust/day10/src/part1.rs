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
        numbers = if curlen >= numberlen {
            // selection wrapped
            let not_selected = numbers
                                .iter()
                                .cycle()
                                .skip(curlen)
                                .take( numberlen - length );

            not_selected.chain(selection_iter)
                        .cycle()
                        .skip( numberlen - (curlen % numberlen))
                        .take(numberlen)
                        .map(|x| *x)
                        .collect()
        } else {
            // selection did not wrap
            let start = numbers.iter().take(current);
            let end = numbers.iter().skip(curlen);
            start.chain(selection_iter).chain(end).map(|x| *x).collect()
        };

        //println!("==> numbers after modifictation: {:?}", numbers);
        //assert_eq!(numbers.len(), numberlen);

        current += (length + skip_size) % numberlen;
        skip_size += 1;
    }

    numbers[0] * numbers[1]
}
