pub fn solve(input: &Vec<String>) -> i32 {

    let mut res = 0;

    for phrase in input {
        let mut words: Vec<&str> = phrase.split(" ").collect();
        words.sort();

        // count invalid passphrases (is easier!)
        for ii in 0..(words.len()-1) {
            if words[ii] == words[ii + 1] {
                res += 1;
                break;
            }
        }
    }

    input.len() as i32 - res
}
