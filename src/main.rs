// use crate::ArbitrarySound;
mod sound_generation;
mod voice_algorithms;
mod video_functions;

use sound_generation::sound_generation::*;
use voice_algorithms::voice_algorithms::*;
use video_functions::video_functions::*;

// use std::{
//     io::stdin,
// };

use std::sync::mpsc;

use rodio::{
    OutputStream,
    Sink,
    // Source,
};

use fltk::{
    app,
    enums,
    enums::Key,
    prelude::*,
    window::Window,
    // button::Button
};


fn main() {

    /* channel definition */
    // define the sender and receiver to send to the audio thread
    let (tx, rx) = mpsc::channel::<NoteIndicator>();


    /* audio initialization */
    // define the audio stream and its handle
    let (
        _stream,
        stream_handle
    ) = OutputStream::try_default().unwrap();

    // get a sink for the output stream
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.15); // reduce volume

    let source = ArbitrarySound::new(
        _quadruple_sine,
        rx
    ); // this is the source for all generated audio

    sink.append(source); // put the source into the sink


    /* video initialization */
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window = Window::new(
        100, 100,
        400, 300,
        "Example fltk app"
    );

    window.handle(move |_, event| match event {
        enums::Event::KeyDown => {
            match app::event_key() {
                Key::Escape => {
                    app.quit();
                    return true
                },
                _ => {
                    let input = app::event_key().to_char().unwrap();
                    parse_keyboard_character(input, true, tx.clone());
                    return true
                }
            }
        },
        enums::Event::KeyUp => {
            match app::event_key() {
                _ => {
                    let input = app::event_key().to_char().unwrap();
                    parse_keyboard_character(input, false, tx.clone());
                    return true
                }
            }
        },
        _ => false
    });

    window.end();
    window.show();

    app.run().unwrap();
}
