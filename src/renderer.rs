use crate::{WIDTH, HEIGHT};

use thiserror::Error;

use sdl2::{
    event::Event,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    ttf::{Font, Sdl2TtfContext},
    video::WindowContext
};



#[derive(Error, Debug)]
pub enum RendererError {
    #[error("font-related error")]
    Font(#[from] sdl2::ttf::FontError),
    #[error("error as a string")]
    String(String),
    #[error("texture value error")]
    TextureValue(#[from] sdl2::render::TextureValueError)
}

impl From<String> for RendererError {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

pub type RendererResult<T> = Result<T, RendererError>;



pub struct Renderer {
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
        x:        i32,
        y:        i32,
        text:     impl AsRef<str>,
        color:    Color,
        font:     &Font,
    ) -> RendererResult<()> {

        let surface = font
            .render(text.as_ref())
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

pub struct TtfFont<'a> {
    pub height: u16,
    pub font: Font<'a, 'a>,
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
