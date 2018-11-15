fn get_names(line: String) -> (String, Vec<String>) {
    let mut ret_string: String = String::from("");
    let mut ret_vec: Vec<String> = Vec::new();

    let mut after = false;
    for elem in line.split_whitespace() {
        let mut name = elem.to_string();
        if name.contains(",") {
            name.pop();
        }

        if ! after {
            ret_string = name;
            after = true;
        } else {
            ret_vec.push(name);
        }
    }

    (ret_string, ret_vec)
}

pub fn solve(input: &Vec<String>) -> String {
    let mut names: Vec<String> = Vec::new();
    let mut names_above_others: Vec<String> = Vec::new();

    for line in input {
        let (first, others) = get_names(line.to_string());
        names.push(first);
        for name in others {
            names_above_others.push(name);
        }
    }

    for name in names_above_others {
        let mut index = -1;
        for ii in 0..names.len() {
            if name == names[ii] {
                index = ii as i32;
                break;
            }
        };

        if index != -1 {
            names.remove(index as usize);
        }
    }

    assert_eq!(names.len(), 1);
    names[0].clone()
}
