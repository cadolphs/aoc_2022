use nom::{
    bytes::complete::tag,
    character::complete::i32,
    combinator::{map, verify},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Point> {
        todo!()
    }
}

fn paths(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(tag("\n"), path)(input)
}

fn path(input: &str) -> IResult<&str, Vec<Point>> {
    let path_parser = separated_list1(tag(" -> "), pair);
    verify(path_parser, |path: &Vec<Point>| path.len() > 1)(input)
}

fn pair(input: &str) -> IResult<&str, Point> {
    map(separated_pair(number, tag(","), number), |(x, y)| Point {
        x,
        y,
    })(input)
}

fn number(input: &str) -> IResult<&str, i32> {
    i32(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_number() {
        let res = number("123abc").unwrap();
        assert_eq!(res.1, 123);
    }

    #[test]
    fn it_parses_a_pair() {
        let (rest, pair) = pair("123,456;").unwrap();
        assert_eq!(rest, ";");
        assert_eq!(Point { x: 123, y: 456 }, pair);
    }

    #[test]
    fn it_parses_a_path() {
        assert!(path("123,456").is_err());
        assert_eq!(
            path("123,456 -> 789,123").unwrap().1,
            vec![Point { x: 123, y: 456 }, Point { x: 789, y: 123 }]
        );
    }

    #[test]
    fn it_parses_paths() {
        let input = "123,456 -> 789,123\n424,456 -> 152,123";
        let expected = vec![vec![Point { x: 123, y: 456 }, Point { x: 789, y: 123 }], vec![Point{x: 424, y: 456}, Point{x:152, y: 123}]];
        let (rest, res) = paths(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(expected, res);
    }
}
