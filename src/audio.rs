use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::{Sample, SampleFormat};

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

fn play_white_noise() {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .expect("Failed to get default output device");
    let mut device_supported_configs = device.supported_output_configs()
        .expect("Failed to get devices supported output configs");
    let supported_config = device_supported_configs.next()
        .expect("No supported config found")
        .with_max_sample_rate();

    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, output_callback::<f32>, err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, output_callback::<i16>, err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, output_callback::<u16>, err_fn)
    }.unwrap();

    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(4000));
}