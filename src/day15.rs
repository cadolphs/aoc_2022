mod interval {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Interval(i32, i32);

    impl Interval {
        pub fn new(x: i32, y: i32) -> Self {
            if x <= y {
                Interval(x, y)
            } else {
                Interval(y, x)
            }
        }

        pub fn len(&self) -> i32 {
            self.1 - self.0
        }

        pub fn subtract(self, other: &Interval) -> Vec<Self> {
            if self.0 > other.1 || self.1 < other.0 {
                // disjoint intervals
                vec![self]
            // at this point we know self.0 <= other.1 && self.1 >= other.0
            } else if self.0 >= other.0 && self.1 <= other.1 {
                // contained interval
                vec![]
            // at this point we also know self.0 < other.0 || self.1 > other.1
            } else if self.0 < other.0 && self.1 > other.1 {
                // self contains other with no boundary overlap
                vec![Interval(self.0, other.0-1), Interval(other.1+1, self.1)]
            } else if self.0 < other.0 && self.1 <= other.1 {
                vec![Interval(self.0, other.0-1)]
            } else if self.0 >= other.0 && self.1 > other.1 {
                vec![Interval(other.1+1, self.1)]
            } else {
                panic!("Seems like I missed some interval subtraction logic");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::interval::*;

    #[test]
    fn test_interval_creation() {
        assert_eq!(Interval::new(1, 2), Interval::new(2, 1));
    }

    #[test]
    fn test_interval_subtraction() {
        let base = Interval::new(0, 10);

        let subs = vec![Interval::new(42, 55),
                                        Interval::new(2, 8),
                                        Interval::new(-2, 13),
                                        Interval::new(0, 5),
                                        Interval::new(5, 10),
                                        Interval::new(-2, 4),
                                        Interval::new(8, 12)];
        let expecteds = vec![vec![Interval::new(0, 10)],
                                            vec![Interval::new(0, 1), Interval::new(9, 10)],
                                            vec![],
                                            vec![Interval::new(6, 10)],
                                            vec![Interval::new(0, 4)],
                                            vec![Interval::new(5, 10)],
                                            vec![Interval::new(0, 7)]];

        for (sub, expected) in subs.into_iter().zip(expecteds) {
            assert_eq!(expected, base.subtract(&sub))
        }

    }
}
