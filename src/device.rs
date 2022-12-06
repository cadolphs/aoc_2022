#[cfg(test)]
mod tests {
    #[test]
    fn hacking_around_with_matching() {
        let line = "abcabcabcdefg";
        let expected: usize = 10;

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
                            Two(c, c1)
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
                    Four(c1, c2, c3, c4) => {
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

        let mut state = State::Empty;
        let mut chars = line.chars();
        let mut pos = 0;
        while ! state.match_found() {
            state = state.advance(chars.next().unwrap());
            pos += 1;
        }
        assert_eq!(pos, expected);
    }
}
