use std::io;
use cpal::traits::StreamTrait;
mod audio;
mod midi;

fn main() {     
    let _connection = midi::start_midi().unwrap();
    let audio_stream = audio::audio_stream(); 
    audio_stream.play().unwrap();
    println!("Press enter to close");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // wait for next enter key press
}