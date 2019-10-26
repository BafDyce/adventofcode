struct KeyPad2 {
    pad: [[char; 5]; 5],
    xx: usize,
    yy: usize,
}

impl KeyPad2 {
    pub fn new() -> KeyPad2 {
        KeyPad2 {
            pad: [
                [' ', ' ', '1', ' ', ' '],
                [' ', '2', '3', '4', ' '],
                ['5', '6', '7', '8', '9'],
                [' ', 'A', 'B', 'C', ' '],
                [' ', ' ', 'D', ' ', ' '],
            ],
            xx: 2,
            yy: 0,
        }
    }

    pub fn move_pos(&mut self, dir: char) {
        match dir {
            'U' if self.xx > 0 && self.pad[self.xx-1][self.yy] != ' ' => self.xx -= 1,
            'D' if self.xx < self.pad.len() - 1 && self.pad[self.xx+1][self.yy] != ' ' => self.xx += 1,
            'L' if self.yy > 0 && self.pad[self.xx][self.yy-1] != ' ' => self.yy -= 1,
            'R' if self.yy < self.pad.len() - 1 && self.pad[self.xx][self.yy+1] != ' ' => self.yy += 1,
            _ => {}
        }
    }

    pub fn get_pos(&self) -> char {
        self.pad[self.xx][self.yy]
    }
}

pub fn solve(input: &[String]) -> String {
    let mut result = String::new();
    let mut keypad = KeyPad2::new();

    for line in input {
        for dir in line.chars() {
            keypad.move_pos(dir);
        }

        result.push(keypad.get_pos());
    }

    result
}