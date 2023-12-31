
#[macro_use] extern crate crossterm;
#[macro_use] extern crate anyhow;

use cpal::traits::StreamTrait;

mod state;
mod sound_types;
mod voice_algorithms;

mod ui;


fn main() {

    let keyboard_state = state::KeyboardState::new();

    let stream = sound_types::AudioState::make_stream(keyboard_state.clone());
    stream.play().unwrap();

    let mut ui = ui::UiThread::new(keyboard_state.clone());

    if let Err(_) = ui::UiThread::init_drawing_area() {
        let _ = ui;
        return;
    }

    ui.run().unwrap();


    // stream.pause().unwrap();
}
