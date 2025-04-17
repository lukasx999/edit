#![allow(dead_code, unused_imports)]

mod edit;
use edit::{Buffer, Editor};

use thiserror::Error;

use sdl2::{
    event::Event, pixels::Color, rect::Rect, render::WindowCanvas, sys::ttf, ttf::{Font, Sdl2TtfContext}, video::WindowContext
};


type DynError = Box<dyn std::error::Error>;


const WIDTH:  u32 = 1600;
const HEIGHT: u32 = 900;




#[derive(Error, Debug)]
enum RendererError {
    #[error("font-related error")]
    FontError(#[from] sdl2::ttf::FontError),
    #[error("error as a string")]
    StringError(String),
    #[error("texture value error")]
    TextureValueError(#[from] sdl2::render::TextureValueError)
}

impl From<String> for RendererError {
    fn from(value: String) -> Self {
        Self::StringError(value)
    }
}

type RendererResult<T> = Result<T, RendererError>;



struct Renderer {
    pub sdl:             sdl2::Sdl,
    pub video:           sdl2::VideoSubsystem,
    pub canvas:          WindowCanvas,
    pub texture_creator: sdl2::render::TextureCreator<WindowContext>,
    pub event_pump:      sdl2::EventPump,
}

impl Renderer {
    pub fn new() -> RendererResult<Self> {

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
            texture_creator: canvas.texture_creator(),
            event_pump: sdl.event_pump()?,
            video,
            canvas,
            sdl
        })

    }

    pub fn render_text(
        &mut self,
        text:     &str,
        x:        i32,
        y:        i32,
        color:    Color,
        font:     &Font,
    ) -> RendererResult<()> {

        let surface = font
            .render(text)
            .solid(color)?;

        let texture = self.texture_creator
            .create_texture_from_surface(&surface)?;

        let rect = Rect::new(x, y, surface.width(), surface.height());
        self.canvas.copy(&texture, None, Some(rect))?;

        Ok(())
    }

    pub fn render_rect(
        &mut self,
        x:      i32,
        y:      i32,
        width:  u32,
        height: u32,
        color:  Color
    ) -> RendererResult<()> {

        self.canvas.set_draw_color(color);
        let rect = Rect::new(x, y, width, height);
        self.canvas.fill_rect(rect)?;

        Ok(())
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

}

struct TtfFont<'a> {
    height: u16,
    font: Font<'a, 'a>,
}

impl<'a> TtfFont<'a> {
    pub fn new(
        ttf:    &'a Sdl2TtfContext,
        name:   &str,
        height: u16
    ) -> Result<Self, String> {
        Ok(Self {
            height,
            font: ttf.load_font(name, height)?,
        })
    }
}




fn render_buf(buf: &Buffer, renderer: &mut Renderer, font: &TtfFont) -> RendererResult<()> {

    // TODO: non-monospace fonts
    let fontwidth = font.font.size_of_char('X')?.0;

    for (i, line) in buf.lines.iter().enumerate() {

        if buf.cursor_line == i {
            renderer.render_rect(
                0,
                (i*font.height as usize) as i32,
                WIDTH,
                font.height as u32,
                Color::GRAY
            )?;
        }

        renderer.render_rect(
            (buf.cursor_char * fontwidth as usize) as i32,
            (buf.cursor_line * font.height as usize) as i32,
            fontwidth,
            font.height as u32,
            Color::BLUE
        )?;

        if !line.is_empty() {
            renderer.render_text(
                line,
                0,
                (i*font.height as usize) as i32,
                Color::WHITE,
                &font.font
            )?;
        }


    }

    Ok(())

}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut ed = Editor::new("src/file.txt")?;

    let mut rd = Renderer::new()?;
    let ttf = sdl2::ttf::init()?;
    let font = TtfFont::new(&ttf, "src/fonts/roboto.ttf", 64)?;

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
