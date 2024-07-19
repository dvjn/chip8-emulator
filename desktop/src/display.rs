use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct DisplayDevice {
    canvas: Canvas<Window>,
}

impl DisplayDevice {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().expect("video subsystem for sdl2");
        let window = video_subsystem
            .window("Chip-8 Emulator", 960, 480)
            .build()
            .expect("window from video subsystem");
        let canvas = window.into_canvas().build().expect("window canvas");

        Self { canvas }
    }

    pub fn render(&mut self, buffer: &[bool; 2048]) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (i, &pixel) in buffer.iter().enumerate() {
            if pixel {
                let x = (i % 64) as i32 * 15;
                let y = (i / 64) as i32 * 15;
                self.canvas
                    .fill_rect(Rect::new(x, y, 15, 15))
                    .expect("rectangle fill");
            }
        }

        self.canvas.present();
    }
}
