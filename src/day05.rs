use lazy_regex::regex;
use simple_error::SimpleError;
use std::error::Error;

use self::crate_stacks::CrateStacks;

pub fn run_day_05(input: String) {
    let mut blocks = input.split("\n\n");
    let crate_block = blocks.next().unwrap();
    let move_block = blocks.next().unwrap();

    let mut crate_stacks = parse_crate_block(crate_block);

    for move_line in move_block.lines() {
        let m = parse_move_line(move_line).unwrap();
        crate_stacks.apply_move(m)
    }

    let top_items: String = crate_stacks.top_items().collect();

    println!("After all this moving, the top items on the crates are {}", top_items);
}

fn parse_crate_block(crate_block: &str) -> CrateStacks {
    //First, figure out number of crate blocks:
    let num_stacks: usize = (crate_block.lines().next().unwrap().len() + 1) / 4;
    let mut crate_stacks = CrateStacks::new(num_stacks);

    for line in crate_block.lines() {
        let stack_crate_vec = parse_crate_line(line);
        for (stack, crate_id) in stack_crate_vec {
            crate_stacks.append_stack(stack, crate_id);
        }
    }

    crate_stacks
}

mod crate_stacks {
    use std::collections::VecDeque;

    use super::Move;

    pub struct CrateStacks {
        stacks: Vec<VecDeque<char>>,
    }

    impl CrateStacks {
        pub fn new(num_stacks: usize) -> CrateStacks {
            CrateStacks {
                stacks: vec![VecDeque::new(); num_stacks],
            }
        }

        pub fn num_stacks(&self) -> usize {
            self.stacks.len()
        }

        pub fn append_stack(&mut self, stack: usize, crate_id: char) {
            self.stacks[stack-1].push_front(crate_id);
        }

        pub fn apply_move(&mut self, m: Move) {
            for _ in 0..m.num {
                let removed_item = self.stacks[m.from-1].pop_back().unwrap();
                self.stacks[m.to-1].push_back(removed_item);
            }
        }

        pub fn top_items(&self) -> impl Iterator<Item = &char> {
            self.stacks
                .iter()
                .map(|dequeue| dequeue.back().unwrap_or(&' '))
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_move_line(line: &str) -> Result<Move, Box<dyn Error>> {
    let re = regex!(r"^move (\d+) from (\d+) to (\d)$");
    let caps = re.captures(line).ok_or(Box::new(SimpleError::new(
        "Line doesn't match expectation for move",
    )))?;

    let num: usize = caps.get(1).unwrap().as_str().parse().unwrap(); // If the expreccsion matched, then it should work here without panic
    let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let to: usize = caps.get(3).unwrap().as_str().parse().unwrap();

    Ok(Move { num, from, to })
}

fn parse_crate_line(line: &str) -> Vec<(usize, char)> {
    let mut stack_crate = Vec::new();
    let num_stacks: usize = (line.len() + 1) / 4;
    let chars: Vec<char> = line.chars().collect();

    for stack in (1..num_stacks + 1) {
        let stack_pos = (stack - 1) * 4 + 1;
        let char_at = chars[stack_pos as usize];
        match char_at {
            'A'..='Z' => {
                stack_crate.push((stack, char_at));
            }
            _ => (),
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

    #[test]
    fn test_crate_stack() {
        use crate_stacks::CrateStacks;
        let mut stack = CrateStacks::new(3);
        let top_items: String = stack.top_items().collect();
        assert_eq!(top_items, "   ".to_string());

        stack.append_stack(1, 'D');
        let top_items: String = stack.top_items().collect();
        assert_eq!(top_items, "D  ".to_string());

        let m = Move{num: 1, from: 1, to: 3};
        stack.apply_move(m);
        let top_items: String = stack.top_items().collect();
        assert_eq!(top_items, "  D".to_string());
    }

    #[test]
    fn test_parse_crate_block() {
        let crate_stack = parse_crate_block("[A] [B]    \n[C] [D] [E]");
        let top_items: String = crate_stack.top_items().collect();
        assert_eq!(top_items, "ABE".to_string());
    }
}
