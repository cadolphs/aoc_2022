use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

use crate::day11::division::DivisibleTest;
use crate::day11::operation::Operation;
use lazy_regex::regex;


pub struct Monkey {
    test: DivisibleTest,
    op: Operation,

    items: Vec<i32>,
    divisor: i32,
}

impl Monkey {
    pub fn new(items: Vec<i32>, op: Operation, test: DivisibleTest) -> Self {
        Monkey{items, op, test, divisor: 3}
    }

    pub fn take_turn(&mut self) -> Vec<MonkeyMove> {
        self.items.drain(..).map(|item| {
            let x = self.op.apply(&item) / self.divisor;
            let x = self.test.get_reduced_x(x);
            let target_monkey = self.test.get_target_monkey_for(x);
            MonkeyMove{item: x, target_monkey}
        }).collect()
    }

    pub fn receive_item(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn set_divisor(&mut self, divisor: i32) {
        self.divisor = divisor;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MonkeyMove {
    pub item: i32,
    pub target_monkey: usize
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().skip(1).collect_vec();
        
        let start_items = parse_start_items(lines[0])?;
        let operation: Operation = lines[1].parse()?;
        
        let test: DivisibleTest = DivisibleTest::from_lines(lines[2..5].try_into()?);

        Ok(Self::new(start_items, operation, test))
    }
}

fn parse_start_items(line: &str) -> Result<Vec<i32>, ParseIntError> {
    let re = regex!(r"\d+");
    let matches = re.find_iter(line);

    matches.map(|mat| mat.as_str().parse::<i32>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_monkey_turn_with_single_item() {
        let op = Operation::Square;
        let test = DivisibleTest::new(3, 42, 55);
        let items = vec![3];

        let mut monkey = Monkey::new(items, op, test);

        let result = monkey.take_turn();
        assert_eq!(monkey.items.len(), 0);
        
        let expected = MonkeyMove{item: 3, target_monkey:42};
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_parsing() {
        let input = indoc!("
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
        Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3
        "
        );

        let monkey: Monkey = input.parse().unwrap();

        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.op, Operation::Mul(19));
        assert_eq!(monkey.test, DivisibleTest::new(23, 2, 3));
    }
}