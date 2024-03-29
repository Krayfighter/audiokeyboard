
use cpal::traits::{
    HostTrait,
    DeviceTrait
};


pub struct AudioState {
    sample_rate: u64,
    current_x_value: u64,
    transform: fn(f32, f32, f32) -> f32
}

impl AudioState {

    #[inline]
    fn increment_sample(&mut self) {
        self.current_x_value += 1;
        self.current_x_value %= self.sample_rate*10;
    }

    pub fn make_stream(
        keystates: crate::state::KeyboardState,
        transform_ref: crate::state::SyncFunctionPtr,
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
            sample_rate: output_config.sample_rate().0 as u64,
            current_x_value: 0,
            transform: crate::voice_algorithms::sine_function
        };
    
        return device.build_output_stream(
            &output_config.config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                if let Some(fnct_ptr) = transform_ref.get() {
                    state.transform = fnct_ptr;
                }
                for value in data.iter_mut() {
                    state.increment_sample();
                    let ksc = keystates.cloned();
                    let length = ksc.len();
                    *value = ksc
                        .iter()
                        .map(|(_, freq)| {
                            return ( state.transform ) (
                                state.current_x_value as f32,
                                *freq,
                                state.sample_rate as f32,
                            );
                        })
                        .map(|num| num/(length+1) as f32)
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


