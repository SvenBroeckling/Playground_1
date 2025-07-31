use sdl3::rect::Rect;

pub struct Viewport {
    pub view_area: Rect,
    pub speed: i32,
}

impl Viewport {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Viewport {
            view_area: Rect::new(x, y, width, height),
            speed: 10,
        }
    }

    pub fn move_up(&mut self) {
        self.view_area.set_y(self.view_area.y() - self.speed);
    }

    pub fn move_down(&mut self) {
        self.view_area.set_y(self.view_area.y() + self.speed);
    }

    pub fn move_left(&mut self) {
        self.view_area.set_x(self.view_area.x() - self.speed);
    }

    pub fn move_right(&mut self) {
        self.view_area.set_x(self.view_area.x() + self.speed);
    }

    pub fn constrain(&mut self, scene_width: u32, scene_height: u32) {
        if self.view_area.x() < 0 {
            self.view_area.set_x(0);
        } else if self.view_area.x() + self.view_area.width() as i32 > scene_width as i32 {
            self.view_area
                .set_x(scene_width as i32 - self.view_area.width() as i32);
        }

        if self.view_area.y() < 0 {
            self.view_area.set_y(0);
        } else if self.view_area.y() + self.view_area.height() as i32 > scene_height as i32 {
            self.view_area
                .set_y(scene_height as i32 - self.view_area.height() as i32);
        }
    }
}
