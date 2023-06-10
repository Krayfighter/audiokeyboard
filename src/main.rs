


mod sound_types;
// pub mod ui;
mod voice_algorithms;


use core::panic;


fn main() {

        // NOTE: I am using rodio for audio instead of sdl2::audio
        // I may want to change this in the future to reduce the dependency
        // count, but sdl2::audio works a little different so it would be
        // a significant investment

    /* audio initialization */
    // define the audio stream and its handle

    // TODO move into match statement
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // TODO move into match statement
    // get a sink for the output stream
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.15); // reduce volume

    // define the sender_receiver pair (gets the sender back)
    let (sound_gen, sender) = sound_types::SoundGenerator::new(
        voice_algorithms::_sine_function,
    );

    sink.append(sound_gen);
    // END AUDIO SETUP

    let sdl_context = match sdl2::init() {
        Ok(sdl) => sdl,
        Err(errcode) => {panic!("sdl init error: {}", errcode)},
    };

    let sdl_video = match sdl_context.video() {
        Ok(video) => video,
        Err(errcode) => panic!("sdl video init error: {}", errcode),
    };

    let window = sdl_video
        .window("AudioKeyboard", 800, 600)
        .position_centered()
        .build()
        .map_err(|error| {error.to_string()}).unwrap()
    ;

    let mut canvas = match window.into_canvas()
        .build()
        .map_err(|e| e.to_string()
    ) {
        Ok(canvas) => canvas,
        Err(errcode) => panic!("canvas creation error: {}", errcode),
    };

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = match sdl_context.event_pump() {
        Ok(epump) => epump,
        Err(errcode) => panic!("event creation failed: {}", errcode),
    }; // I believe this creates a thread synced queue for event to be received from

    let mut previous_keys: std::collections::HashSet<sdl2::keyboard::Keycode> = std::collections::HashSet::new();


    'main: loop {
        for event in events.poll_iter() {
            // I still don't know what if let really does
            if let sdl2::event::Event::Quit{..} = event {
                break 'main;
            }
        }
        
        let keys: std::collections::HashSet<sdl2::keyboard::Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(sdl2::keyboard::Keycode::from_scancode)
            .collect()
        ;

        if keys.contains(&sdl2::keyboard::Keycode::Escape) {
            break 'main;
        }

        let pressed_keys = &keys - &previous_keys;
        let released_keys = &previous_keys - &keys;


        for key in pressed_keys {
            match sender.send((sound_types::key_to_note(key), true)) {
                Ok(_) => {},
                Err(_) => println!("error sending key"),
            }
        }

        for key in released_keys {
            match sender.send((sound_types::key_to_note(key), false)) {
                Ok(_) => {},
                Err(_) => println!("error sending key"),
            }
        }

        previous_keys = keys;

        std::thread::sleep(std::time::Duration::from_millis(18));

    };






}