use std::{str::FromStr, error::Error};

use simple_error::SimpleError;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Operation: new = old * old" {
            Ok(Operation::Square)
        } else if s.starts_with("Operation: new = old + ") {
            let num: i32 = s[23..].parse()?;
            Ok(Operation::Add(num))
        } else if s.starts_with("Operation: new = old * ") {
            let num: i32 = s[23..].parse()?;
            Ok(Operation::Mul(num))
        } else {
            Err(Box::new(SimpleError::new("Not a valid operation")))
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

    #[test]
    fn test_parsing() {
        let result: Operation = "Operation: new = old * old".parse().unwrap();
        assert_eq!(Operation::Square, result);

        let result: Operation = "Operation: new = old + 42".parse().unwrap();
        assert_eq!(Operation::Add(42), result);

        let result: Operation = "Operation: new = old * 55".parse().unwrap();
        assert_eq!(Operation::Mul(55), result);
    }
}