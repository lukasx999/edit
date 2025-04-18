use std::io::Write;
use std::fs;
use std::fmt::Display;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::event::Event;



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Mode::Normal => "normal",
            Mode::Insert => "insert",
        };
        write!(f, "{}", fmt)
    }
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
            buf: Buffer::from_file(filename)?,
        })
    }

    fn mode_normal(&mut self) {
        self.mode = Mode::Normal;
    }

    fn mode_insert(&mut self) {
        self.mode = Mode::Insert;
        if self.buf.is_current_line_empty() {
            self.buf.append = true;
        }
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
                self.mode_normal();
                self.buf.append = false;
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

        if let Event::TextInput { text, .. } = event {
            match text.as_str() {
                "j"       => self.buf.move_down(),
                "k"       => self.buf.move_up(),
                "h"       => self.buf.move_left(),
                "l"       => self.buf.move_right(),
                "x"       => self.buf.delete_char(),
                "d"       => self.buf.delete_line(),
                "0" | "_" => self.buf.move_start_line(),
                "$"       => self.buf.move_end_line(),
                "g"       => self.buf.move_top(),
                "G"       => self.buf.move_bot(),
                "w"       => self.buf.save_to_own_file().unwrap(), // TODO: handle error
                "i"       => self.mode_insert(),

                "I" => {
                    self.buf.move_start_line();
                    self.mode_insert();
                }

                "a" => {
                    self.buf.move_right();
                    self.mode_insert();
                }

                "A" => {
                    self.buf.append = true;
                    self.buf.move_end_line();
                    self.buf.move_right();
                    self.mode_insert();
                }

                "O" => {
                    self.buf.newline_above();
                    self.mode_insert();
                }

                "o" => {
                    self.buf.newline_below();
                    self.buf.move_down();
                    self.mode_insert();
                }

                _ => {}
            }

        }

    }

}



#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub filename: Option<String>,
    // cursor must be signed for out-of-bounds checking
    pub cursor_char: isize,
    pub cursor_line: isize,
    pub lines: Vec<String>,
    // append mode, allows the cursor to be out-of-bounds
    // by one char at the end of the line
    // used by `A`, and `a` at end of line
    pub append: bool, 
}

impl Buffer {

    pub fn from_file(filename: &str) -> std::io::Result<Self> {

        let abspath = fs::canonicalize(filename)?
            .to_str()
            .unwrap()
            .to_string();

        Ok(Self {
            lines: fs::read_to_string(&abspath)?
                .lines()
                .map(|line| line.to_string())
                .collect(),
            filename: Some(abspath),
            ..Default::default()
        })
    }

    pub fn save_to_own_file(&self) -> std::io::Result<()> {
        // TODO: error when buffer is not backed by a file
        self.save_to_file(self.filename
            .as_ref()
            .unwrap())?;

        Ok(())
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let buf = self.lines.join("\n");

        fs::File::options()
            .write(true)
            .truncate(true)
            .open(filename)?
            .write_all(buf.as_bytes())?;

        Ok(())
    }

    fn check_cursor(&mut self) {

        if self.cursor_line < 0 {
            self.cursor_line = 0;
        }

        let line_count = self.lines.len() as isize;

        if self.cursor_line >= line_count {
            self.cursor_line = line_count - 1;
        }

        if !self.append {
            let line_len = self.lines[self.cursor_line as usize].len() as isize;

            if self.cursor_char >= line_len {
                self.cursor_char = line_len - 1;
            }
        }

        if self.cursor_char < 0 {
            self.cursor_char = 0;
        }

    }

    pub fn is_current_line_empty(&self) -> bool {
        self.lines[self.cursor_line as usize].is_empty()
    }

    // returns None if cursor is out-of-bounds because of append mode
    pub fn current_char(&self) -> Option<char> {
        self.lines[self.cursor_line as usize]
            .chars()
            .nth(self.cursor_char as usize)
    }

    pub fn newline_above(&mut self) {
        self.lines.insert(self.cursor_line as usize, String::new());
        self.check_cursor();
    }

    pub fn newline_below(&mut self) {
        self.lines.insert(self.cursor_line as usize + 1, String::new());
        self.check_cursor();
    }

    pub fn insert(&mut self, s: &str) {
        self.lines[self.cursor_line as usize]
            .insert_str(self.cursor_char as usize, s);
    }

    pub fn delete_line(&mut self) {
        self.lines.remove(self.cursor_line as usize);
        self.check_cursor();
    }

    pub fn delete_char(&mut self) {
        self.lines[self.cursor_line as usize]
            .remove(self.cursor_char as usize);
        self.check_cursor();
    }

    pub fn move_down(&mut self) {
        self.cursor_line += 1;
        self.check_cursor();
    }

    pub fn move_up(&mut self) {
        self.cursor_line -= 1;
        self.check_cursor();
    }

    pub fn move_right(&mut self) {
        self.cursor_char += 1;
        self.check_cursor();
    }

    pub fn move_top(&mut self) {
        self.cursor_line = 0;
        self.check_cursor();
    }

    pub fn move_bot(&mut self) {
        self.cursor_line = self.lines.len() as isize - 1;
        self.check_cursor();
    }

    pub fn move_left(&mut self) {
        self.cursor_char -= 1;
        self.check_cursor();
    }

    pub fn move_start_line(&mut self) {
        self.cursor_char = 0;
    }

    pub fn move_end_line(&mut self) {
        self.cursor_char = (self.lines[self.cursor_line as usize].len() - 1) as isize;
    }

}
