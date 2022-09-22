

pub mod sound_generation {

    pub const TOPSCALE1: [f32; 21] = [
        207.6523, // Ab3
        220.0000, // A3
        233.0819, // Bb3
        246.9417, // B3
        261.6256, // C4
        277.1826, // Db4
        293.6648, // D4
        311.1270, // Eb4
        329.6276, // E4
        349.2282, // F4
        369.9944, // Gb4
        391.9954, // G4
        415.3047, // Ab4
        440.0000, // A4
        466.1638, // Bb4
        493.8833, // B4
        523.2511, // C5
        554.3653, // Db5
        587.3295, // D5
        622.2540, // Eb5
        659.2551, // E5
    ];

    pub const TOPSCALE1BLACK: [f32; 9] = [
        207.6523, // Ab3
        233.0819, // Bb3
        277.1826, // Db4
        311.1270, // Eb4
        369.9944, // Gb4
        415.3047, // Ab4
        466.1638, // Bb4
        554.3653, // Db5
        622.2540, // Eb5
    ];


    const BOTTOMSCALE2: [f32; 18] = [
        698.4565, // F5
        739.9888, // Gb5
        783.9909, // G5
        830.6094, // Ab5
        880.0000, // A5
        932.3275, // Bb5
        987.7666, // B5
        1046.502, // C6
        1108.731, // Db6
        1174.659, // D6
        1244.508, // Eb6
        1318.510, // E6
        1396.913, // F6
        1479.978, // Gb6
        1567.982, // G6
        1661.219, // Ab6
        1760.000, // A6
        1864.655, // Bb6
    ];

    use std::{
        sync::mpsc::Receiver
    };

    const PI: f32 = std::f32::consts::PI;
    const RADIAN: f32 = 2.0 * PI;


    pub struct NoteIndicator {
        pub number: usize,
        pub upper: bool,
        pub value: bool
    }

    impl NoteIndicator {
        pub fn new(number: usize, upper: bool, value: bool) -> Self {
            return Self {
                number: number,
                upper: upper,
                value: value
            }
        }
    }

    impl Default for NoteIndicator {
        fn default() -> Self {
            Self { number: Default::default(), upper: Default::default(), value: Default::default() }
        }
        // fn default() -> Self {
    // }
    }

    #[derive (Debug)]
    pub struct ArbitrarySound {
        generator_function: fn(f32) -> f32,
        sample_num: usize,
        upper_signals: [bool; 21],
        lower_signals: [bool; 18],
        rx: Receiver<NoteIndicator>
    }

    impl ArbitrarySound {
        // #[inline]
        pub fn new(genfunc: fn(f32) -> f32, rx: Receiver<NoteIndicator>) -> ArbitrarySound {
            return ArbitrarySound {
                generator_function: genfunc,
                sample_num: 0,
                upper_signals: [false; 21],
                lower_signals: [false; 18],
                rx: rx
            };
        }

    }

    impl Iterator for ArbitrarySound {
        type Item = f32;

        #[inline]
        fn next(&mut self) -> Option<f32> {
            {
                let mut _tmp = Ok(NoteIndicator::default());
                loop {
                    _tmp = self.rx.try_recv();
                    match _tmp {
                        Ok(_tmp) => {
                            // self.active_signals[_tmp.number] = _tmp.value;
                            if _tmp.upper {
                                self.upper_signals[_tmp.number] = _tmp.value;
                            }else {
                                self.lower_signals[_tmp.number] = _tmp.value;
                            }
                            // contine;
                        }
                        Err(_tmp) => {break;}
                    }
                }
            }
            // println!("running inside audio loop");
            
            self.sample_num = self.sample_num.wrapping_add(1);

            let para_xvalue: f32 = RADIAN * self.sample_num as f32 / 48000.0;

            let mut yval: f32 = 0.0;
            let mut ydiv: usize = 0;

            let mut iter: usize = 0;
            for val in self.upper_signals {
                if val {
                    yval += (self.generator_function)(TOPSCALE1[iter]*para_xvalue);
                    ydiv += 1;
                }
                iter += 1;
            }
            iter = 0;
            for val in self.lower_signals {
                if val {
                    yval += (self.generator_function)(BOTTOMSCALE2[iter]*para_xvalue);
                    ydiv += 1;
                }
                iter += 1;
            }

            if ydiv < 1 {
                // println!("nothing to do");
                self.sample_num -= 1;
                return Some(0.0);
            }else {
                return Some(yval/ydiv as f32);
            }
            // return Some(0.0);
        }
    }

    impl rodio::Source for ArbitrarySound {
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
            return 48000
        }

        #[inline]
        fn total_duration(&self) -> Option<std::time::Duration> {
            return None
        }
    }

}