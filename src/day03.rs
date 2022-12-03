use std::collections::HashSet;

use simple_error::SimpleError;

pub fn run_day_03(input: String) {
    todo!()
}

fn find_item_that_appears_in_both(input: String) -> Result<char, SimpleError> {
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

fn score_item(item: char) -> Result<u64, SimpleError> { 
    if !item.is_alphabetic() {
        return Err(SimpleError::new("Not an alphabetic character"));
    }

    let ascii = item as u64;
    if ascii >= ('a' as u64) && ascii <= ('z' as u64) {
        Ok(ascii - ('a' as u64) + 1)
    }
    else if ascii >= ('A' as u64) && ascii <= ('Z' as u64) {
        Ok(ascii - ('A' as u64) + 27)
    }
    else {
        Err(SimpleError::new("Invalid character encountered"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_items_that_apper_in_both() {
        let input = "aa".to_string();

        assert_eq!(find_item_that_appears_in_both(input), Ok('a'));

        let input: String = "aaabccbd".to_string();
        assert_eq!(find_item_that_appears_in_both(input), Ok('b'));

        let input = "xyz".to_string();
        assert!(find_item_that_appears_in_both(input).is_err());
    }

    #[test]
    fn test_some_item_scores() {
        let input_output = vec![('a', 1), ('z', 26), ('A', 27), ('Z', 52)];
        for (input, expected) in input_output {
            assert_eq!(Ok(expected), score_item(input))
        }
    }
}