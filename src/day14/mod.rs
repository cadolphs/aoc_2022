use self::{
    cave::{Cave, Square},
    lines::Point,
};

mod cave;
mod lines;

pub fn run_day_14(input: String) {
    let cave = Cave::parse(&input);
    let sand_source = Point{x: 500, y: 0};

    let mut sim = Simulator::new(cave, sand_source);

    let mut steps = 0;
    while sim.step() != SimulationStepResult::Finished {
        steps += 1;
    }

    println!("There's {} units of sand produced until it flows out.", steps);
}

#[derive(Debug, PartialEq)]
pub enum SimulationStepResult {
    Finished,
    NotFinished,
}

struct Simulator {
    cave: Cave,
    sand_path: Vec<Point>,
}

impl Simulator {
    fn new(cave: Cave, sand_source: Point) -> Self {
        Simulator {
            cave,
            sand_path: vec![sand_source],
        }
    }

    fn step(&mut self) -> SimulationStepResult {
        loop {
            let start = self.get_sand_start();

            let next_square = self.try_candidates(&start);
            if let Some(next) = next_square {
                if self.cave.is_out_of_bounds(&next) {
                    return SimulationStepResult::Finished;
                }
                self.sand_path.push(next);
            } else {
                // couldn't find a next square from current starting pos
                // that means we're coming to rest:
                self.cave.mark_sand(&start);
                // now we need to backtrack the path
                self.sand_path.pop();
                return SimulationStepResult::NotFinished
            }
        }
    }

    fn try_candidates(&self, p: &Point) -> Option<Point> {
        for candidate in Self::get_candidate_squares(p) {
            if self.cave.square_at(&candidate) == Square::Air {
                return Some(candidate)
            }
        }
        None
    }

    fn get_sand_start(&self) -> Point {
        self.sand_path[self.sand_path.len() - 1]
    }

    fn get_candidate_squares<'a>(p: &'a Point) -> impl Iterator<Item = Point> + 'a {
        [0, -1, 1].into_iter().map(|dx| Point {
            x: p.x + dx,
            y: p.y + 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_simulation() {
        let p1 = Point{x: 3, y: 5};
        let p2 = Point{x: 10, y: 5};

        let path = vec![p1, p2];
        let paths = vec![path];

        let cave = Cave::from_paths(paths);

        let mut sim = Simulator::new(cave, Point{x: 5, y:0});

        // do one step
        let res = sim.step();
        assert_eq!(res, SimulationStepResult::NotFinished);

        assert_eq!(sim.cave.square_at(&Point{x: 5, y: 4}), Square::Sand);

        sim.step();
        assert_eq!(sim.cave.square_at(&Point{x: 4, y: 4}), Square::Sand);

        sim.step();
        assert_eq!(sim.cave.square_at(&Point{x: 6, y: 4}), Square::Sand);

        sim.step();
        assert_eq!(sim.cave.square_at(&Point{x: 5, y: 3}), Square::Sand);

        // check if terminates
        while sim.step() == SimulationStepResult::NotFinished {

        }
        assert!(true, "Found the end");

    }
}