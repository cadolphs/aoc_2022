use crate::day11::division::DivisibleTest;
use crate::day11::operation::Operation;

pub struct Monkey {
    test: DivisibleTest,
    op: Operation,

    items: Vec<i32>,
    num_inspected: usize,
}

impl Monkey {
    pub fn new(items: Vec<i32>, op: Operation, test: DivisibleTest) -> Self {
        Monkey{items, op, test, num_inspected: 0}
    }

    pub fn take_turn(&mut self) -> Vec<MonkeyMove> {
        self.num_inspected += self.items.len();

        self.items.drain(..).map(|item| {
            let x = self.op.apply(item) / 3;
            let target_monkey = self.test.get_target_monkey_for(x);
            MonkeyMove{item: x, target_monkey}
        }).collect()
    }

    pub fn receive_item(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn num_inspected(&self) -> usize {
        self.num_inspected
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonkeyMove {
    pub item: i32,
    pub target_monkey: usize
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(1, monkey.num_inspected());
    }
}