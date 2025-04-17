use std::io::Write;
use std::fs;

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
            buf: Buffer::from_file(filename)?,
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
                "$"       => self.buf.move_end_line(),
                "g"       => self.buf.move_top(),
                "G"       => self.buf.move_bot(),
                "i"       => self.mode = Mode::Insert,
                "w"       => self.buf.save_to_own_file().unwrap(), // TODO: handle error

                "I" => {
                    self.buf.move_start_line();
                    self.mode = Mode::Insert;
                }

                "a" => {
                    self.buf.move_right();
                    self.mode = Mode::Insert;
                }

                // TODO: fix
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
    pub filename: Option<String>,
    // cursor must be signed for out-of-bounds checking
    pub cursor_char: isize,
    pub cursor_line: isize,
    pub lines: Vec<String>,
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
            .truncate(true)
            .write(true)
            .open(filename)?
            .write_all(buf.as_bytes())?;

        Ok(())
    }

    fn check_cursor(&mut self) {

        let line_len = self.lines[self.cursor_line as usize].len() as isize;

        if self.cursor_char >= line_len {
            self.cursor_char = line_len - 1;
        }

        if self.cursor_char < 0 {
            self.cursor_char = 0;
        }

    }

    pub fn current_char(&self) -> char {
        let line = &self.lines[self.cursor_line as usize];
        line
            .chars()
            .nth(self.cursor_char as usize)
            .unwrap()
    }

    pub fn newline(&mut self) {
        self.lines.insert(self.cursor_line as usize, String::new());
        self.check_cursor();
    }

    pub fn insert(&mut self, s: &str) {
        self.lines[self.cursor_line as usize]
            .insert_str(self.cursor_char as usize, s);
    }

    pub fn delete_line(&mut self) {
        self.lines.remove(self.cursor_line as usize);
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
