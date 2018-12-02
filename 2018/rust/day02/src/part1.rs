use super::*;

pub type OutputType = usize;

pub fn solve(input: &InputType) -> OutputType {
    let (twos, threes) = input.iter().fold((0, 0), |(twos, threes), line| {
        let mut two_found = 0;
        let mut three_found = 0;
        let id: Vec<char> = line.chars().collect();

        for letter in String::from("abcdefghijklmnopqrstuvwxyz").chars() {
            let letters: Vec<&char> = id.iter().filter_map(|xx| {
                if *xx == letter {
                    Some(xx)
                } else {
                    None
                }
            })
            .collect();

            if letters.len() == 3 {
                three_found = 1;
            } else if letters.len() == 2 {
                two_found = 1;
            }
        }

        (twos + two_found, threes + three_found)
    });

    twos * threes
}
