extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod game_state;
use game_state::{GameContext, GameState, Point};
mod game_renderer;
use game_renderer::Renderer;
mod game_config;
use game_config::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};

pub fn update(renderer: &mut Renderer, context: &mut GameContext) {
    renderer.draw(context);
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("game window", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Game state and renderer
    let mut game_context = GameContext::new();
    let mut renderer = Renderer::new(window).unwrap();
    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
        frame_counter += 1;
        if (frame_counter == 10) {
            update(&mut renderer, &mut game_context);
            frame_counter = 0;
        }
    }
    return Ok(());
}
