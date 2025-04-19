#![allow(dead_code, unused_imports)] // TODO: remove me

mod edit;
use edit::{Buffer, Editor, Mode};

mod sdlwrap;
use sdlwrap::{SDLResult, SDLError, TtfFont, render_text, render_rect};

use thiserror::Error;

use sdl2::{
    event::Event, pixels::Color, rect::Rect, render::WindowCanvas, sys::Window, ttf::{Font, Sdl2TtfContext}, video::WindowContext
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







fn render_statusbar(ed: &Editor, cv: &mut WindowCanvas, font: &TtfFont, bounds: Rect) -> SDLResult<()> {

    render_text(
        bounds.x,
        bounds.y,
        ed.mode.to_string(),
        cv,
        Color::RED,
        &font.font
    )?;

    Ok(())
}

pub fn render_buf(
    ed:     &Editor,
    cv:     &mut WindowCanvas,
    font:   &TtfFont,
    bounds: Rect
) -> SDLResult<()> {

    let buf = &ed.buf;

    // the amount of lines that can fit onto the screen
    let linecount = (bounds.h / font.height as i32) as usize;

    for (i, line) in buf.lines[..linecount].iter().enumerate() {

        // cursorline
        if buf.cursor_line as usize == i {

            render_rect(
                bounds.x,
                bounds.y + (i * font.height as usize) as i32,
                bounds.w as u32,
                font.height as u32,
                Color::GRAY,
                cv
            )?;


            // width of all chars leading up to cursor
            let widthsum = font.font.size_of(&line[..buf.cursor_char as usize])?.0;

            // width of current char
            let cursor = match buf.current_char() {
                Some(c) if ed.mode == Mode::Normal
                => font.font.size_of_char(c)?.0,
                _ => CURSOR_SIZE,
            };

            // char cursor
            render_rect(
                bounds.x + widthsum as i32,
                bounds.y + (buf.cursor_line * font.height as isize) as i32,
                cursor,
                font.height as u32,
                Color::BLUE,
                cv
            )?;

        }

        // get slice of line that is small enough to render
        let mut line_slice = line.as_str();
        while font.font.size_of(&line_slice)?.0 as i32 > bounds.w {
            line_slice = &line_slice[..line_slice.len()-1];
        }

        // text
        if !line.is_empty() { // SDL cant render zero width text
            render_text(
                bounds.x,
                bounds.y + (i * font.height as usize) as i32,
                &line_slice,
                cv,
                Color::WHITE,
                &font.font
            )?;
        }

    }

    Ok(())

}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
        .window("edit", 1600, 900)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump()?;

    let ttf = sdl2::ttf::init()?;
    let font = TtfFont::new(&ttf, FONTPATH, FONTSIZE)?;

    let mut layout = Layout::default();

    let mut ed = Editor::new(FILEPATH)?;

    video.text_input();

    'running: loop {
        if let Some(event) = event_pump.poll_event() {

            ed.handle_keypress(&event);

            if let Event::Quit { .. } = event {
                break 'running;
            }

            if let Event::TextInput { text, .. } = event {
                if text == "q" {
                    break 'running;
                }
            }

        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let (width, height) = canvas.window().size();
        layout.calculate(width, height);

        canvas.set_draw_color(Color::GRAY);
        canvas.draw_rect(layout.statusbar)?;
        render_statusbar(&ed, &mut canvas, &font, layout.statusbar)?;

        canvas.set_draw_color(Color::GRAY);
        canvas.draw_rect(layout.buffer)?;
        render_buf(&ed, &mut canvas, &font, layout.buffer)?;


        canvas.present();

    }


    Ok(())
}
