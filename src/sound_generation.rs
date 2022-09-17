

pub mod sound_generation {
    use std::{
        sync::mpsc::Receiver
        // sync::atomic::{
        //     AtomicBool,
        //     Ordering::Relaxed
        // }
    };

    const PI: f32 = std::f32::consts::PI;
    const RADIAN: f32 = 2.0 * PI;

    // impl Copy for AtomicBool {

    // }

    pub struct NumberedBool {
        number: usize,
        value: bool
    }

    impl NumberedBool {
        pub fn new(number: usize, value: bool) -> Self {
            return Self {
                number: number,
                value: value
            }
        }
        // pub fn default() -> Self {
        //     return Self {
        //         number: 0,
        //         value: false
        //     }
        // }
    }

    impl Default for NumberedBool {
        fn default() -> Self {
        Self { number: Default::default(), value: Default::default() }
    }
    }

    #[derive (Debug)]
    pub struct ArbitrarySound {
        generator_function: fn(f32) -> f32,
        sample_num: usize,
        frequencies: [f32; 8],
        active_signals: [bool; 8],
        rx: Receiver<NumberedBool>
        // active_signals: [AtomicBool; 8]
    }

    impl ArbitrarySound {
        // #[inline]
        pub fn new(genfunc: fn(f32) -> f32, freqs: [f32; 8], rx: Receiver<NumberedBool>) -> ArbitrarySound{
            return ArbitrarySound {
                generator_function: genfunc,
                sample_num: 0,
                frequencies: freqs,
                active_signals: [false, false, false, false, false, false, false, false],
                rx: rx
                // active_signals: [AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false)]
                // active_signals: [false]
            };
        }

        // pub fn read_signals(&self) -> [bool; 8] {
        //     return [
        //         self.active_signals[0].load(Relaxed),
        //         self.active_signals[1].load(Relaxed),
        //         self.active_signals[2].load(Relaxed),
        //         self.active_signals[3].load(Relaxed),
        //         self.active_signals[4].load(Relaxed),
        //         self.active_signals[5].load(Relaxed),
        //         self.active_signals[6].load(Relaxed),
        //         self.active_signals[7].load(Relaxed)
        //     ]
        // }

        // pub fn activate_signal(&mut self, func_number: usize) {
        //     self.active_signals[func_number].store(true, Relaxed);
        // }

        // pub fn deactivate_signal(&mut self, func_number: usize) {
        //     self.active_signals[func_number].store(false, Relaxed);
        // }
    }

    impl Iterator for ArbitrarySound {
        type Item = f32;

        #[inline]
        fn next(&mut self) -> Option<f32> {
            {
                let mut _tmp = Ok(NumberedBool::default());
                loop {
                    _tmp = self.rx.try_recv();
                    match _tmp {
                        Ok(_tmp) => {
                            self.active_signals[_tmp.number] = _tmp.value;
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

            let mut fucktard: usize = 0;
            for val in self.active_signals {
                if val {
                    yval += (self.generator_function)(self.frequencies[fucktard]*para_xvalue);
                    ydiv += 1;
                }
                fucktard += 1;
            }

            if ydiv < 1 {
                // println!("nothing to do");
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