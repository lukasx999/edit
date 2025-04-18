#![allow(dead_code, unused_imports)]

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



const WIDTH:  u32 = 1600;
const HEIGHT: u32 = 900;
const FONTPATH: &str = "src/fonts/roboto.ttf";
// const FONTPATH: &str = "src/fonts/jetbrainsmono.ttf";
const FONTSIZE: u16 = 46;
const FILEPATH: &str = "src/file.txt";
const CURSOR_SIZE: u32 = 3;




fn render_statusbar(ed: &Editor, rd: &mut Renderer, font: &TtfFont, bounds: Rect) -> RendererResult<()> {

    rd.render_text(
        bounds.x,
        bounds.y,
        ed.mode.to_string(),
        Color::RED,
        &font.font
    )?;

    Ok(())
}

fn render_buf(ed: &Editor, rd: &mut Renderer, font: &TtfFont, bounds: Rect) -> RendererResult<()> {
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



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut ed = Editor::new(FILEPATH)?;

    let mut rd = Renderer::new()?;
    let ttf = sdl2::ttf::init()?;
    let font = TtfFont::new(&ttf, FONTPATH, FONTSIZE)?;

    rd.video.text_input();

    'running: loop {
        if let Some(event) = rd.event_pump.poll_event() {

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

        rd.clear(Color::BLACK);

        let rect_buf = Rect::new(100, 100, 1000, 200);
        rd.canvas.set_draw_color(Color::GRAY);
        rd.canvas.draw_rect(rect_buf)?;
        render_buf(&ed, &mut rd, &font, rect_buf)?;

        let rect_mode = Rect::new(0, 0, 200, 200);
        render_statusbar(&ed, &mut rd, &font, rect_mode)?;

        rd.canvas.present();

    }


    Ok(())
}
