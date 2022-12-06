pub fn find_start_of_packet_marker<'a, S: IntoIterator<Item = char>>(stream: S) -> Option<usize> {
    find_start_of_marker(stream, 4)
}

pub fn find_start_of_message_marker<'a, S: IntoIterator<Item = char>>(stream: S) -> Option<usize> {
    find_start_of_marker(stream, 14)
}

pub fn find_start_of_marker<'a, S: IntoIterator<Item = char>>(
    stream: S,
    size: usize,
) -> Option<usize> {
    let mut start = 0;
    let mut letters: Vec<char> = Vec::new();

    let mut chars = stream.into_iter();
    while letters.len() != size {
        let next_letter = chars.next();
        if next_letter.is_none() {
            return None;
        }
        let next_letter = next_letter.unwrap();

        if !letters.contains(&next_letter) {
            letters.push(next_letter);
        } else {
            let most_recent_pos = letters.iter().rposition(|&c| c == next_letter).unwrap();
            // Need to drop vector elements up until that
            letters = letters[most_recent_pos + 1..].to_vec();
            letters.push(next_letter);
            start = start + most_recent_pos + 1;
        }
    }
    // We're here, so now we have `size` letters, beginnig with start. So the pos should be start + size
    let pos = start + size;
    Some(pos)
}

#[cfg(test)]
mod tests {
    use super::*;
 

    #[test]
    fn test_find_packet_marker() {
        let line = "aabbabccabacbd";
        let expected: usize = line.len();

        let pos = find_start_of_marker(line.chars(), 4);
        assert_eq!(Some(expected), pos);
    }

    #[test]
    fn scanline_hacking() {
        let line = "aabbabccabacbdfgefg";
        let expected = 14;
        let size = 4;

        let mut start = 0;
        let mut letters: Vec<char> = Vec::new();

        let mut chars = line.chars();
        while letters.len() != size {
            let next_letter = chars.next().unwrap();
            if !letters.contains(&next_letter) {
                letters.push(next_letter);
            } else {
                let most_recent_pos = letters.iter().rposition(|&c| c == next_letter).unwrap();
                // Need to drop vector elements up until that
                letters = letters[most_recent_pos + 1..].to_vec();
                letters.push(next_letter);
                start = start + most_recent_pos + 1;
            }
        }
        // We're here, so now we have `size` letters, beginnig with start. So the pos should be start + size + 1
        let pos = start + size;
        assert_eq!(expected, pos);
    }
}
