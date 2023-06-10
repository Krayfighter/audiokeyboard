

// use std::collections::HashMap;
use std::sync::mpsc::{
    Receiver,
    TryRecvError
};
use std::sync::mpsc;

// the constant for the change in angle at the current audio playback rate
const SAMPLE_RATE: u32 = 44100;
const ANGLE_CONSTANT: f32 = (std::f32::consts::PI*2.0)/SAMPLE_RATE as f32;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TET {
    A = 1,
    Bb = 2,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
}

// #[derive(PartialEq)]
impl TET {
    #[allow(dead_code)]
    pub fn from_int(value: i32) -> Self {
        return match value {
            1 => TET::A,
            2 => TET::Bb,
            3 => TET::B,
            4 => TET::C,
            5 => TET::Db,
            6 => TET::D,
            7 => TET::Eb,
            8 => TET::E,
            9 => TET::F,
            10 => TET::Gb,
            11 => TET::G,
            12 => TET::Ab,
            _ => {
                println!("unmatched int to TET {}", value);
                TET::A
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Note {
    tone: TET,
    octave: u8
}

impl Note {
    #[allow(dead_code)]
    pub fn new(tone: TET, octave: u8) -> Self {
        return Self{
            tone,
            octave
        }
    }

    pub fn freq(&self) -> f32 {
        // see https://en.wikipedia.org/wiki/12_equal_temperament#Mathematical_properties
        // for more details
        return (440 as f32)* //reference pitch
        ( // calculate the pitch offest
            ((2 as f32).powf(1.0/12.0)). // get 12th root of two
            powf( // to the power of the key number (n) - 49 (a)
                ((self.octave) as f32 * 12.0+ // the number of keys per octave
                // for some reason the normal tone produced is an augmented fifth
                // above where it should be so we subtract 8 half-steps to even it out
                (self.tone as i32) as f32-8.0)- // add the current key
                49.0 // the value of the reference key
            )
        )
    }

    // TODO remove ?
    #[allow(dead_code)]
    pub fn tone(&self) -> TET {
        return self.tone;
    }
}


pub struct SoundGenerator {
    audio_callback: fn(f32) -> f32,
    sample_number: usize, // the current x value (in terms of a graph)
    active_keys: Vec<Note>, // TODO might be better as double linked list ?
    rx: Receiver<(Option<Note>, bool)> // carry note and whether or not it's pressed
}

impl SoundGenerator {
    pub fn new(callback: fn(f32) -> f32) -> (Self, mpsc::Sender<(Option<Note>, bool)>) {
        // rx is owned by the SoundGenerator struct, and ownership of tx is handed to the main thread
        let (tx, rx) = mpsc::channel::<(Option<Note>, bool)>();
        return (
            Self {
                audio_callback: callback,
                sample_number: 0,
                active_keys: vec!(),
                rx
            },
            tx
        )
    }
}

impl Iterator for SoundGenerator{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        { // receive any updates from main thread
            let mut state_change: Result<(Option<Note>, bool), TryRecvError>;
            loop {
                state_change = self.rx.try_recv();
                match state_change {
                    Ok(state_change) => {
                        if state_change.1 {
                                // println!("state on");
                            // add state_change.0 to the active keys
                            match state_change.0 {
                                Some(change) => {
                                    // println!("{}", change.freq());
                                    self.active_keys.push(change);
                                },
                                None => {},
                            };
                        }else {

                            match state_change.0 {
                                Some(change) => self.active_keys.retain(|&x| x != change),
                                None => {},
                            };
                                // println!("state off");
                            // should remove all instances of state_change.0 from active_keys
                            // self.active_keys.retain(|&x| x != state_change.0);
                        }
                    }
                    Err(_) => {break;} // this means the no more items are in the receiver queue
                }
            }
        }

        self.sample_number = self.sample_number.wrapping_add(1);

        let mut sample_value: f32 = 0.0;
        let key_count = self.active_keys.len();

        let x_value = (self.sample_number as f32) * ANGLE_CONSTANT;

        for key in &self.active_keys {
            // println!("{}, {}, {:?}", self.sample_number, key.freq(), key);
            sample_value += (self.audio_callback)(
                x_value* // take the current x value
                key.freq() // and multiply by the fundamental frequency of the current note
            );
        }

        // because we just added the y values from the above notes
        // we must divide by the number of notes played to get the average
        // so that the volume does not increase with number of keys pressed
        sample_value /= key_count as f32;

        if key_count > 0 {
            // println!("value: {}, toned down: {}", sample_value.tanh(), sample_value.tanh()*0.5);
            return Some(sample_value.tanh() as f32*0.5);
        }else {
            return Some(0.0);
        }
    }

}


impl rodio::Source for SoundGenerator {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        return None
    }

    #[inline]
    fn channels(&self) -> u16 {
        return 1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        return SAMPLE_RATE
    }

    #[inline]
    fn total_duration(&self) -> Option<std::time::Duration> {
        return None
    }
}

pub enum OctaveMod {
    ShiftDown,
    ShiftUp,
}

#[inline]
pub fn key_to_note(key: sdl2::keyboard::Keycode) -> Option<Note> {
    // use sound_types::{TET, Note};
    // const base: u8 = 4;
    // let mut octave_shift = 4; // starting point
    // let shift: u8 = match modifier {
    //     Some(_mod) => {
    //         match _mod {
    //             OctaveMod::ShiftDown => 3,
    //             OctaveMod::ShiftUp => 5,
    //         }
    //     },
    //     None => 4,
    // };
    // println!("{}", shift);
    let shift = 4;

    // 

    match key {
        sdl2::keyboard::Keycode::A => Some(Note::new(TET::Ab, shift-2)),
        sdl2::keyboard::Keycode::Z => Some(Note::new(TET::A, shift-1)),
        sdl2::keyboard::Keycode::S => Some(Note::new(TET::Bb,shift-1)),
        sdl2::keyboard::Keycode::X => Some(Note::new(TET::B, shift-1)),
        sdl2::keyboard::Keycode::C => Some(Note::new(TET::C, shift-1)),
        sdl2::keyboard::Keycode::F => Some(Note::new(TET::Db, shift-1)),
        sdl2::keyboard::Keycode::V => Some(Note::new(TET::D, shift-1)),
        sdl2::keyboard::Keycode::G => Some(Note::new(TET::Eb, shift-1)),
        sdl2::keyboard::Keycode::B => Some(Note::new(TET::E, shift-1)),
        sdl2::keyboard::Keycode::N => Some(Note::new(TET::F, shift-1)),
        sdl2::keyboard::Keycode::J => Some(Note::new(TET::Gb, shift-1)),
        sdl2::keyboard::Keycode::M => Some(Note::new(TET::G, shift-1)),
        sdl2::keyboard::Keycode::K => Some(Note::new(TET::Ab, shift-1)),
        sdl2::keyboard::Keycode::Comma => Some(Note::new(TET::A, shift)),
        sdl2::keyboard::Keycode::L => Some(Note::new(TET::Bb,shift)),
        sdl2::keyboard::Keycode::Period => Some(Note::new(TET::B, shift)),

        // TOD ROW
        sdl2::keyboard::Keycode::Q => Some(Note::new(TET::C, shift)),
        sdl2::keyboard::Keycode::Num2 => Some(Note::new(TET::Db, shift)),
        sdl2::keyboard::Keycode::W => Some(Note::new(TET::D, shift)),
        sdl2::keyboard::Keycode::Num3 => Some(Note::new(TET::Eb, shift)),
        sdl2::keyboard::Keycode::E => Some(Note::new(TET::E, shift)),
        sdl2::keyboard::Keycode::R => Some(Note::new(TET::F, shift)),
        sdl2::keyboard::Keycode::Num5 => Some(Note::new(TET::Gb, shift)),
        sdl2::keyboard::Keycode::T => Some(Note::new(TET::G, shift)),
        sdl2::keyboard::Keycode::Num6 => Some(Note::new(TET::Ab, shift)),
        sdl2::keyboard::Keycode::Y => Some(Note::new(TET::A, shift+1)),
        sdl2::keyboard::Keycode::Num7 => Some(Note::new(TET::Bb,shift+1)),
        sdl2::keyboard::Keycode::U => Some(Note::new(TET::B, shift+1)),
        sdl2::keyboard::Keycode::I => Some(Note::new(TET::C, shift+1)),
        sdl2::keyboard::Keycode::Num9 => Some(Note::new(TET::Db, shift+1)),
        sdl2::keyboard::Keycode::O => Some(Note::new(TET::D, shift+1)),
        sdl2::keyboard::Keycode::Num0 => Some(Note::new(TET::Eb, shift+1)),
        sdl2::keyboard::Keycode::P => Some(Note::new(TET::E, shift+1)),
        _ => {
            // TODO fix this
            None
        }
    }
}

// TODO implement as hashmap
// pub fn generate_freq_array() -> vec<(Note, f32)> {
//     todo!()
// }

