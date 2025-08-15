extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem
        .window("My SDL3 Window", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to build window!");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        // Add rendering logic here if needed
    }
}
