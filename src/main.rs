mod edit;
use edit::{Editor, Mode};

mod sdlwrap;
use sdlwrap::{SDLResult, TtfFont, render_text, render_rect, render_text_bounded};

use sdl2::video::Window;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::WindowCanvas};


type DynError = Box<dyn std::error::Error>;

const MONOSPACE: bool = false;
const FONTPATH: &str = if MONOSPACE {
    "src/fonts/jetbrainsmono.ttf"
} else {
    "src/fonts/roboto.ttf"
};
const FONTSIZE: u16 = 50;
const FILEPATH: &str = "src/file.txt";
const CURSOR_SIZE: u32 = 3;
const PADDING: u32 = 25;
const COLOR_BG:         Color = Color::RGB(40, 43, 46);
const COLOR_CURSORLINE: Color = Color::RGB(60, 64, 69);
const COLOR_CURSOR:     Color = Color::RGB(186, 194, 204);
const COLOR_TEXT:       Color = Color::WHITE;
const COLOR_STATUSLINE: Color = Color::RGB(158, 189, 219);



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

    pub fn calculate(&mut self, width: u32, height: u32, status_height: u32) {

        self.buffer = Rect::new(
            PADDING as i32,
            PADDING as i32,
            width - PADDING * 2,
            height - status_height - PADDING *  3,
        );

        self.statusbar = Rect::new(
            PADDING as i32,
            (height - status_height - PADDING) as i32,
            width - PADDING * 2,
            status_height,
        );

    }

}

struct Application {
    cv:     WindowCanvas,
    ed:     Editor,
    layout: Layout,
}

impl Application {

    pub fn new(cv: WindowCanvas, filepath: &str) -> SDLResult<Self> {
        Ok(Self {
            layout: Layout::default(),
            ed: Editor::new(filepath)?,
            cv,
        })
    }

    fn render_statusline(&mut self, font: &TtfFont) -> SDLResult<()> {

        let Self { cv, layout, .. } = self;
        let bounds = layout.statusbar;

        cv.set_draw_color(Color::GRAY);
        cv.draw_rect(bounds)?;

        render_text_bounded(
            bounds.x,
            bounds.y,
            self.ed.statusbar.as_str(),
            cv,
            COLOR_STATUSLINE,
            &font.font,
            bounds.w
        )?;

        Ok(())
    }

    fn render_buf(&mut self, font: &TtfFont) -> SDLResult<()> {

        let Self { ed, cv, layout } = self;
        let buf = &ed.buf;
        let bounds = layout.buffer;

        cv.set_draw_color(Color::GRAY);
        cv.draw_rect(bounds)?;

        // the amount of lines that can fit onto the screen
        let linecount = (bounds.h as usize / font.height as usize)
            .clamp(0, buf.lines.len());

        for (i, line) in buf.lines[..linecount].iter().enumerate() {

            // cursorline
            if buf.cursor_line as usize == i {

                render_rect(
                    bounds.x,
                    bounds.y + (i * font.height as usize) as i32,
                    bounds.w as u32,
                    font.height as u32,
                    COLOR_CURSORLINE,
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
                    COLOR_CURSOR,
                    cv
                )?;

            }

            // text
            if !line.is_empty() { // SDL cannot render zero width text
                render_text_bounded(
                    bounds.x,
                    bounds.y + (i * font.height as usize) as i32,
                    line,
                    cv,
                    COLOR_TEXT,
                    &font.font,
                    bounds.w
                )?;
            }

        }

        Ok(())

    }

    pub fn handle_event(&mut self, ev: &Event) {
        self.ed.handle_keypress(ev);
    }

    pub fn render(&mut self, font: &TtfFont) -> SDLResult<()> {

        self.ed.update_statusbar();

        self.cv.set_draw_color(COLOR_BG);
        self.cv.clear();

        let (width, height) = self.cv
            .window()
            .size();
        self.layout.calculate(width, height, font.height as u32);

        self.render_statusline(&font)?;
        self.render_buf(&font)?;

        self.cv.present();

        Ok(())
    }

}




fn main() -> Result<(), Box<dyn std::error::Error>> {

    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;

    let video = sdl.video()?;
    video.text_input();

    let mut window = video
        .window("edit", 1600, 900)
        .resizable()
        .position_centered()
        .build()?;

    window.set_minimum_size(500, 500)?;

    let cv = window
        .into_canvas()
        .build()?;

    let mut app = Application::new(cv, FILEPATH)?;

    let ttf = sdl2::ttf::init()?;
    let font = TtfFont::new(&ttf, FONTPATH, FONTSIZE)?;


    'running: loop {
        for event in event_pump.poll_iter() {

            app.handle_event(&event);

            if let Event::Quit { .. } = event {
                break 'running;
            }

            if let Event::TextInput { text, .. } = event {
                if text == "q" {
                    break 'running;
                }
            }


        }

        app.render(&font)?;

    }


    Ok(())
}
