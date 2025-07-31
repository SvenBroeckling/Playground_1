use crate::stars::Star;
use crate::viewport::Viewport;
use rand::Rng;
use sdl3::mouse::MouseButton;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Scene {
    lifetime: u32,
    stars: Vec<Star>,
    size: Rect,
    pub(crate) viewport: Viewport,
}

impl Scene {
    pub fn new(width: u32, height: u32, viewport_width: u32, viewport_height: u32) -> Scene {
        let center_x = width as i32 / 2;
        let center_y = height as i32 / 2;
        let vw = viewport_width as i32;
        let vh = viewport_height as i32;

        let mut scene = Scene {
            lifetime: 0,
            stars: Vec::with_capacity((width * height / 10000) as usize),
            size: Rect::new(0, 0, width, height),
            viewport: Viewport::new(
                center_x - vw / 2,
                center_y - vh / 2,
                viewport_width,
                viewport_height,
            ),
        };

        let num_stars = (width * height) / 10000;
        for _ in 0..num_stars {
            scene.stars.push(Star::new(
                rand::rng().random_range(0..width) as i32,
                rand::rng().random_range(0..height) as i32,
                rand::rng().random_range(1..255),
            ));
        }

        scene
    }

    pub fn handle_mouse(&mut self, x: f32, y: f32, button: MouseButton) {
        let world_x = x as i32 + self.viewport.view_area.x();
        let world_y = y as i32 + self.viewport.view_area.y();
        let distance = rand::random::<u8>();
        self.stars.push(Star::new(world_x, world_y, distance));
    }

    pub fn move_viewport_up(&mut self) {
        self.viewport.move_up();
        self.viewport
            .constrain(self.size.width(), self.size.height());
    }

    pub fn move_viewport_down(&mut self) {
        self.viewport.move_down();
        self.viewport
            .constrain(self.size.width(), self.size.height());
    }

    pub fn move_viewport_left(&mut self) {
        self.viewport.move_left();
        self.viewport
            .constrain(self.size.width(), self.size.height());
    }

    pub fn move_viewport_right(&mut self) {
        self.viewport.move_right();
        self.viewport
            .constrain(self.size.width(), self.size.height());
    }

    pub fn center_viewport_to_bounds(&mut self, rect: Rect) {
        self.viewport.view_area = rect;
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for star in self.stars.iter_mut() {
            if self.viewport.view_area.contains_point(star.position) {
                let screen_x = star.position.x - self.viewport.view_area.x();
                let screen_y = star.position.y - self.viewport.view_area.y();
                star.render_at(canvas, screen_x, screen_y);
            }
        }
    }

    pub fn update(&mut self) {
        self.lifetime += 1;
        self.create_random_stars();
        self.clear_out_of_bounds_stars();

        let center_x = (self.size.width() / 2) as i32;
        let center_y = (self.size.height() / 2) as i32;

        for star in self.stars.iter_mut() {
            star.update(center_x, center_y);
        }
    }

    fn create_random_stars(&mut self) {
        let center_x = (self.size.width() / 2) as i32;
        let center_y = (self.size.height() / 2) as i32;

        self.stars.push(Star::new(
            center_x + rand::rng().random_range(-100..100),
            center_y + rand::rng().random_range(-100..100),
            rand::rng().random_range(1..255),
        ));
    }

    fn clear_out_of_bounds_stars(&mut self) {
        self.stars
            .retain(|star| self.size.contains_point(star.position));
    }
}
