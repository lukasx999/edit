#![allow(dead_code, unused_imports)]

mod edit;
use edit::{Buffer, Mode, Editor};

use thiserror::Error;

use sdl2::{
    pixels::Color,
    render::{Canvas, WindowCanvas},
    rect::Rect,
    video::WindowContext,
    ttf::Font,
    keyboard::Keycode,
    event::Event
};


const WIDTH:  u32 = 1600;
const HEIGHT: u32 = 900;


struct Renderer {
    pub sdl:             sdl2::Sdl,
    pub video:           sdl2::VideoSubsystem,
    pub canvas:          WindowCanvas,
    pub texture_creator: sdl2::render::TextureCreator<WindowContext>,
    pub ttf:             sdl2::ttf::Sdl2TtfContext,
    pub event_pump:      sdl2::EventPump,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {

        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let window = video
            .window("edit", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .unwrap();

        Ok(Self {
            ttf:             sdl2::ttf::init().unwrap(),
            texture_creator: canvas.texture_creator(),
            event_pump:      sdl.event_pump()?,
            video,
            canvas,
            sdl,
        })

    }

    pub fn render_text(
        &mut self,
        text:     &str,
        x:        i32,
        y:        i32,
        fontsize: u16,
        fontname: &str,
        color:    Color,
    ) -> SDLResult<()> {

        let font = self.ttf.load_font(fontname, fontsize)?;

        let surface = font
            .render(text)
            .solid(color)?;

        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let rect = Rect::new(x, y, surface.width(), surface.height());
        self.canvas.copy(&texture, None, Some(rect))
            .unwrap();

        Ok(())
    }

    pub fn render_rect(
        &mut self,
        x:      i32,
        y:      i32,
        width:  u32,
        height: u32,
        color:  Color
    ) -> SDLResult<()> {

        self.canvas.set_draw_color(color);
        let rect = Rect::new(x, y, width, height);
        self.canvas.fill_rect(rect)?;

        Ok(())
    }

}

#[derive(Error, Debug)]
enum SDLError {
    #[error("font-related error")]
    FontError(#[from] sdl2::ttf::FontError),
    #[error("error as a string")]
    StringError(String),
}

impl From<String> for SDLError {
    fn from(value: String) -> Self {
        Self::StringError(value)
    }
}

type SDLResult<T> = Result<T, SDLError>;



fn render_buf(buf: &Buffer, renderer: &mut Renderer) -> SDLResult<()> {

    let font = "src/jetbrainsmono/JetBrainsMono-Regular.ttf";
    let fontheight: usize = 64;

    // TODO: non-monospace fonts
    let fontwidth = {
        let f = renderer.ttf.load_font(font, fontheight as u16)?;
        f.size_of_char('X')?.0
    };

    for (i, line) in buf.lines.iter().enumerate() {

        if buf.cursor_line == i {
            renderer.render_rect(
                0,
                (i*fontheight) as i32,
                WIDTH,
                fontheight as u32,
                Color::GRAY
            )?;
        }

        renderer.render_rect(
            (buf.cursor_char * fontwidth as usize) as i32,
            (buf.cursor_line * fontheight) as i32,
            fontwidth as u32,
            fontheight as u32,
            Color::BLUE
        )?;

        if !line.is_empty() {
            renderer.render_text(
                line,
                0,
                (i*fontheight) as i32,
                fontheight as u16,
                font,
                Color::WHITE
            )?;
        }


    }

    Ok(())

}




fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut ed = Editor::new("src/file.txt")?;
    let mut renderer = Renderer::new()?;

    'running: loop {
        if let Some(event) = renderer.event_pump.poll_event() {

            match event {

                Event::Quit { .. } => break 'running,

                Event::KeyDown { keycode: Some(key), .. } =>
                ed.handle_keypress(key),

                _ => {}

            }
        }

        renderer.canvas.set_draw_color(Color::BLACK);
        renderer.canvas.clear();

        render_buf(&ed.buf, &mut renderer)?;

        renderer.canvas.present();

    }


    Ok(())
}
