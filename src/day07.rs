use crate::device::parse_terminal_output_for_dir_sizes;

pub fn run_day_07(input: String) {

    let dir_sizes = parse_terminal_output_for_dir_sizes(&input);

    let total_sum: u64 = dir_sizes.values().filter(|size| **size <= 100000).sum();

    println!("Total sum of directories at most 100000 is {}", total_sum);
}
