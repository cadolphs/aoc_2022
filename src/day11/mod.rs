pub use self::monkey::{Monkey, MonkeyMove};

mod operation;
mod division;
mod monkey;

pub struct MonkeyGame {
    monkeys: Vec<Monkey>
}



impl MonkeyGame {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        MonkeyGame{ monkeys: monkeys }
    }

    pub fn play_round(&mut self) {
        for monkey_id in 0..self.monkeys.len() {
            let monkey_moves = self.monkeys[monkey_id].take_turn();
            for monkey_move in monkey_moves {
                self.monkeys[monkey_move.target_monkey].receive_item(monkey_move.item);
            }
        }
    }
}

#[cfg(test)]
mod tests {

}