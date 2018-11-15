extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

use std::ascii::AsciiExt;


fn main() {
    let day: i32 = 14;

    let input = aocutils::import(day, Some("puzzle1")).remove(0);

    let mut hashes: Vec<String> = Vec::new();
    for ii in 0..128 {
        let data = format!("{}-{}", input, ii);
        let hash = knothash(&data);

        hashes.push(hash);
    }

    let res1 = part1::solve(&hashes);
    let res2 = part2::solve(&hashes);

    println!("Result for {}: {} and {}", input, res1, res2);
}

pub fn knothash(input: &String) -> String {
    let mut numbers: Vec<i32> = (0..256i32).collect();

    let numberlen = numbers.len();
    let mut lengths: Vec<usize> = Vec::new();
    for ch in input.chars() {
        assert!(ch.is_ascii());
        lengths.push( (ch as u8) as usize);
    }
    lengths.extend([17, 31, 73, 47, 23].iter());

    let mut current = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for length in &lengths {
            let mut selection: Vec<i32> = numbers
                                            .iter()
                                            .cycle()
                                            .skip(current)
                                            .take(*length)
                                            .map(|x| *x)
                                            .collect();
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
            //assert_eq!(numbers.len(), numberlen);

            current += (length + skip_size) % numberlen;
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
