use std::{str::FromStr, error::Error};

pub use self::monkey::{Monkey, MonkeyMove};

mod operation;
mod division;
mod monkey;

pub fn run_day_11(input: String) {
    let mut game: MonkeyGame = input.clone().parse().unwrap();

    for _ in 0..20 {
        game.play_round();
    }

    // for the small num of monkeys, sorting isn't the worst
    let mut monkey_business = game.get_monkey_business();
    monkey_business.sort();

    let ans = monkey_business[monkey_business.len() - 1] * monkey_business[monkey_business.len() - 2];
    println!("The current monkey business score is {}", ans);

    // new game
    let mut game: MonkeyGame = input.clone().parse().unwrap();
    game.set_divisor(1);

    for _ in 0..10000 {
        game.play_round();
    }

    let mut monkey_business = game.get_monkey_business();
    monkey_business.sort();

    let ans = monkey_business[monkey_business.len() - 1] * monkey_business[monkey_business.len() - 2];
    println!("The current monkey business score is {}", ans);
}

pub struct MonkeyGame {
    monkeys: Vec<Monkey>,
    monkey_item_counter: Vec<usize>,
    lcm: u64,
}


impl MonkeyGame {
    pub fn new(monkeys: Vec<Monkey>, lcm: u64) -> Self {
        let counter = vec![0usize; monkeys.len()];

        MonkeyGame{ monkeys: monkeys, monkey_item_counter: counter, lcm: lcm }
    }

    pub fn play_round(&mut self) {
        for monkey_id in 0..self.monkeys.len() {
            let monkey_moves = self.monkeys[monkey_id].take_turn();
            self.monkey_item_counter[monkey_id] += monkey_moves.len();
            for monkey_move in monkey_moves {
                self.monkeys[monkey_move.target_monkey].receive_item(monkey_move.item % self.lcm);
            }
        }
    }

    pub fn get_monkey_business(&self) -> Vec<usize> {
        self.monkey_item_counter.clone()
    }

    pub fn set_divisor(&mut self, divisor: u64) {
        for monkey in &mut self.monkeys {
            monkey.set_divisor(divisor);
        }
    }
}

impl FromStr for MonkeyGame {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = s.split("\n\n");

        // let monkeys= blocks.map(|block| block.parse()).collect::<Result<_,_>>()?;
        let mut monkeys: Vec<Monkey> = Vec::new();

        for block in blocks {
            monkeys.push(block.parse()?);
        }

        // not strictly the LCM but _should_ do the trick
    
        let lcm = monkeys.iter().map(|monkey| monkey.get_prime_test()).reduce(|acc, x| num::integer::lcm(acc, x)).unwrap();
        Ok(MonkeyGame::new(monkeys, lcm))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parsing() {
        let input = indoc!("
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
");

    let monkeygame: MonkeyGame = input.parse().unwrap();

    assert_eq!(monkeygame.monkeys.len(), 4);
    }
}