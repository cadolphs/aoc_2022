mod parsing {}

mod themap {
    use super::{vec2d::Vec2D, interval::Interval};

    pub struct SensorBeaconPair {
        sensor: Vec2D,
        beacon: Vec2D,

        size: i32
    }

    impl SensorBeaconPair {
        pub fn new(sensor: Vec2D, beacon: Vec2D) -> Self {
            let size = sensor.manhattan(&beacon);
            SensorBeaconPair{sensor, beacon, size}
        }

        pub fn get_y_intersect(&self, y_pos: i32) -> Option<Interval> {
            let y_dist = (y_pos - self.sensor.1).abs();
            if y_dist > self.size {
                None
            } else {
                //what is the intersect?
                let left_over_for_x_dist = self.size - y_dist;
                let min_x = self.sensor.0 - left_over_for_x_dist;
                let max_x = self.sensor.0 + left_over_for_x_dist;
                Some(Interval::new(min_x, max_x))
            }
        }
    }
}

mod vec2d {
    use std::ops::{Sub, Add};

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Vec2D(pub i32, pub i32);

    impl Vec2D {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn inf_norm(&self) -> i32 {
            std::cmp::max(self.0.abs(), self.1.abs())
        }

        pub fn norm1(&self) -> i32 {
            self.0.abs() + self.1.abs()
        }

        pub fn manhattan(&self, other: &Self) -> i32 {
            (*self - *other).norm1()
        }
    }

    impl Add<Vec2D> for Vec2D {
        type Output = Vec2D;

        fn add(self, rhs: Vec2D) -> Vec2D {
            Vec2D(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl Sub<Vec2D> for Vec2D {
        type Output = Vec2D;

        fn sub(self, rhs: Vec2D) -> Vec2D {
            Vec2D(self.0 - rhs.0, self.1 - rhs.1)
        }
    }
}

mod interval {
    use itertools::Itertools;

    #[derive(Debug)]
    pub struct IntervalSet {
        intervals: Vec<Interval>,
    }

    impl IntervalSet {
        pub fn new() -> Self {
            let intervals = vec![];
            IntervalSet { intervals }
        }

        pub fn add(&mut self, interval: Interval) {
            let intersector = self
                .intervals
                .iter()
                .find_position(|i| interval.intersects_or_abuts(i));

            if let Some((pos, other)) = intersector {
                let new_interval = interval.combine(other);
                self.intervals.remove(pos);
                self.add(new_interval);
            } else {
                // new interval disjoint with all other intervals so we can just add it.
                self.intervals.push(interval);
                // maybe this makes live easier? sort intervals in "ascending" order?
                self.intervals.sort_unstable_by_key(|interval| interval.0);
            }
        }

        pub fn len(&self) -> usize {
            self.intervals.iter().map(|i| i.len()).sum()
        }
    }

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

        pub fn len(&self) -> usize {
            (self.1 - self.0 + 1) as usize
        }

        pub fn intersects_or_abuts(&self, other: &Self) -> bool {
            !(self.0 > other.1 + 1 || self.1 < other.0 - 1)
        }

        pub fn combine(self, other: &Self) -> Self {
            assert!(self.intersects_or_abuts(other));

            let x = std::cmp::min(self.0, other.0);
            let y = std::cmp::max(self.1, other.1);

            Interval::new(x, y)
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
                vec![Interval(self.0, other.0 - 1), Interval(other.1 + 1, self.1)]
            } else if self.0 < other.0 && self.1 <= other.1 {
                vec![Interval(self.0, other.0 - 1)]
            } else if self.0 >= other.0 && self.1 > other.1 {
                vec![Interval(other.1 + 1, self.1)]
            } else {
                panic!("Seems like I missed some interval subtraction logic");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{interval::*, vec2d::Vec2D, themap::SensorBeaconPair};

    #[test]
    fn test_interval_creation() {
        assert_eq!(Interval::new(1, 2), Interval::new(2, 1));
    }

    #[test]
    fn test_interval_subtraction() {
        let base = Interval::new(0, 10);

        let subs = vec![
            Interval::new(42, 55),
            Interval::new(2, 8),
            Interval::new(-2, 13),
            Interval::new(0, 5),
            Interval::new(5, 10),
            Interval::new(-2, 4),
            Interval::new(8, 12),
        ];
        let expecteds = vec![
            vec![Interval::new(0, 10)],
            vec![Interval::new(0, 1), Interval::new(9, 10)],
            vec![],
            vec![Interval::new(6, 10)],
            vec![Interval::new(0, 4)],
            vec![Interval::new(5, 10)],
            vec![Interval::new(0, 7)],
        ];

        for (sub, expected) in subs.into_iter().zip(expecteds) {
            assert_eq!(expected, base.subtract(&sub))
        }
    }

    #[test]
    fn test_interval_set_compputations() {
        let interval = Interval::new(0, 9);
        let mut intervals = IntervalSet::new();

        intervals.add(interval);
        assert_eq!(
            intervals.len(),
            interval.len(),
            "Single interval should just get added to interval set"
        );

        intervals.add(Interval::new(13, 14));
        assert_eq!(
            intervals.len(),
            12,
            "Adding disjoint interval increases len by that amount"
        );

        intervals.add(Interval::new(3, 7));
        assert_eq!(
            intervals.len(),
            12,
            "Adding subsumed interval doesn't change length"
        );

        intervals.add(Interval::new(-5, 3));
        assert_eq!(intervals.len(), 17);
    }

    #[test]
    fn test_y_intersect() {
        let sensor = Vec2D(0, 5);
        let beacon = Vec2D(0, 10);
        
        let sb = SensorBeaconPair::new(sensor, beacon);

        let y_pos = -100;
        assert_eq!(sb.get_y_intersect(y_pos), None);

        let y_pos = 3;
        //intersect points should be (-3, 3) all the way up to (3, 3)
        
        let interval = sb.get_y_intersect(y_pos);
        assert_eq!(interval, Some(Interval::new(-3, 3)));
    }
}
