use midir::MidiInput;
use std::io;
use std::error::Error;

fn main() {
    //play_white_noise();    
    start_midi().unwrap();
    
}

fn start_midi() -> Result<(), Box<dyn Error>> {
    let midi_input = MidiInput::new("midi_connections").unwrap();
    let port_number = choose_midi_input(&midi_input).unwrap();
    println!("Chosen MIDI port is {}", port_number);
    let ports_list = midi_input.ports();
    let midi_input_port = ports_list.get(port_number).unwrap();
    midi_input.connect(midi_input_port,
        "midi_input", 
        move | stamp, message, _| {
            println!("MSG");
            println!("MSG IN: {}, {:?} | Len = {}", stamp, message, message.len());
        }, ())?;
    // let mut buf = String::new();
    // io::stdin().read_line(&mut buf).expect("Failed to read");
    std::thread::sleep(std::time::Duration::from_secs(10));
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


