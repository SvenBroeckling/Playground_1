use sdl3::pixels::Color;
use sdl3::rect::{Point, Rect};
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Star {
    pub(crate) position: Point,
    distance: u8,
}

impl Star {
    pub fn new(x: i32, y: i32, distance: u8) -> Star {
        Star {
            position: Point::new(x, y),
            distance,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(
            255 - self.distance,
            255 - self.distance,
            255 - self.distance,
        ));
        let rect = Rect::new(
            self.position.x,
            self.position.y,
            (self.distance / 60) as u32,
            (self.distance / 60) as u32,
        );
        canvas.fill_rect(rect).unwrap();
    }

    pub fn render_at(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        canvas.set_draw_color(Color::RGB(
            255 - self.distance,
            255 - self.distance,
            255 - self.distance,
        ));
        let rect = Rect::new(
            x,
            y,
            (self.distance / 60) as u32,
            (self.distance / 60) as u32,
        );
        canvas.fill_rect(rect).unwrap();
    }

    pub fn update(&mut self, center_x: i32, center_y: i32) {
        let velocity = (1.0 - self.distance as f32 / 255.0) * 5.0 + 1.0;

        let dx = self.position.x - center_x;
        let dy = self.position.y - center_y;

        let length = ((dx * dx + dy * dy) as f32).sqrt();
        let direction_x = if length > 0.0 {
            dx as f32 / length
        } else {
            0.0
        };
        let direction_y = if length > 0.0 {
            dy as f32 / length
        } else {
            0.0
        };

        self.position += Point::new(
            (direction_x * velocity) as i32,
            (direction_y * velocity) as i32,
        );
    }
}
