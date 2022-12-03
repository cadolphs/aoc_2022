use itertools::Itertools;
use simple_error::SimpleError;
use std::collections::HashSet;

pub fn run_day_03(input: String) {
    let total_score: u64 = input
        .lines()
        .map(|line| find_item_that_appears_in_both(line).unwrap())
        .map(|item| score_item(item).unwrap())
        .sum();

    println!("Total score of the mixed-up items is {}", total_score);

    let total_score_2: u64 = input
        .lines()
        .tuples()
        .map(|(a, b, c)| [a, b, c])
        .map(|rucksacks| find_shared_item_in_three_group(&rucksacks).unwrap())
        .map(|item| score_item(item).unwrap())
        .sum();

    println!("Total score of the bages is {}", total_score_2);
}

fn find_item_that_appears_in_both(input: &str) -> Result<char, SimpleError> {
    if input.len() % 2 != 0 {
        return Err(SimpleError::new("Uneven length for input string!"));
    }

    let compartments = input.split_at(input.len() / 2);
    let first_items: HashSet<char> = compartments.0.chars().collect();
    let second_items: HashSet<char> = compartments.1.chars().collect();

    let shared_items: Vec<char> = first_items.intersection(&second_items).cloned().collect();

    if shared_items.len() != 1 {
        return Err(SimpleError::new(format!(
            "The two backpacks share more than one item: {:?}",
            shared_items
        )));
    } else {
        return Ok(shared_items[0]);
    }
}

fn find_shared_item_in_three_group(rucksacks: &[&str; 3]) -> Result<char, SimpleError> {
    let item_sets: [HashSet<char>; 3] = rucksacks.map(|rucksack| rucksack.chars().collect());
    let intersection = item_sets
        .into_iter()
        .reduce(|acc, it| acc.intersection(&it).cloned().collect())
        .unwrap();

    if intersection.len() != 1 {
        Err(SimpleError::new(
            "Three-group has intersection with not exactly one element",
        ))
    } else {
        Ok(intersection.into_iter().next().unwrap())
    }
}

fn score_item(item: char) -> Result<u64, SimpleError> {
    if !item.is_alphabetic() {
        return Err(SimpleError::new("Not an alphabetic character"));
    }

    let ascii = item as u64;
    if ascii >= ('a' as u64) && ascii <= ('z' as u64) {
        Ok(ascii - ('a' as u64) + 1)
    } else if ascii >= ('A' as u64) && ascii <= ('Z' as u64) {
        Ok(ascii - ('A' as u64) + 27)
    } else {
        Err(SimpleError::new("Invalid character encountered"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_items_that_apper_in_both() {
        let input = "aa";

        assert_eq!(find_item_that_appears_in_both(input), Ok('a'));

        let input = "aaabccbd";
        assert_eq!(find_item_that_appears_in_both(input), Ok('b'));

        let input = "xyz";
        assert!(find_item_that_appears_in_both(input).is_err());
    }

    #[test]
    fn test_some_item_scores() {
        let input_output = vec![('a', 1), ('z', 26), ('A', 27), ('Z', 52)];
        for (input, expected) in input_output {
            assert_eq!(Ok(expected), score_item(input))
        }
    }

    #[test]
    fn test_the_three_finder() {
        let input: [&str; 3] = ["abc", "acd", "adf"];
        assert_eq!(find_shared_item_in_three_group(&input), Ok('a'));
    }
}
