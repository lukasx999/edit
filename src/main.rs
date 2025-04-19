#![allow(dead_code, unused_imports)] // TODO: remove me

mod edit;
use edit::{Buffer, Editor, Mode};

mod renderer;
use renderer::{Renderer, RendererResult, RendererError, TtfFont};

use thiserror::Error;

use sdl2::{
    event::Event,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    ttf::{Font, Sdl2TtfContext},
    video::WindowContext
};


type DynError = Box<dyn std::error::Error>;

const FONTPATH: &str = "src/fonts/roboto.ttf";
// const FONTPATH: &str = "src/fonts/jetbrainsmono.ttf";
const FONTSIZE: u16 = 46;
const FILEPATH: &str = "src/file.txt";
const CURSOR_SIZE: u32 = 3;




#[derive(Debug, Clone, Copy)]
struct Layout {
    statusbar: Rect,
    buffer: Rect,
}

impl Default for Layout {
    fn default() -> Self {
        let empty = Rect::new(0, 0, 0, 0);
        Self {
            statusbar: empty,
            buffer: empty,
        }
    }
}

impl Layout {

    pub fn calculate(&mut self, width: u32, height: u32) {

        let status_height = 75;
        let pad = 50;

        self.statusbar = Rect::new(
            pad as i32,
            pad as i32,
            width - pad * 2,
            status_height
        );

        self.buffer = Rect::new(
            pad as i32,
            (status_height + pad * 2) as i32,
            width - pad * 2,
            height - status_height - pad * 3
        );

    }

}




struct Application {
    ed: Editor,
    rd: Renderer,
}

impl Application {

    pub fn new(filename: &str) -> Result<Self, DynError> {
        Ok(Self {
            ed: Editor::new(filename)?,
            rd: Renderer::new()?,
        })
    }

    pub fn render_statusbar(&mut self, font: &TtfFont, bounds: Rect) -> RendererResult<()> {
        let Self { ed, rd } = self;

        rd.render_text(
            bounds.x,
            bounds.y,
            ed.mode.to_string(),
            Color::RED,
            &font.font
        )?;

        Ok(())
    }

    pub fn render_buf(&mut self, font: &TtfFont, bounds: Rect) -> RendererResult<()> {

        let Self { ed, rd } = self;
        let buf = &ed.buf;

        // the amount of lines that can fit onto the screen
        let linecount = (bounds.h / font.height as i32) as usize;

        for (i, line) in buf.lines[..linecount].iter().enumerate() {

            // cursorline
            if buf.cursor_line as usize == i {

                rd.render_rect(
                    bounds.x,
                    bounds.y + (i * font.height as usize) as i32,
                    bounds.w as u32,
                    font.height as u32,
                    Color::GRAY
                )?;


                // width of all chars leading up to cursor
                let widthsum = font.font.size_of(&line[..buf.cursor_char as usize])?.0;

                // width of current char
                let cursor = match buf.current_char() {
                    Some(c) if ed.mode == Mode::Normal => font.font.size_of_char(c)?.0,
                    _    => CURSOR_SIZE,
                };

                // char cursor
                rd.render_rect(
                    bounds.x + widthsum as i32,
                    bounds.y + (buf.cursor_line * font.height as isize) as i32,
                    cursor,
                    font.height as u32,
                    Color::BLUE
                )?;

            }

            // get slice of line that is small enough to render
            let mut line_slice = line.as_str();
            while font.font.size_of(&line_slice)?.0 as i32 > bounds.w {
                line_slice = &line_slice[..line_slice.len()-1];
            }

            // text
            if !line.is_empty() { // SDL cant render zero width text
                rd.render_text(
                    bounds.x,
                    bounds.y + (i * font.height as usize) as i32,
                    &line_slice,
                    Color::WHITE,
                    &font.font
                )?;
            }

        }

        Ok(())

    }

}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut app = Application::new(FILEPATH)?;
    let ttf = sdl2::ttf::init()?;
    let font = TtfFont::new(&ttf, FONTPATH, FONTSIZE)?;
    let mut layout = Layout::default();

    app.rd.video.text_input();

    'running: loop {
        if let Some(event) = app.rd.event_pump.poll_event() {

            app.ed.handle_keypress(&event);

            if let Event::Quit { .. } = event {
                break 'running;
            }

            if let Event::TextInput { text, .. } = event {
                if text == "q" {
                    break 'running;
                }
            }

        }
        app.rd.clear(Color::BLACK);

        let (width, height) = app.rd.canvas.window().size();
        layout.calculate(width, height);

        app.rd.canvas.set_draw_color(Color::GRAY);
        app.rd.canvas.draw_rect(layout.statusbar)?;
        app.render_statusbar(&font, layout.statusbar)?;

        app.rd.canvas.set_draw_color(Color::GRAY);
        app.rd.canvas.draw_rect(layout.buffer)?;
        app.render_buf(&font, layout.buffer)?;


        app.rd.canvas.present();

    }


    Ok(())
}
