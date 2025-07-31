extern crate sdl3;

use crate::scenes;
use crate::scenes::Scene;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::video::Window;
use sdl3::{Sdl, VideoSubsystem};
use std::time::Duration;

pub struct Game {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: sdl3::render::Canvas<Window>,
    is_fullscreen: bool,
}

impl Game {
    pub fn new(title: &str, width: u32, height: u32) -> Game {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas();

        Game {
            sdl_context,
            video_subsystem,
            canvas,
            is_fullscreen: false,
        }
    }

    fn toggle_fullscreen(&mut self, scene: &mut Scene) {
        self.is_fullscreen = !self.is_fullscreen;
        let window = self.canvas.window_mut();

        if self.is_fullscreen {
            if let Ok(displays) = self.video_subsystem.displays() {
                if let Some(display) = displays.get(0) {
                    if let Ok(bounds) = display.get_bounds() {
                        scene.center_viewport_to_bounds(bounds);
                        window
                            .set_size(bounds.width() as u32, bounds.height() as u32)
                            .unwrap_or_else(|e| eprintln!("Failed to set window size: {}", e));
                    }
                }
            }

            window
                .set_fullscreen(true)
                .unwrap_or_else(|e| eprintln!("Failed to enter fullscreen: {}", e));
        } else {
            window
                .set_fullscreen(false)
                .unwrap_or_else(|e| eprintln!("Failed to exit fullscreen: {}", e));
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let window_size = self.canvas.window().size();
        let mut scene = scenes::Scene::new(8000, 8000, window_size.0, window_size.1);

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::F),
                        ..
                    } => self.toggle_fullscreen(&mut scene),
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => scene.handle_mouse(x as f32, y as f32, mouse_btn),
                    _ => {}
                }
            }

            let keyboard_state = event_pump.keyboard_state();

            if keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::W)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::K)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::Up)
            {
                scene.move_viewport_up();
            }
            if keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::S)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::J)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::Down)
            {
                scene.move_viewport_down();
            }
            if keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::A)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::H)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::Left)
            {
                scene.move_viewport_left();
            }
            if keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::D)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::L)
                || keyboard_state.is_scancode_pressed(sdl3::keyboard::Scancode::Right)
            {
                scene.move_viewport_right();
            }

            scene.update();
            scene.render(&mut self.canvas);
            self.canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
