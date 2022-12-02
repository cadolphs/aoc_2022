use itertools::Itertools;

use simple_error::SimpleError;
pub fn run_day_02(input: String) {
    let rounds = parse_input(input.clone()).unwrap();

    let total_score = score_game(rounds.iter());
    println!("Total score following initial startegy is: {}", total_score);

    let part2_input = parse_part2_input(input).unwrap();
    let part2_rounds: Vec<(HandShape, HandShape)> = compute_required_hand_shapes(part2_input);

    let total_score_round2 = score_game(part2_rounds.iter());
    println!("With the renewed strategy, the total score is: {}", total_score_round2);
}

fn compute_required_hand_shapes(part2_input: Vec<(HandShape, GameOutcome)>) -> Vec<(HandShape, HandShape)> {
    part2_input.into_iter().map(|(shape, outcome)| (shape, hand_shape_for_outcome(&shape, &outcome))).collect()
}

fn parse_input(input: String) -> Result<Vec<(HandShape, HandShape)>, Box<dyn Error>> {
    input.lines().map(parse_line).collect()
}

fn parse_part2_input(input: String) -> Result<Vec<(HandShape, GameOutcome)>, Box<dyn Error>> {
    input.lines().map(parse_part2_line).collect()
}

fn parse_line(input: &str) -> Result<(HandShape, HandShape), Box<dyn Error>> {
    let res_vec: Result<Vec<HandShape>, _> = input.split(' ').map(|entry| entry.parse()).collect();
    res_vec?
        .into_iter()
        .collect_tuple()
        .ok_or(Box::new(SimpleError::new(
            "Wrong number of hand shapes on that line",
        )))
}

fn parse_part2_line(input: &str) -> Result<(HandShape, GameOutcome), Box<dyn Error>> {
    let codes: (&str, &str) = input
        .split(' ')
        .collect_tuple()
        .ok_or(Box::new(SimpleError::new("Wrong format for line")))?;
    Ok((codes.0.parse()?, codes.1.parse()?))
}

fn score_game<'a>(rounds: impl Iterator<Item = &'a (HandShape, HandShape)>) -> u64 {
    rounds
        .map(|(opponent, own)| score_round(&own, &own.battle(&opponent)))
        .sum()
}

fn hand_shape_for_outcome(opponent_shape: &HandShape, intended_outcome: &GameOutcome) -> HandShape {
    use HandShape::*;
    use GameOutcome::*;
    match (opponent_shape, intended_outcome) {
        (any_shape, Draw) => *any_shape,
        (Rock, Win) | (Scissors, Loss) => Paper,
        (Paper, Win) | (Rock, Loss) => Scissors,
        _ => Rock
    }
}

fn score_round(hand_shape: &HandShape, outcome: &GameOutcome) -> u64 {
    score_shape(hand_shape) + score_outcome(outcome)
}

fn score_shape(hand_shape: &HandShape) -> u64 {
    use HandShape::*;
    match hand_shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn score_outcome(outcome: &GameOutcome) -> u64 {
    use GameOutcome::*;
    match outcome {
        Win => 6,
        Draw => 3,
        Loss => 0,
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

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
            _ => Err(HandShapeParseError { msg: s.to_string() }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameOutcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for GameOutcome {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(SimpleError::new("Cannot parse that to a game outcome"));
        }

        let character = s.chars().next().unwrap();
        use GameOutcome::*;
        match character {
            'X' => Ok(Loss),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err(SimpleError::new(
                "Only X, Y, and Z are valid characters for game outcomes",
            )),
        }
    }
}

impl HandShape {
    fn battle(&self, opponent_shape: &HandShape) -> GameOutcome {
        use HandShape::*;
        match (*self, *opponent_shape) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => GameOutcome::Win,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => GameOutcome::Draw,
            _ => GameOutcome::Loss,
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

    #[test]
    fn hacking_around_with_parsing_a_tuple() {
        let two_shapes: (HandShape, HandShape) = parse_line("A X").unwrap();

        assert_eq!(two_shapes.0, HandShape::Rock)
    }

    #[test]
    fn test_parsing_of_game_outcome() {
        let outcome: GameOutcome = "X".parse().unwrap();
        assert_eq!(outcome, GameOutcome::Loss);
    }
}
