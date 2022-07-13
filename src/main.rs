
use std::io;

mod midi;

fn main() {
    //play_white_noise();    
    let _connection = midi::start_midi().unwrap();

    println!("Press enter to close");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // wait for next enter key press
}