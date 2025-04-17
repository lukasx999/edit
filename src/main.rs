#![allow(dead_code, unused_imports)]

mod edit;
use edit::{Buffer, Editor};

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








fn render_buf(buf: &Buffer, renderer: &mut Renderer, font: &TtfFont) -> RendererResult<()> {

    for (i, line) in buf.lines.iter().enumerate() {

        // cursorline
        if buf.cursor_line as usize == i {

            renderer.render_rect(
                0,
                (i * font.height as usize) as i32,
                WIDTH,
                font.height as u32,
                Color::GRAY
            )?;

            // width of all chars leading up to cursor
            let widthsum = font.font.size_of(&line[..buf.cursor_char as usize])?.0;

            // width of current char
            let charwidth = font.font.size_of_char(buf.current_char())?.0;

            // we have to do it like that, to support non-monospace fonts

            // char cursor
            renderer.render_rect(
                widthsum as i32,
                (buf.cursor_line * font.height as isize) as i32,
                charwidth,
                font.height as u32,
                Color::BLUE
            )?;

        }


        // text
        if !line.is_empty() { // SDL cant render zero width text
            renderer.render_text(
                0,
                (i * font.height as usize) as i32,
                line,
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
        render_buf(&ed.buf, &mut rd, &font)?;
        rd.canvas.present();

    }


    Ok(())
}
