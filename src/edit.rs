use sdl2::keyboard::{Keycode, Mod};
use sdl2::event::Event;


#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}


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

    pub fn handle_keypress(&mut self, event: &Event) {

        match self.mode {
            Mode::Normal => self.handle_keypress_normal(event),
            Mode::Insert => self.handle_keypress_insert(event),
        }
    }

    fn handle_keypress_insert(&mut self, event: &Event) {

        match event {

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.mode = Mode::Normal;
                self.buf.move_left();
            }

            Event::TextInput { text, .. } => {
                self.buf.insert(text);
                self.buf.move_right();
            }

            _ => {}
        }
    }


    fn handle_keypress_normal(&mut self, event: &Event) {

        match event {

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.mode = Mode::Normal;
                self.buf.move_left();
            }

            Event::TextInput { text, .. } =>
            match text.as_str() {
                "j"       => self.buf.move_down(),
                "k"       => self.buf.move_up(),
                "h"       => self.buf.move_left(),
                "l"       => self.buf.move_right(),
                "x"       => self.buf.delete_char(),
                "d"       => self.buf.delete_line(),
                "0" | "_" => self.buf.move_start_line(),
                "i"       => self.mode = Mode::Insert,
                "$"       => self.buf.move_end_line(),

                "I" => {
                    self.buf.move_start_line();
                    self.mode = Mode::Insert;
                }

                "a" => {
                    self.buf.move_right();
                    self.mode = Mode::Insert;
                }

                "A" => {
                    self.buf.move_end_line();
                    self.buf.move_right();
                    self.mode = Mode::Insert;
                }

                "o" => {
                    self.buf.move_down();
                    self.buf.newline();
                    self.mode = Mode::Insert;
                }

                _ => {}
            }

            _ => {}
        }

    }

}



#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub cursor_char: usize,
    pub cursor_line: usize,
    pub lines: Vec<String>,
}

impl Buffer {

    pub fn from_file(filename: &str) -> std::io::Result<Self> {

        let lines = std::fs::read_to_string(filename)?
            .lines()
            .map(|elem| elem.to_string())
            .collect();

        Ok(Self { lines, ..Default::default() })
    }

    pub fn newline(&mut self) {
        self.lines.insert(self.cursor_line, String::new());
    }

    pub fn insert(&mut self, s: &str) {
        self.lines[self.cursor_line].insert_str(self.cursor_char, s);
    }

    pub fn delete_line(&mut self) {
        self.lines.remove(self.cursor_line);
    }

    pub fn delete_char(&mut self) {
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

    pub fn move_start_line(&mut self) {
        self.cursor_char = 0;
    }

    pub fn move_end_line(&mut self) {
        self.cursor_char = self.lines[self.cursor_line].len()-1;
    }

}
