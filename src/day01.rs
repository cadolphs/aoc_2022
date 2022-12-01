pub fn run_day_01(input: String) {
    let blocks = input.trim().split("\n\n");
    let sums = blocks.map(handle_block);

    let max_sum = sums.max().unwrap();
    println!("The elf with the most calories has {} calories.", max_sum);
}

fn handle_block(block: &str) -> i32 {
    block
        .split('\n')
        .map(|num_as_str| num_as_str.parse::<i32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day01::handle_block;


    #[test]
    fn hacking_around() {
        let input = "1\n\n2\n3\n\n4\n".trim().to_string();

        let mut split_on_blanks = input.split("\n\n");
        assert_eq!(3, split_on_blanks.clone().count());

        

        let sum_iterator = split_on_blanks.map(handle_block);
        assert_eq!(5, sum_iterator.max().unwrap());
    }
}
