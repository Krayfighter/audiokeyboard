
use std::{io::{
    Write
}, any::Any};

use crossterm::{
    queue,
    execute,
    QueueableCommand,
};

use cpal::traits::StreamTrait;

mod state;
mod sound_types;
mod voice_algorithms;



fn unwrap_clean<T: Any, U: std::error::Error>(item: Result<T, U>) -> T {
    return match item {
        Ok(value) => value,
        Err(err) => {
            crossterm::terminal::disable_raw_mode().unwrap();
            panic!("{}", err);
        }
    };
}

// see https://en.wikipedia.org/wiki/12_equal_temperament#Mathematical_properties
fn note_freq(note_num: u32) -> f32 {
    440.0f32 * 2.0f32.powf((note_num as i32-49) as f32 / 12.)
}

fn get_freq(chr: char) -> Option<f32> {
    // the number passed to `note_freq` is the note number
    // on a piano with 49 being middle A or 440 hz
    return match chr {
        '1' => Some(note_freq(48)),
        'q' => Some(note_freq(49)),
        '2' => Some(note_freq(50)),
        'w' => Some(note_freq(51)),
        'e' => Some(note_freq(52)),
        '4' => Some(note_freq(53)),
        'r' => Some(note_freq(54)),
        '5' => Some(note_freq(55)),
        't' => Some(note_freq(56)),
        'y' => Some(note_freq(57)),
        '7' => Some(note_freq(58)),
        'u' => Some(note_freq(59)),
        '8' => Some(note_freq(60)),
        'i' => Some(note_freq(61)),
        '9' => Some(note_freq(62)),
        'o' => Some(note_freq(63)),
        'p' => Some(note_freq(64)),
        '-' => Some(note_freq(65)),
        '[' => Some(note_freq(66)),
        '=' => Some(note_freq(67)),
        ']' => Some(note_freq(68)),
        _ => None
    }
}

fn main() {

    let mut keyboard_state = state::KeyboardState::new();

    let stream = sound_types::AudioState::make_stream(keyboard_state.clone());
    stream.play().unwrap();


    let mut stdout = std::io::stdout();

    // setup the terminal for advanced usage
    unwrap_clean(crossterm::terminal::enable_raw_mode());
    unwrap_clean(execute!(
        stdout,
        crossterm::event::PushKeyboardEnhancementFlags (
            crossterm::event::KeyboardEnhancementFlags::all()
        )
    ));

    stdout.queue(
        crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All
        )).unwrap();

    unwrap_clean(stdout.flush());


    'mainloop: loop {
        if let Err(_) = crossterm::event::poll(
            std::time::Duration::from_millis(50)
        ) {
            continue 'mainloop;
        }

        use crossterm::event::KeyCode as KC;
        use crossterm::event::KeyEventKind as KEK;


        unwrap_clean(queue!(
            stdout,
            crossterm::terminal::Clear(
                crossterm::terminal::ClearType::All)
        ));

        match crossterm::event::read() {
            Ok(
                crossterm::event::Event::Key(keyevent)
            ) => { match keyevent.code {
                KC::Esc => {
                    break 'mainloop;
                },
                KC::Char(chr) => {
                    match keyevent.kind {
                        KEK::Press => {
                            if let Some(freq) = get_freq(chr) {
                                keyboard_state.set(chr, freq);
                            }
                        },
                        KEK::Release => {
                            keyboard_state.remove(chr);
                        },
                        KEK::Repeat => {},
                    }
                    unwrap_clean(execute!(
                        stdout,
                        crossterm::cursor::MoveTo(0, 0),
                        crossterm::style::Print(format!(
                            "Character -> {} | Type -> {:?} | as hex -> {:#x}",
                            chr, keyevent.kind,
                            TryInto::<u8>::try_into(chr).unwrap()
                        )),
                    ));
                }
                other => {
                    unwrap_clean(execute!(
                        stdout,
                        crossterm::cursor::MoveTo(0, 0),
                        crossterm::style::Print(format!("{:?}", other))
                    ));
                    continue 'mainloop;
                },
            }},
            Ok(_) => {},
            Err(e) => {
                println!("encountered input error: {}", e);
                break 'mainloop;
            }
        }
    }

    // deinit terminal modes
    crossterm::terminal::disable_raw_mode().unwrap();
    execute!(
        stdout,
        crossterm::event::PopKeyboardEnhancementFlags
    ).unwrap();

    // stream.pause().unwrap();
}
