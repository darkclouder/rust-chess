use termion::event::Key;


const MAX_LINE_LENGTH: usize = 16;


pub struct Prompt {
    line: [char; MAX_LINE_LENGTH],
    position: usize,
}


impl Prompt {
    pub fn default() -> Self {
        Self {
            line: ['\0'; MAX_LINE_LENGTH],
            position: 0,
        }
    }

    pub fn consume_key(&mut self, key: &Key) {
        match key {
            Key::Char(c) => self.add_char(c),
            Key::Backspace => self.remove_last_char(),
            _ => (),
        }
    }

    pub fn get_line(& self) -> String {
        self.line[0..self.position].iter().collect()
    }

    pub fn clear(&mut self) {
        self.position = 0;
    }

    fn add_char(&mut self, character: &char) {
        if self.position < MAX_LINE_LENGTH {
            self.line[self.position] = *character;
            self.position += 1;
        }
    }

    fn remove_last_char(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }
}
