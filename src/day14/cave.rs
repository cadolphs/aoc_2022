use itertools::Itertools;
use std::collections::HashMap;

use super::lines::{get_points_on_segment, parse_paths, Path, Point};

pub trait Cave {
    fn square_at(&self, pos: &Point) -> Square;
    fn is_out_of_bounds(&self, pos: &Point) -> bool;
    fn mark_sand(&mut self, pos: &Point);
}

pub struct VoidCave {
    content: HashMap<Point, Square>,
    deepest_y: i32,
}

impl VoidCave {
    pub fn parse(input: &str) -> Self {
        let (_, paths) = parse_paths(input).unwrap();
        Self::from_paths(paths)
    }

    pub fn from_paths(paths: Vec<Path>) -> Self {
        let mut content = HashMap::new();

        let deepest_y = paths.iter().flatten().map(|p| p.y).max().unwrap();

        for path in paths {
            for (x, y) in path.into_iter().tuple_windows() {
                for p in get_points_on_segment(x, y) {
                    content.insert(p, Square::Rock);
                }
            }
        }

        VoidCave { content, deepest_y }
    }
}

impl Cave for VoidCave {
    fn square_at(&self, pos: &Point) -> Square {
        let square = self.content.get(pos);
        square.cloned().unwrap_or_default()
    }

    fn is_out_of_bounds(&self, pos: &Point) -> bool {
        pos.y >= self.deepest_y
    }

    fn mark_sand(&mut self, pos: &Point) {
        self.content.insert(*pos, Square::Sand);
    }
}

pub struct CaveWithFloor {
    cave: VoidCave,
    floor_pos: i32,
}

impl CaveWithFloor {
    pub fn new(cave: VoidCave) -> Self {
        let floor_pos = cave.deepest_y + 2;
        CaveWithFloor { cave, floor_pos }
    }
}

impl Cave for CaveWithFloor {
    fn mark_sand(&mut self, pos: &Point) {
        self.cave.content.insert(*pos, Square::Sand);
    }

    fn is_out_of_bounds(&self, pos: &Point) -> bool {
        false
    }

    fn square_at(&self, pos: &Point) -> Square {
        if pos.y == self.floor_pos {
            Square::Rock
        } else {
            self.cave.square_at(pos)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Square {
    Air,
    Rock,
    Sand,
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
        let p1 = Point { x: 0, y: 5 };
        let p2 = Point { x: 10, y: 5 };

        let path = vec![p1, p2];
        let paths = vec![path];

        let cave = VoidCave::from_paths(paths);

        use Square::*;
        for x in 0..=10 {
            let checkpoint = Point { x: x, y: 5 };
            assert_eq!(cave.square_at(&checkpoint), Rock);
        }
        for x in 0..=10 {
            let checkpoint = Point { x: x, y: 7 };
            assert_eq!(cave.square_at(&checkpoint), Air);
        }

        assert_eq!(cave.is_out_of_bounds(&Point { x: 42, y: 5 }), true);
        assert_eq!(cave.is_out_of_bounds(&Point { x: 42, y: 4 }), false);
    }

    #[test]
    fn floor_cave_tests() {
        let p1 = Point { x: 0, y: 5 };
        let p2 = Point { x: 10, y: 5 };

        let path = vec![p1, p2];
        let paths = vec![path];

        let cave = VoidCave::from_paths(paths);
        
        let cave = CaveWithFloor::new(cave);
        assert_eq!(cave.square_at(&Point{x: 400, y: 7}), Square::Rock);

    }
}
