extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use std::time::Duration;

mod view;
use view::board_view;

mod model;
use model::game::make_blank_board;
use model::game::GameState;

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

    let board_view: board_view::Renderer = board_view::Renderer {
        screen_area: Rect::new(0, 0, screen_width, screen_height),
        clear_color: Color::RGB(64, 192, 255),
    };

    let mut game_state = GameState { board: make_blank_board() };

    game_state.jumble_board();
    game_state.print_board();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                },
                _ => {}
            }
        }
        // Add rendering logic here if needed

        board_view.render(&mut canvas, &game_state.board);
        canvas.present();
    }
}
