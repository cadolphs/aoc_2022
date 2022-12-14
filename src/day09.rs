use std::{ops::{Add, Sub}, cmp::max, collections::HashSet};

use strum_macros::EnumString;

pub fn run_day_09(input: String) {
    
    let mut game: GameTracker<Situation1> = GameTracker::new();
    for (m, num_steps) in input.lines().map(read_input_line) {
        game.apply_move(m, num_steps);
    }

    let ans = game.num_visited_by_tail();
    println!("Number of unique positions visited by the tail is {}", ans);

    let mut game: GameTracker<Situation2> = GameTracker::new();
    for (m, num_steps) in input.lines().map(read_input_line) {
        game.apply_move(m, num_steps);
    }

    let ans = game.num_visited_by_tail();
    println!("Number of unique positions visited by the new tail is {}", ans);
}

fn read_input_line(line: &str) -> (Move, usize) {
    let split_lines: Vec<&str> = line.split(' ').collect();
    assert!(split_lines.len() == 2);
    
    let m = split_lines[0].parse().unwrap();
    let num = split_lines[1].parse().unwrap();

    (m, num)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2D(i32, i32);

impl Vec2D {
    fn new() -> Self {
        Self::default()
    }

    fn inf_norm(&self) -> i32 {
        max(self.0.abs(), self.1.abs())
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

trait Situation {
    fn new() -> Self;
    fn apply_single_move(&mut self, m: Move);
    fn get_tail_pos(&self) -> Vec2D;
}

#[derive(Debug, Copy, Clone)]
struct Situation1 {
    head: Vec2D,
    tail: Vec2D
}

impl Situation for Situation1 {
    fn new() -> Self {
        Situation1{ head: Vec2D::new(), tail: Vec2D::new() }
    }

    fn apply_single_move(&mut self, m: Move) {
        let v = m.to_vec();

        let head = self.head + v;
        let mut tail = self.tail;

        let dist = head - self.tail;
        if dist.inf_norm() > 1 {
            let m = match (dist.0, dist.1) {
                (d, 2) => Vec2D(d, 1),
                (d, -2) => Vec2D(d, -1),
                (2, d) => Vec2D(1, d),
                (-2, d) => Vec2D(-1, d),
                (x, y) => panic!("Should never see something like {:?}", (x, y))
            };

            tail = tail + m;    
        }

        self.head = head;
        self.tail = tail;
    }

    fn get_tail_pos(&self) -> Vec2D {
        self.tail
    }

}

struct GameTracker<S: Situation> {
    situation: S,
    visited_by_tail: HashSet<Vec2D>
}

impl<S: Situation> GameTracker<S> {
    fn new() -> Self {
        let situation = S::new();
        let mut visited_by_tail = HashSet::new();
        visited_by_tail.insert(situation.get_tail_pos());

        GameTracker{ situation, visited_by_tail }
    }

    fn apply_move(&mut self, m: Move, num_steps: usize) {
        for _ in 0..num_steps {
            self.situation.apply_single_move(m);
            self.visited_by_tail.insert(self.situation.get_tail_pos());
        }
    }

    fn num_visited_by_tail(&self) -> usize {
        self.visited_by_tail.len()
    }
}

#[derive(Debug, Clone, Copy, EnumString, PartialEq)]
enum Move {
    L,
    R,
    U,
    D
}

impl Move {
    fn to_vec(&self) -> Vec2D {
        use Move::*;
        match self {
            L => Vec2D(-1, 0),
            R => Vec2D(1, 0),
            U => Vec2D(0, 1),
            D => Vec2D(0, -1)
        }
    }
}

#[derive(Debug)]
struct Situation2 {
    knots: [Vec2D; 10]
}

impl Situation for Situation2 {
    fn new() -> Self {
        Situation2{knots: [Vec2D::new(); 10]}
    }

    fn apply_single_move(&mut self, m: Move) {
        self.knots[0] = self.knots[0] + m.to_vec();

        for i in 1..10 {
            let distance_to_target = self.knots[i-1] - self.knots[i];

            if distance_to_target.inf_norm() > 1 {
                let dx = distance_to_target.0.signum();
                let dy = distance_to_target.1.signum();
                let delta = Vec2D(dx, dy);
                self.knots[i] = self.knots[i] + delta;
            }
        }
    }

    fn get_tail_pos(&self) -> Vec2D {
        self.knots[9]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn some_vec_tests() {
        let v = Vec2D::default();
        assert_eq!((0, 0), (v.0, v.1));
    }

    #[test]
    fn test_simple_move_stuff() {
        let mut situation = Situation1::new();
        situation.apply_single_move(Move::R);
        let expected_tail = Vec2D(0, 0);
        assert_eq!(expected_tail, situation.tail);

        situation.apply_single_move(Move::R);
        let expected_tail = Vec2D(1, 0);
        assert_eq!(expected_tail, situation.tail);

        situation.apply_single_move(Move::U);
        assert_eq!(expected_tail, situation.tail);

        situation.apply_single_move(Move::U);
        let expected_tail = Vec2D(2, 1);
        assert_eq!(expected_tail, situation.tail);
    }

    #[test]
    fn test_game_tracker() {
        let mut game: GameTracker<Situation1> = GameTracker::new();
        game.apply_move(Move::R, 2); //tail now was at 0,0 and 1,0
        game.apply_move(Move::D, 2); //tail now also at 1,-1

        assert_eq!(3, game.num_visited_by_tail());
    }

    #[test]
    fn test_read_input_line() {
        let (m, num) = read_input_line("R 4");
        assert_eq!(m, Move::R);
        assert_eq!(num, 4);
    }
}