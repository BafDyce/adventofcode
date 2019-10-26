struct KeyPad1 {
    pad: [[char; 3]; 3],
    xx: usize,
    yy: usize,
}

impl KeyPad1 {
    pub fn new() -> KeyPad1 {
        KeyPad1 {
            pad: [
                ['1', '2', '3'],
                ['4', '5', '6'],
                ['7', '8', '9'],
            ],
            xx: 1,
            yy: 1,
        }
    }

    pub fn move_pos(&mut self, dir: char) {
        match dir {
            'U' if self.xx > 0 => self.xx -= 1,
            'D' if self.xx < self.pad.len() - 1 => self.xx += 1,
            'L' if self.yy > 0 => self.yy -= 1,
            'R' if self.yy < self.pad.len() - 1 => self.yy += 1,
            _ => {}
        }
    }

    pub fn get_pos(&self) -> char {
        self.pad[self.xx][self.yy]
    }
}

pub fn solve(input: &[String]) -> String {
    let mut result = String::new();
    let mut keypad = KeyPad1::new();

    for line in input {
        for dir in line.chars() {
            keypad.move_pos(dir);
        }

        result.push(keypad.get_pos());
    }

    result
}