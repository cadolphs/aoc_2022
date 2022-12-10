use std::{str::FromStr, error::Error};

use simple_error::SimpleError;
use lazy_regex::regex_captures;

pub struct CPUtracker {
    xs: Vec<i32>
}

impl CPUtracker {
    pub fn new() -> Self {
        CPUtracker { xs: vec![1] }
    }

    pub fn x(&self) -> i32 {
        *self.xs.last().unwrap()
    }

    fn tick(&mut self, new_x: i32) {
        self.xs.push(new_x);
    }

    pub fn execute(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            NOOP => self.tick(self.x()),
            ADDX(delta) => {
                let x = self.x();
                self.tick(x);
                self.tick(x + delta)
            }
        }
    }

    pub fn all_xs(&self) -> impl Iterator<Item=&i32> {
        self.xs.iter()
    }

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
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

    #[test]
    fn test_cpu_tracker() {
        let mut tracker = CPUtracker::new();
        assert_eq!(1, tracker.x());

        tracker.execute(Instruction::ADDX(-5));
        tracker.execute(Instruction::NOOP);
        tracker.execute(Instruction::ADDX(2));

        let expected = vec![1, 1, -4, -4, -4, -2];
        assert_eq!(expected, tracker.all_xs().cloned().collect::<Vec<i32>>())
    }
}