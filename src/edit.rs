use sdl2::keyboard::Keycode;

#[derive(Debug, Clone)]
pub struct Editor {
    pub mode: Mode,
    pub buf: Buffer,
}

impl Editor {

    pub fn new(filename: &str) -> std::io::Result<Self> {
        Ok(Self {
            mode: Mode::Normal,
            buf:  Buffer::from_file(filename)?,
        })
    }

    pub fn handle_keypress(&mut self, key: Keycode) {
        match self.mode {
            Mode::Normal => self.handle_keypress_normal(key),
            Mode::Insert => self.handle_keypress_insert(key),
        }
    }

    fn handle_keypress_insert(&mut self, key: Keycode) {

        match key {

            Keycode::Escape => {
                self.mode = Mode::Normal;
                self.buf.move_left();
            }

            _ => {
                self.buf.insert(key.into_i32() as u8 as char);
                self.buf.move_right();
            }

        }

    }

    fn handle_keypress_normal(&mut self, key: Keycode) {

        match key {
            Keycode::J => self.buf.move_down(),
            Keycode::K => self.buf.move_up(),
            Keycode::H => self.buf.move_left(),
            Keycode::L => self.buf.move_right(),
            Keycode::X => self.buf.delete(),
            Keycode::D => self.buf.delete_line(),
            Keycode::I => self.mode = Mode::Insert,
            _ => {}
        }

    }

}


#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}






#[derive(Debug, Clone)]
pub struct Buffer {
    pub cursor_char: usize,
    pub cursor_line: usize,
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            cursor_char: 0,
            cursor_line: 0,
            lines: Vec::new(),
        }
    }
}

impl Buffer {

    pub fn from_file(filename: &str) -> std::io::Result<Self> {
        let lines = std::fs::read_to_string(filename)?
            .lines()
            .map(|elem| elem.to_string())
            .collect();
        Ok(Self {
            lines,
            ..Default::default()
        })
    }

    pub fn insert(&mut self, c: char) {
        self.lines[self.cursor_line].insert(self.cursor_char, c);
    }

    pub fn delete_line(&mut self) {
        self.lines.remove(self.cursor_line);
    }

    pub fn delete(&mut self) {
        self.lines[self.cursor_line].remove(self.cursor_char);
    }

    pub fn move_down(&mut self) {
        self.cursor_line += 1;
    }

    pub fn move_up(&mut self) {
        self.cursor_line -= 1;
    }

    pub fn move_right(&mut self) {
        self.cursor_char += 1;
    }

    pub fn move_left(&mut self) {
        self.cursor_char -= 1;
    }

}
