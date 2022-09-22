// use crate::ArbitrarySound;
mod sound_generation;
mod voice_algorithms;
mod video_functions;
mod fltk_extensions;

use sound_generation::sound_generation::*;
use voice_algorithms::voice_algorithms::*;
use video_functions::video_functions::*;
use fltk_extensions::fltk_extensions::*;

// use std::{
//     io::stdin,
// };

use std::{
    sync::mpsc,
    thread,
    time::Duration
};

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
    // define the sender and receiver to communicate with the graphics changer thread
    let (tx1, rx1) = mpsc::channel::<NoteIndicator>();


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
        _double_sine,
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

    let key_list = KeysWrapper::new();


    // keylist[0].set_color(enums::Color::White);
    // keylist[0].set

    // let mut key = PianoKey::new(50, 50, 5, 5, "A");
    // key.set_color(enums::Color::White);
    // key.set_callback(|_| println!("Clicked"));

    // let keyref = &mut keylist[0];

    window.handle(move |_, event| match event {
        enums::Event::KeyDown => {
            match app::event_key() {
                Key::Escape => {
                    app.quit();
                    return true
                },
                _ => {
                    let input = app::event_key().to_char().unwrap();
                    parse_keyboard_character(input, true, tx.clone(), tx1.clone());
                    // keyref.set_color(enums::Color::Gray0);
                    // keyref.redraw();
                    return true
                }
            }
        },
        enums::Event::KeyUp => {
            match app::event_key() {
                _ => {
                    let input = app::event_key().to_char().unwrap();
                    parse_keyboard_character(input, false, tx.clone(), tx1.clone());
                    // keyref.set_color(enums::Color::White);
                    // keyref.redraw();
                    return true
                }
            }
        },
        _ => false
    });

    window.end();
    window.show();


    thread::spawn(move || {
        let mut recvalue: Result<NoteIndicator, mpsc::TryRecvError>; // variable to hold received info
        loop {
            recvalue = rx1.try_recv(); // try to receive a NoteIndicator from the queue
            match recvalue {
                Ok(recvalue) => { // if NoteIndicator is received
                    // println!("alt thread -> number: {}, upper: {}, value: {}", recvalue.number, recvalue.upper, recvalue.value);
                    // mutable reference to the key in position specified by the NoteIndicator
                    let mut key_ref = (key_list.keys[recvalue.number]).borrow_mut();
                    // if recvalucl
                    if recvalue.value { // if note is on
                        key_ref.set_on(true);
                    }else { // if the note is off
                        key_ref.set_on(false);
                    }


                    let intermediate_usize: usize = recvalue.number.clone();

                    // println!("{}", recvalue.number);

                    if TOPSCALE1BLACK.contains(&TOPSCALE1[intermediate_usize.abs_diff(1)]) {
                        // println!("black key is bellow currernt");
                        // println!("inter {}", intermediate_usize-1);
                        // println!("recva {}", recvalue.number);
                        let mut tmp = key_list.keys[intermediate_usize.abs_diff(1)].borrow_mut();
                        tmp.redraw();
                        // tmp.
                    }
                    if intermediate_usize != 20 {
                        if TOPSCALE1BLACK.contains(&TOPSCALE1[intermediate_usize+1]) {
                        key_list.keys[intermediate_usize+1].borrow_mut().redraw();
                            // println!("black key is above current");
                        }
                    }

                    app::awake(); // awake the app to activate the redraws
                    continue // redo the loop if NoteIndicator is received
                },
                Err(_) => { // do nothing when no NoteIndicator is received
                    ()
                }
            }
            thread::sleep(Duration::from_millis(20)); // sleep 20 millis until restart to preserve performance
        }
    });

    app.run().unwrap();

}
