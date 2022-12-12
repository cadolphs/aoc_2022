use std::{str::FromStr, error::Error};
use simple_error::SimpleError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Square,
    Add(u64),
    Mul(u64)
}

impl Operation {
    pub fn apply(&self, old: &u64) -> u64 {
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
        let s_trimmed = s.trim();
        if s_trimmed == "Operation: new = old * old" {
            Ok(Operation::Square)
        } else if s_trimmed.starts_with("Operation: new = old + ") {
            let num: u64 = s_trimmed[23..].parse()?;
            Ok(Operation::Add(num))
        } else if s_trimmed.starts_with("Operation: new = old * ") {
            let num: u64 = s_trimmed[23..].parse()?;
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
        let expecteds: Vec<u64> = vec![9.into(), 45.into(), 6.into()];

        let x: u64 = 3.into();
        for (op, expected) in ops.into_iter().zip(expecteds) {
            assert_eq!(op.apply(&x), expected);
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