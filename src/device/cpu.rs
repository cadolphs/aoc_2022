use std::{str::FromStr, error::Error};

use simple_error::SimpleError;
use lazy_regex::regex_captures;


#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    NOOP,
    ADDX(i32)
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::NOOP)
        }
        else if let Some((_, num_as_str)) = regex_captures!(r"^addx (-?\d+)$", s) {
            let num: i32 = num_as_str.parse().unwrap();
            Ok(Instruction::ADDX(num))
        }
        else{
            Err(Box::new(SimpleError::new("This instruction not yet implemented")))
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn read_a_command_from_line() {
        let instr: Instruction = "noop".parse().unwrap();
        assert_eq!(instr, Instruction::NOOP);

        let instr: Instruction = "addx -5".parse().unwrap();
        assert_eq!(instr, Instruction::ADDX(-5));
    }
}