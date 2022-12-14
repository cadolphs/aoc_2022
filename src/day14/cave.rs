use std::collections::HashMap;
use itertools::Itertools;

use super::lines::{Point, Path, get_points_on_segment, parse_paths};

pub struct Cave {
    content: HashMap<Point, Square>,
    deepest_y: i32,
}

impl Cave {
    pub fn parse(input: &str) -> Self {
        let (_, paths) = parse_paths(input).unwrap();
        Self::from_paths(paths)
    }

    pub fn from_paths(paths: Vec<Path>) -> Self {
        let mut content = HashMap::new();
        
        let deepest_y = paths.iter().flatten()
        .map(|p| p.y).max().unwrap();

        for path in paths {
            for (x, y) in path.into_iter().tuple_windows() {
                for p in get_points_on_segment(x, y) {
                    content.insert(p, Square::Rock);
                }
            }
        }

        Cave{content, deepest_y}
    }

    pub fn square_at(&self, pos: &Point) -> Square {
        let square = self.content.get(pos);
        square.cloned().unwrap_or_default()
    }

    pub fn is_out_of_bounds(&self, pos: &Point) -> bool {
        pos.y >= self.deepest_y
    }

    pub fn mark_sand(&mut self, pos: &Point) {
        self.content.insert(*pos, Square::Sand);
    }

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Square {
    Air,
    Rock,
    Sand
}

impl Default for Square {
    fn default() -> Self {
        Square::Air
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cave() {
        let p1 = Point{x: 0, y: 5};
        let p2 = Point{x: 10, y: 5};

        let path = vec![p1, p2];
        let paths = vec![path];

        let cave = Cave::from_paths(paths);

        use Square::*;
        for x in 0..=10 {
            let checkpoint = Point{x: x, y: 5};
            assert_eq!(cave.square_at(&checkpoint), Rock);
        }
        for x in 0..=10 {
            let checkpoint = Point{x: x, y: 7};
            assert_eq!(cave.square_at(&checkpoint), Air);
        }

        assert_eq!(cave.is_out_of_bounds(&Point{x: 42, y:5}), true);
        assert_eq!(cave.is_out_of_bounds(&Point{x: 42, y:4}), false);
    }
}