pub fn run_day_02(input: String) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_hand_shape_from_character() {
        let character = 'A';
        let hand_shape: HandShape = character.try_into();
    }
}