// use crate::ArbitrarySound;
mod sound_generation;
mod voice_algorithms;

use sound_generation::sound_generation::*;
use voice_algorithms::voice_algorithms::*;

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
    let (tx, rx) = mpsc::channel::<NumberedBool>();

    /* audio initialization */
    let (
        _stream,
        stream_handle
    ) = OutputStream::try_default().unwrap();

    // let (_stream2, stream_handle2) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // let sink = Sink::try_new(&stream_handle2).unwrap();

    sink.set_volume(0.1);
    // sink.set_volume(0.1);


    // sink.append(ArbitrarySound::new(261.6256, sine_function));
    // sink.pause();

    let source = ArbitrarySound::new(
        sine_function,
        [
            261.6256, // C
            293.6648, // D
            329.6276, // E
            349.2282, // F
            391.9954, // G
            440.0000, // A
            493.8833, // B
            523.2511 // C
        ],
        rx
    );

    // rx.recv();

    sink.append(source);

    // rx;
    // sink.pause();

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
                    if input == 'd' {
                        // let tmp =  tx.send(
                        //     NumberedBool::new(0, true)
                        // );
                        // match tmp {
                        //     Ok(tmp) => {},
                        //     Err(tmp) => {
                        //         print!("{}", tmp)
                        //     }
                        // }
                        tx.send(
                            NumberedBool::new(0, true)
                        ).unwrap();
                        return true;
                    }else if input == 'f' {
                        tx.send(
                            NumberedBool::new(1, true)
                        ).unwrap();
                    }else if input == 'g' {
                        tx.send(
                            NumberedBool::new(2, true)
                        ).unwrap();
                    }else if input == 'h' {
                        tx.send(
                            NumberedBool::new(3, true)
                        ).unwrap();
                    }else if input == 'j' {
                        tx.send(
                            NumberedBool::new(4, true)
                        ).unwrap();
                    }else if input == 'k' {
                        tx.send(
                            NumberedBool::new(5, true)
                        ).unwrap();
                    }else if input == 'l' {
                        tx.send(
                            NumberedBool::new(6, true)
                        ).unwrap();
                    }else if input == ';' {
                        tx.send(
                            NumberedBool::new(7, true)
                        ).unwrap();
                    }else {}
                    return true
                }
            }
        },
        enums::Event::KeyUp => {
            match app::event_key() {
                _ => {
                    let input = app::event_key().to_char().unwrap();
                    if input == 'd' {
                        // let tmp = tx.send(
                        //     NumberedBool::default()
                        // );
                        // match tmp {
                        //     Ok(tmp) => {},
                        //     Err(tmp) => {
                        //         print!("{:?}", tmp)
                        //     }
                        // }
                        tx.send(
                            NumberedBool::new(0, false)
                        ).unwrap();
                    }else if input == 'f' {
                        tx.send(
                            NumberedBool::new(1, false)
                        ).unwrap();
                    }else if input == 'g' {
                        tx.send(
                            NumberedBool::new(2, false)
                        ).unwrap();
                    }else if input == 'h' {
                        tx.send(
                            NumberedBool::new(3, false)
                        ).unwrap();
                    }else if input == 'j' {
                        tx.send(
                            NumberedBool::new(4, false)
                        ).unwrap();
                    }else if input == 'k' {
                        tx.send(
                            NumberedBool::new(5, false)
                        ).unwrap();
                    }else if input == 'l' {
                        tx.send(
                            NumberedBool::new(6, false)
                        ).unwrap();
                    }else if input == ';' {
                        tx.send(
                            NumberedBool::new(7, false)
                        ).unwrap();
                    }else {}
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
