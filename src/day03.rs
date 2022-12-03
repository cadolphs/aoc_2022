use std::collections::HashSet;

use simple_error::SimpleError;

pub fn run_day_03(input: String) {
    todo!()
}

fn find_item_that_appears_in_both(input: String) -> Result<char, SimpleError> {
    let compartments = input.split_at(input.len() / 2);
    let first_items: HashSet<char> = compartments.0.chars().collect();
    let second_items: HashSet<char> = compartments.1.chars().collect();

    let shared_items: Vec<char> = first_items.intersection(&second_items).cloned().collect();
    
    if shared_items.len() != 1 {
        return Err(SimpleError::new(format!("The two backpacks share more than one item: {:?}", shared_items)));
    }
    else {
        return Ok(shared_items[0]);
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
}
