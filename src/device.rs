pub fn find_start_of_packet_marker<'a, S: IntoIterator<Item=char>>(stream: S) -> Option<usize> {
    let mut state = State::Empty;
    let mut pos = 0;
    let mut chars = stream.into_iter();

    while ! state.match_found() {
        let c = chars.next();
        if let Some(c) = c {
            state = state.advance(c);
            pos += 1;
        } else {
            return None;
        }
    }
    Some(pos)
}

enum State {
    Empty,
    One(char),
    Two(char, char),
    Three(char, char, char),
    Four(char, char, char, char),
}

impl State {
    fn advance(self, c: char) -> State {
        use State::*;
        match self {
            Empty => One(c),
            One(c1) => {
                if c == c1 {
                    One(c)
                } else {
                    Two(c1, c)
                }
            }
            Two(c1, c2) => {
                if c == c2 {
                    One(c)
                } else if c == c1 {
                    Two(c2, c)
                } else {
                    Three(c1, c2, c)
                }
            }
            Three(c1, c2, c3) => {
                if c == c3 {
                    One(c)
                } else if c == c2 {
                    Two(c3, c)
                } else if c == c1 {
                    Three(c2, c3, c)
                } else {
                    Four(c1, c2, c3, c)
                }
            }
            Four(_c1, _c2, _c3, _c4) => {
                panic!("No need to continue here")
            }
        }
    }

    fn match_found(&self) -> bool {
        match self {
            State::Four(_,_,_,_) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hacking_around_with_matching() {
        let line = "abcabcabcdefg";
        let expected: usize = 10;

        

        let mut state = State::Empty;
        let mut chars = line.chars();
        let mut pos = 0;
        while ! state.match_found() {
            state = state.advance(chars.next().unwrap());
            pos += 1;
        }
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_find_packet_marker() {
        let line = "aabbabccabacbd";
        let expected: usize = line.len();

        let pos = find_start_of_packet_marker(line.chars());
        assert_eq!(Some(expected), pos);
    }
}
