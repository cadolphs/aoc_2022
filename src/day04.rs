use std::error::Error;

use range::Range;
use simple_error::SimpleError;

pub fn run_day_04(input: String) {
    let ranges_iter = input.lines().map(|line| parse_line(line).unwrap());

    let count = ranges_iter.map(|ranges| one_contains_other(ranges))
        .filter(|contains| *contains == true)
        .count();

    println!("There's {} pairs of elves whose areas are contained within each other.", count);

    let ranges_iter = input.lines().map(|line| parse_line(line).unwrap());
    let count_2 = ranges_iter.map(|ranges| ranges.0.overlaps(&ranges.1))
    .filter(|overlaps| *overlaps == true)
    .count();

    println!("There's {} pairs of elves whose areas overlap at all.", count_2);
}

fn one_contains_other(ranges: (Range, Range)) -> bool {
    ranges.0.contains(&ranges.1) || ranges.1.contains(&ranges.0)
}

fn parse_line(line: &str) -> Result<(Range, Range), Box<dyn Error>> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 2 {
        return Err(Box::new(SimpleError::new(
            "Invalid input; doesn't have two comma-separated parts",
        )));
    }

    let ranges: Vec<Range> = parts
        .into_iter()
        .map(|part| part.parse())
        .collect::<Result<Vec<Range>, _>>()?;

    Ok((ranges[0], ranges[1]))
}

mod range {
    use std::str::FromStr;

    use simple_error::SimpleError;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Range {
        low: u64,
        high: u64,
    }

    impl Range {
        pub fn new(low: u64, high: u64) -> Result<Range, SimpleError> {
            if low <= high {
                Ok(Range { low, high })
            } else {
                Err(SimpleError::new("low must be less than or equal high"))
            }
        }

        pub fn contains(&self, other: &Range) -> bool {
            other.low >= self.low && other.high <= self.high
        }

        pub fn overlaps(&self, other: &Range) -> bool {
            !self.disjoint(&other)
        }

        pub fn disjoint(&self, other: &Range) -> bool {
            other.low > self.high || other.high < self.low
        }
    }

    impl FromStr for Range {
        type Err = SimpleError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() != 2 {
                return Err(SimpleError::new("Incorrect format. Needs to be 'low-high'"));
            }

            let low_high: Result<Vec<u64>, _> =
                parts.into_iter().map(|s| s.parse::<u64>()).collect();
            if let Err(error) = low_high {
                return Err(SimpleError::from(error));
            }
            let low_high = low_high.unwrap();

            return Self::new(low_high[0], low_high[1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range::Range;

    #[test]
    fn test_if_ranges_contain_each_other() {
        let range = Range::new(5, 10).unwrap();

        let contained_range = Range::new(7, 8).unwrap();
        assert!(range.contains(&contained_range));

        assert!(!contained_range.contains(&range));

        let overlap_range = Range::new(4, 9).unwrap();
        assert!(!range.contains(&overlap_range));
        assert!(!overlap_range.contains(&range));
    }

    #[test]
    fn test_some_string_parsing() {
        let range: Range = "5-10".parse().unwrap();
        let expected = Range::new(5, 10).unwrap();

        assert_eq!(range, expected);

        let should_error: Result<Range, _> = "abc-10".parse();
        assert!(should_error.is_err(), "Parsing abc-10 should be an error");

        let should_error: Result<Range, _> = "42-10".parse();
        assert!(should_error.is_err(), "Parsing 42-10 should be an error");
    }

    #[test]
    fn test_overlaps() {
        let range = Range::new(2, 4).unwrap();
        let other = Range::new(5, 6).unwrap();
        
        assert!(!range.overlaps(&other));

        let other = Range::new(1, 2).unwrap();
        assert!(range.overlaps(&other));

        let other = Range::new(4, 10).unwrap();
        assert!(range.overlaps(&other));
    }
}
