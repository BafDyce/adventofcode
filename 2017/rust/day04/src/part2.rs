pub fn solve(input: &Vec<String>) -> i32 {

    let mut res = 0;

    for phrase in input {
        let mut words: Vec<String> = phrase.split(" ")
            // for each word in a passphrase, sort its letters
            // this way anagrams turn into the same words
            .map(|w| -> String {
                let mut word: Vec<char> = w.chars().collect();
                word.sort_by(|a, b| b.cmp(a));
                word.iter().cloned().collect::<String>()
            }).collect();
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
