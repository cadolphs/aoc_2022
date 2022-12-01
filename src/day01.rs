use std::collections::BinaryHeap;

pub fn run_day_01(input: String) {
    let blocks = input.trim().split("\n\n");
    let sums = blocks.map(handle_block);

    let (max_sum, next_sum, next_next_sum) = get_top_three_calories(sums);
    println!("The elf with the most calories has {} calories.", max_sum);
    println!("The top three elves have between them {} calories.", max_sum + next_sum + next_next_sum);
}

fn handle_block(block: &str) -> i32 {
    block
        .split('\n')
        .map(|num_as_str| num_as_str.parse::<i32>().unwrap())
        .sum()
}

fn get_top_three_calories<I: Iterator<Item=i32>>(sums: I) -> (i32, i32, i32) {
    let mut heap: BinaryHeap<i32> = sums.collect();
    (heap.pop().unwrap(), heap.pop().unwrap(), heap.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::BinaryHeap;

    use crate::day01::handle_block;


    #[test]
    fn hacking_around() {
        let input = "1\n\n2\n3\n\n4\n".trim().to_string();

        let mut split_on_blanks = input.split("\n\n");
        assert_eq!(3, split_on_blanks.clone().count());

        

        let sum_iterator = split_on_blanks.map(handle_block);
        assert_eq!(5, sum_iterator.max().unwrap());
    }

    #[test]
    fn hacking_around_for_part_2(){
        let input = "1\n\n2\n3\n\n4\n5\n\n6\n7".trim().to_string();

        let blocks = input.split("\n\n");
        let sum_iterator = blocks.map(handle_block);

        let mut heap: BinaryHeap<_> = sum_iterator.collect();
        // Top 2 items are 13 (6 + 7) and 9 (4 + 5).
        assert_eq!(heap.pop(), Some(13));
        assert_eq!(heap.pop(), Some(9));
    }
}
