use crate::device::parse_terminal_output_for_dir_sizes;

pub fn run_day_07(input: String) {

    let dir_sizes = parse_terminal_output_for_dir_sizes(&input);

    let total_sum: u64 = dir_sizes.values().filter(|size| **size <= 100000).sum();

    println!("Total sum of directories at most 100000 is {}", total_sum);

    const TOTAL_SPACE: u64 =  70000000;
    const SPACE_NEEDED: u64 = 30000000;

    let currently_occupied: u64  = *dir_sizes.get("/").unwrap();
    let unused_space = TOTAL_SPACE - currently_occupied;
    
    let need_to_free_at_least = SPACE_NEEDED - unused_space; 

    let dir_to_delete = dir_sizes.values().filter(|size| **size >= need_to_free_at_least).min().unwrap();
    println!("Size of the dir that we should delete is {}", dir_to_delete);
}
