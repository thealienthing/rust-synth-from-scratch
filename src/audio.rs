use cpal::traits::{StreamTrait, DeviceTrait};
use cpal::{Sample, SampleFormat, Host, StreamConfig};
use rand::Rng;

pub struct AudioStream {
    audio_stream: cpal::Stream,
    device: cpal::Device,
    host: Host,
    sample_format: SampleFormat,
    stream_config: StreamConfig,
}

impl AudioStream {
    pub fn new(device: cpal::Device, host: Host) -> AudioStream {
        let mut device_supported_configs = device.supported_output_configs()
        .expect("Failed to get devices supported output configs");
        let supported_config = device_supported_configs.next()
            .expect("No supported config found")
            .with_max_sample_rate();

        let sample_format = supported_config.sample_format();
        let config = supported_config.into();
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let stream = match sample_format {
            SampleFormat::F32 => device.build_output_stream(
                &config,
                move |_data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // react to stream events and read or write stream data here.


                },
                err_fn),
            SampleFormat::I16 => device.build_output_stream(&config, output_callback::<i16>, err_fn),
            SampleFormat::U16 => device.build_output_stream(&config, output_callback::<u16>, err_fn)
        }.unwrap();

        AudioStream { audio_stream: stream, device, host, sample_format, stream_config: config }

    }

    pub fn play(&self) {
        self.audio_stream.play().unwrap();
    }

    pub fn get_sample_format(&self) -> SampleFormat {
        self.sample_format.clone()
    }
}

fn output_callback<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&get_attenuated());
    }
}

fn get_attenuated() -> f32 {
    get_random() * 0.2
}

fn get_random() -> f32 {
    let mut rng = rand::thread_rng();
    let y: f32 = rng.gen();
    return y
}