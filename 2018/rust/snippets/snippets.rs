// Read input into 2 dimensional array of things (i.e. `char`s)
type InputType = Vec<Vec<char>>;
let data: InputType = input.into_iter()
    .map(|line| {
        line.chars().collect()
    })
    .collect()
