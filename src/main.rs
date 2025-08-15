extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use std::time::Duration;

pub fn main() {
    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_queue = sdl_context.event_pump().unwrap();
    let mut running = true;

    let window = video_subsystem
        .window("Hello SDL3", screen_width, screen_height)
        .position_centered()
        .build()
        .expect("Failed to build window!");

    let mut canvas = window.into_canvas();

    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let clear_color = Color::RGB(64, 192, 255);
    canvas.set_draw_color(clear_color);

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                } => {
                    println!("Mouse x: {}, y: {}", x, y);
                }
                _ => {}
            }
        }
        // Add rendering logic here if needed

        canvas.fill_rect(screen_area);
        canvas.present();
    }
}
