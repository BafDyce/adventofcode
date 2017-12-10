use std::ascii::AsciiExt;

pub fn solve(input: &String) -> String {
    let mut numbers: Vec<i32> = (0..256i32).collect();
    // E X A M P L E
    //let mut numbers: Vec<i32> = (0..5i32).collect();

    let numberlen = numbers.len();
    let mut lengths: Vec<usize> = Vec::new();
    for ch in input.chars() {
        assert!(ch.is_ascii());
        lengths.push( (ch as u8) as usize);
    }
    lengths.extend([17, 31, 73, 47, 23].iter());

    //println!("numbers: {:?}", numbers);
    //println!("lengths: {:?}", lengths);

    let mut current = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for length in lengths.clone() {
            let mut selection: Vec<i32> = numbers
                                            .iter()
                                            .cycle()
                                            .skip(current)
                                            .take(length)
                                            .map(|x| *x)
                                            .collect();
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
            assert_eq!(numbers.len(), numberlen);

            current += length + skip_size;
            skip_size += 1;
        }
    }

    let mut hash = String::new();
    for ii in 0..16 {
        let hex = numbers.iter().skip(ii*16).take(16).fold(0, |acc, &x| acc ^ x as usize);
        hash = format!("{}{:02x}", hash, hex);
    }

    hash
}
