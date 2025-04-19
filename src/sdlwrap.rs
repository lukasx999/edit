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
pub enum SDLError {
    #[error("font-related error")]
    Font(#[from] sdl2::ttf::FontError),
    #[error("error as a string")]
    String(String),
    #[error("texture value error")]
    TextureValue(#[from] sdl2::render::TextureValueError)
}

impl From<String> for SDLError {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

pub type SDLResult<T> = Result<T, SDLError>;




pub fn render_text(
    x:        i32,
    y:        i32,
    text:     impl AsRef<str>,
    cv:       &mut WindowCanvas,
    color:    Color,
    font:     &Font,
) -> SDLResult<()> {

    let surface = font
        .render(text.as_ref())
        .solid(color)?;

    let tc = cv.texture_creator();
    let texture = tc.create_texture_from_surface(&surface)?;

    let rect = Rect::new(x, y, surface.width(), surface.height());
    cv.copy(&texture, None, Some(rect))?;

    Ok(())
}

pub fn render_rect(x: i32, y: i32, w: u32, h: u32, color: Color, cv: &mut WindowCanvas) -> SDLResult<()> {
    cv.set_draw_color(color);
    cv.draw_rect(Rect::new(x, y, w, h))?;
    Ok(())
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
