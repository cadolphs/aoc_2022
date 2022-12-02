

pub fn run_day_02(input: String) {

}

fn score_round(hand_shape: &HandShape, outcome: &GameOutcome) -> u64 {
    score_shape(hand_shape) + score_outcome(outcome)
}

fn score_shape(hand_shape: &HandShape) -> u64 {
    use HandShape::*;
    match hand_shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    }
}

fn score_outcome(outcome: &GameOutcome) -> u64 {
    use GameOutcome::*;
    match outcome {
        Win => 6,
        Draw => 3,
        Loss => 0
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors
}

use std::fmt::Display;
use std::str::FromStr;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct HandShapeParseError {
    msg: String,
}

impl Display for HandShapeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldn't parse {} as a hand shape", self.msg)
    }
}

impl Error for HandShapeParseError {}

impl FromStr for HandShape {
    type Err = HandShapeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(HandShapeParseError { msg: s.to_string() });
        }
        let character = s.chars().next().unwrap();
        use HandShape::*;
        match character {
            'A' => Ok(Rock),
            'B' => Ok(Paper),
            'C' => Ok(Scissors),
            'X' => Ok(Rock),
            'Y' => Ok(Paper),
            'Z' => Ok(Scissors),
            _ => Err(HandShapeParseError { msg: s.to_string() })
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameOutcome {
    Win,
    Draw,
    Loss
}

impl HandShape {
    fn battle(&self, opponent_shape: &HandShape) -> GameOutcome {
        use HandShape::*;
        match (*self, *opponent_shape) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => GameOutcome::Win,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => GameOutcome::Draw,
            _ => GameOutcome::Loss
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_hand_shape_from_character() {
        let s = "A";
        let hand_shape: Result<HandShape, _> = s.parse();

        assert_eq!(hand_shape, Ok(HandShape::Rock));
        
        let hand_shape: Result<HandShape, _> = "foo".parse();
        assert!(hand_shape.is_err(), "HandShape is not an error");
    }

    #[test]
    fn check_some_game_outcomes() {
        let hand_shape = HandShape::Rock;
        
        assert_eq!(hand_shape.battle(&HandShape::Scissors), GameOutcome::Win);
        assert_eq!(hand_shape.battle(&HandShape::Rock), GameOutcome::Draw);
    }

    #[test]
    fn check_a_few_scores() {
        let hand_shape = HandShape::Scissors;
        let outcome = GameOutcome::Win;

        assert_eq!(score_round(&hand_shape, &outcome), 9);
    }
}