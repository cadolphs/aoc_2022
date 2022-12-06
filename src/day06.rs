use crate::device::{find_start_of_packet_marker, find_start_of_message_marker};

pub fn run_day_06(input: String) {
    let pos = find_start_of_packet_marker(input.chars());
    if pos.is_none() {
        panic!{"Couldn't find any start of packet marker, yo! 🚨"}
    }
    let pos = pos.unwrap();
    println!("The start-of-packet marker is at position {}", pos);

    let pos = find_start_of_message_marker(input.chars());
    if pos.is_none() {
        panic!{"Couldn't find any start of message marker, yo! 🚨"}
    }
    let pos = pos.unwrap();
    println!("The start-of-message marker is at position {}", pos);

}