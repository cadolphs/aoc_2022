#[derive(Debug, Clone, Copy)]
pub struct DivisibleTest {
    divisor: i32,
    true_monkey: usize,
    false_monkey: usize
}

impl DivisibleTest {
    pub fn new(divisor: i32, true_monkey: usize, false_monkey: usize) -> Self {
        DivisibleTest{divisor, true_monkey, false_monkey}
    }

    pub fn get_target_monkey_for(&self, x: i32) -> usize {
        if x % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
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
}