use lazy_regex::regex;
use simple_error::SimpleError;
use std::error::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
    num: u64,
    from: u64,
    to: u64,
}

fn parse_move_line(line: &str) -> Result<Move, Box<dyn Error>> {
    let re = regex!(r"^move (\d+) from (\d+) to (\d)$");
    let caps = re.captures(line).ok_or(Box::new(SimpleError::new(
        "Line doesn't match expectation for move",
    )))?;

    let num: u64 = caps.get(1).unwrap().as_str().parse().unwrap(); // If the expreccsion matched, then it should work here without panic
    let from: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
    let to: u64 = caps.get(3).unwrap().as_str().parse().unwrap();

    Ok(Move { num, from, to })
}

fn parse_crate_line(line: &str) -> Vec<(u64, char)> {
    let mut stack_crate = Vec::new();
    let num_stacks: u64 = (line.len() as u64 + 1) / 4;
    let chars: Vec<char> = line.chars().collect();

    for stack in (1..num_stacks + 1) {
        let stack_pos = (stack - 1) * 4 + 1;
        let char_at = chars[stack_pos as usize];
        match char_at {
            'A'..='Z' => { stack_crate.push((stack, char_at));}
            _ => ()
        }
    }

    stack_crate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hacking_aoround_with_parsing() {
        // First, let us figure out how many stacks there are in a line
        let test_input = "
[D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

        let max_len = test_input.lines().map(|l| l.len()).max().unwrap();
        // max_len = 3 * num_stacks + (num_stacks - 1) = 4 * num_stacks - 1
        // num_stacks = (max_len + 1) / 4
        let num_stacks = (max_len + 1) / 4;
        assert_eq!(num_stacks, 3);
    }

    #[test]
    fn test_parsing_move_line() {
        let line = "move 11 from 8 to 3";
        let my_move = parse_move_line(line).unwrap();
        assert_eq!(
            my_move,
            Move {
                num: 11,
                to: 3,
                from: 8
            }
        );
    }

    #[test]
    fn test_parse_crate_line() {
        let line = "[M]     [D]";
        let result = parse_crate_line(line);
        let expected = vec![(1, 'M'), (3, 'D')];

        assert_eq!(result, expected);
    }
}
