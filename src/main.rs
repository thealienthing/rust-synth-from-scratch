use midir::{MidiInput, Ignore};
use std::io;
use std::error::Error;

fn main() {
    //play_white_noise();    
    start_midi().unwrap();
    
}

fn start_midi() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut midi_input = MidiInput::new("midi_connections").unwrap();
    midi_input.ignore(Ignore::None);
    let port_number = choose_midi_input(&midi_input).unwrap();
    println!("Chosen MIDI port is {}", port_number);
    let ports_list = midi_input.ports();
    let midi_input_port = ports_list.get(port_number).unwrap();
    let _connection = midi_input.connect(midi_input_port,
        "midir-read-input", 
        move | stamp, message, _| {
            println!("MSG IN: {}, {:?} | Len = {}", stamp, message, message.len());
        }, ())?;
    
    io::stdin().read_line(&mut input)?; // wait for next enter key press
    Ok(())
}

fn choose_midi_input(midi_input: &MidiInput) -> Result<usize, io::Error> {
    println!("Available MIDI Input Ports:");
    println!("===========================\n");
    let ports = midi_input.ports();
    for p in 0..ports.len() {
        let port = ports.get(p).unwrap();
        let name = midi_input.port_name(port).unwrap();
        println!("Port {}: {}", p, name);

    }
    print!("Select Port > ");
    let mut midi_port = String::new();

    std::io::stdin()
        .read_line(&mut midi_port)
        .expect("Failed to read input");

    let midi_port = midi_port.trim()
        .parse::<usize>()
        .expect("Must be a positive number");
    
    if midi_port > ports.len() {
        panic!("Chosen midi port out of bounds");
    }
    Ok(midi_port)
}


