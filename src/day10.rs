use crate::device::{CPUtracker, Instruction};

pub fn run_day_10(input: String) {
    let mut tracker = CPUtracker::new();

    for instruction in input.lines().map(|line| line.parse().unwrap()) {
        tracker.execute(instruction);
    }

    // This is simplest way
    let ans: i32 = (1..).zip(tracker.all_xs())
        .skip(19)
        .step_by(40)
        .take(6).map(|(i, signal)| i * signal).sum();
    
    println!("Multiplying stuff like in part 1 gives answer {}", ans);
}