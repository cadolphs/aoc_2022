#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Square,
    Add(i32),
    Mul(i32)
}

impl Operation {
    pub fn apply(&self, old: i32) -> i32 {
        use Operation::*;
        match &self {
            Square => old * old,
            Add(x) => old + x,
            Mul(x) => old * x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_square() {
        let ops = vec![Operation::Square, Operation::Add(42), Operation::Mul(2)];
        let expecteds = vec![9, 45, 6];

        let x = 3;
        for (op, expected) in ops.into_iter().zip(expecteds) {
            assert_eq!(op.apply(x), expected);
        }
    }
}