use json::parse;
use std::{error::Error, str::FromStr};

use itertools::Itertools;
use simple_error::SimpleError;

pub fn run_day_13(input: String) {
    let blocks = input.split("\n\n");

    let ans: i32 = blocks
        .map(|block| {
            let (packet1, packet2): (PacketData, PacketData) = block
                .lines()
                .map(|line| line.parse::<PacketData>().unwrap())
                .collect_tuple()
                .unwrap();
            packet1 < packet2
        })
        .zip(1..)
        .filter(|(right_order, _)| *right_order == true)
        .map(|(_, index)| index)
        .sum();

    println!("Index sum for pairs in right order is {}", ans);

    let mut all_packets: Vec<PacketData> = input.split("\n").filter(|line| line.len() > 0).map(|line| line.parse().unwrap()).collect();
    let driver1: PacketData = "[[2]]".parse().unwrap();
    let driver2: PacketData = "[[6]]".parse().unwrap();

    all_packets.push(driver1.clone());
    all_packets.push(driver2.clone());

    all_packets.sort_unstable();

    // now find the indices
    let pos1 = all_packets.iter().position(|item| item == &driver1).unwrap() + 1;
    let pos2 = all_packets.iter().position(|item| *item == driver2).unwrap() + 1;
    let ans = pos1 * pos2;

    println!("The decoder key is {}", ans);


}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
enum PacketData {
    Num(i32),
    List(Vec<PacketData>),
}

impl From<Vec<i32>> for PacketData {
    fn from(v: Vec<i32>) -> Self {
        use PacketData::*;
        List(v.into_iter().map(|x| Num(x)).collect())
    }
}

impl From<i32> for PacketData {
    fn from(i: i32) -> Self {
        PacketData::Num(i)
    }
}

impl FromStr for PacketData {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nested_data = parse(s).unwrap();

        nested_data.try_into()
    }
}

impl TryFrom<json::JsonValue> for PacketData {
    type Error = Box<dyn Error>;

    fn try_from(value: json::JsonValue) -> Result<Self, Self::Error> {
        use json::JsonValue::*;
        match value {
            Number(_) => Ok(PacketData::Num(value.as_i32().unwrap())),
            Array(items) => {
                let pieces: Result<Vec<PacketData>, _> =
                    items.into_iter().map(|item| item.try_into()).collect();
                Ok(PacketData::List(pieces?))
            }
            _ => Err(Box::new(SimpleError::new(
                "Can only deal with numbers and lists!",
            ))),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use itertools::EitherOrBoth::{Both, Left, Right};
        use std::cmp::Ordering::*;
        use PacketData::*;
        match (self, other) {
            (Num(x), Num(y)) => x.partial_cmp(y),
            (Num(x), List(_)) => List(vec![Num(*x)]).partial_cmp(other),
            (List(_), Num(y)) => self.partial_cmp(&List(vec![Num(*y)])),
            (List(xs), List(ys)) => {
                let pairs = xs.iter().zip_longest(ys.iter());
                for pair in pairs {
                    match pair {
                        Both(left, right) => {
                            if left < right {
                                return Some(Less);
                            } else if right < left {
                                return Some(Greater);
                            } else {
                                continue;
                            }
                        }
                        Left(_) => return Some(Greater),
                        Right(_) => return Some(Less),
                    }
                }
                // List exhausted without one item smaller or greater, so they're the same
                return Some(Equal);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PacketData;
    use PacketData::*;

    #[test]
    fn test_simple_conversions() {
        let data: PacketData = vec![1, 2, 3].into();
        let expected = List(vec![Num(1), Num(2), Num(3)]);

        assert_eq!(data, expected);
    }

    #[test]
    fn equality_tests() {
        // start with numbers
        assert!(Num(3) < Num(4), "Test Num(3) < Num(4)");
        assert!(Num(4) > Num(3));

        // now do equal-length lists
        assert!(PacketData::from(vec![1, 2, 3]) < PacketData::from(vec![2, 2, 3]));
        assert!(PacketData::from(vec![1, 2, 3]) < PacketData::from(vec![1, 2, 4]));

        // now do inequal-length lists
        assert!(PacketData::from(vec![1, 2, 3]) < PacketData::from(vec![1, 2, 3, 1]));
        assert!(PacketData::from(vec![1, 2, 3, 4]) > PacketData::from(vec![1, 2, 3]));

        // now do integer conversion lists
        assert!(Num(1) < List(vec![Num(2)]));

        // now some nesting
        let lhs = List(vec![Num(1), List(vec![Num(2), Num(3)])]);
        let rhs = List(vec![Num(1), List(vec![Num(2), Num(4)])]);
        assert!(lhs < rhs);
        // seems okay so far!
    }

    #[test]
    fn test_parsing() {
        assert_eq!(Num(3), "3".parse().unwrap());
        assert_eq!(List(vec![]), "[]".parse().unwrap());

        let lhs = List(vec![Num(1), List(vec![Num(2), Num(3)])]);
        let s = "[1, [2, 3]]";
        assert_eq!(lhs, s.parse().unwrap());
    }
}
