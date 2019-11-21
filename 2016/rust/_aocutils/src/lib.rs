use std::fs::OpenOptions;
use std::io::{self, BufRead};

pub fn import(day: i32, name: Option<&str>) -> Vec<String> {
    let name = match name {
        Some(name) => name,
        None => "puzzle1"
    };

    let fname = format!("../../_inputs/day{:02}/{}.input", day, name);
    let file = match OpenOptions::new().read(true).write(false).create(false).open(&fname) {
        Ok(file) => file,
        Err(err) => panic!("Error reading file \"{}\": {:?}", fname, err)
    };

    match io::BufReader::new(file).lines().collect() {
        Ok(lines) => lines,
        Err(err) => panic!("Error reading file \"{}\": {:?}", fname, err)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
