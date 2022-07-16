use std::io;
use cpal::traits::{HostTrait};
mod audio;
mod midi;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .expect("Failed to get default output device");
    
    let _connection = midi::start_midi().unwrap();
    let audio_stream = audio::AudioStream::new(device, host);


    audio_stream.play();

    println!("Press enter to close");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // wait for next enter key press
}