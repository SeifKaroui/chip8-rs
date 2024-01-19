use sdl2::{pixels, rect::Rect, render::Canvas, video::Window};

use super::{HEIGHT, SCALE, WIDTH};

pub struct DisplayManager {
    canvas: Canvas<Window>,
}
impl DisplayManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "Chip8 Emulator",
                (WIDTH * SCALE as usize) as u32,
                (HEIGHT * SCALE as usize) as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        DisplayManager { canvas }
    }

    pub fn draw_screen(&mut self, screen: &[[u8; WIDTH]; HEIGHT]) {
        for (y, row) in screen.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;
                let color = if pixel == 0 {
                    pixels::Color::RGB(0, 0, 0)
                } else {
                    pixels::Color::RGB(255, 255, 255)
                };

                self.canvas.set_draw_color(color);
                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
            }
        }
        self.canvas.present();
    }
}
