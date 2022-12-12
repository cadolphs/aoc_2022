use lazy_regex::regex_captures;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DivisibleTest {
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize
}

impl DivisibleTest {
    pub fn new(divisor: u64, true_monkey: usize, false_monkey: usize) -> Self {
        DivisibleTest{divisor, true_monkey, false_monkey}
    }

    pub fn get_target_monkey_for(&self, x: u64) -> usize {
        if x % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }

    pub fn get_divisor(&self) -> u64 {
        self.divisor
    }

    pub fn from_lines(lines: &[&str; 3]) -> Self {
        let (test_line, true_line, false_line) = (lines[0], lines[1], lines[2]);

        let divisor: u64 = regex_captures!(r"Test: divisible by (\d+)", test_line).unwrap().1.parse().unwrap();
        let true_monkey: usize = regex_captures!(r"If true: throw to monkey (\d+)", true_line).unwrap().1.parse().unwrap();
        let false_monkey: usize = regex_captures!(r"If false: throw to monkey (\d+)", false_line).unwrap().1.parse().unwrap();

        DivisibleTest { divisor, true_monkey, false_monkey }
    }
}

#[cfg(test)]
mod tests {
    use super::DivisibleTest;

    #[test]
    fn test_monkey_divisor_test() {
        let checker = DivisibleTest::new(11, 42, 55);

        assert_eq!(42, checker.get_target_monkey_for(5*11));
        for i in 1..=10 {
            assert_eq!(55, checker.get_target_monkey_for(14*11 + i));
        }
    }

    #[test]
    fn test_from_lines() {
        let lines = ["  Test: divisible by 23", "    If true: throw to monkey 2", "     If false: throw to monkey 3"];
        let div_test = DivisibleTest::from_lines(&lines);

        assert_eq!(2, div_test.get_target_monkey_for(46));
        assert_eq!(3, div_test.get_target_monkey_for(47));
    }
}