
use cpal::traits::{
    HostTrait,
    DeviceTrait
};

use std::f32::consts::TAU;
const FREQUENCY: f32 = 440.0; // hertz


pub struct AudioState {
    sample_rate: f32,
    current_x_value: f32,
    transform: fn(f32, f32, f32) -> f32
}

impl AudioState {

    #[inline]
    fn increment_sample(&mut self) {
        self.current_x_value += 1.;
        self.current_x_value %= self.sample_rate;
    }

    fn sine_function(&self, x_value: &mut f32) {
        *x_value = (
            TAU * self.current_x_value *
            (FREQUENCY / self.sample_rate)
        ).sin();
    }

    pub fn make_stream(
        keystates: crate::state::KeyboardState
    ) -> cpal::Stream {
        // get default host and output device
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .unwrap();
    
        // get the highest sample rate supported by device
        let mut supported_config_list = device
            .supported_output_configs()
            .unwrap();

        let output_config = supported_config_list
            .next()
            .unwrap()
            .with_max_sample_rate();

        let mut state = Self {
            sample_rate: output_config.sample_rate().0 as f32,
            current_x_value: 0.,
            transform: crate::voice_algorithms::sine_function
        };
    
        return device.build_output_stream(
            &output_config.config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for value in data.iter_mut() {
                    state.increment_sample();
                    let ksc = keystates.cloned();
                    let length = ksc.len();
                    *value = ksc
                        .iter()
                        .map(|(_, freq)| {
                            return ( state.transform ) (
                                state.current_x_value,
                                *freq,
                                state.sample_rate,
                            );
                        })
                        .map(|num| num/length as f32)
                        .reduce(|acc, num| acc+num)
                        .unwrap_or(0.);
                }
            },
            move |err| {
                println!("encountered error in audio stream: {}", err);
            },
            None, // timeoute = None means blocking
        ).unwrap();
    }
}


