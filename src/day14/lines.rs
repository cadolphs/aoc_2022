use nom::{
    bytes::complete::tag,
    character::complete::i32,
    combinator::{map, verify},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub type Path = Vec<Point>;

pub fn get_points_on_segment(point1: Point, point2: Point) -> Vec<Point> {
    let (x1, y1) = (point1.x, point1.y);
    let (x2, y2) = (point2.x, point2.y);

    if x1 == x2 {
        either_way_range(y1, y2)
            .map(|y| Point { x: x1, y })
            .collect()
    } else if y1 == y2 {
        either_way_range(x1, x2).map(|x| Point { x, y: y1 }).collect()
    } else {
        panic!("Invalid input. Successive points aren't on the same line")
    }
}

fn either_way_range(start: i32, end: i32) -> RangeInclusive<i32> {
    let (start, end) = if start < end {
        (start, end)
    } else {
        (end, start)
    };
    start..=end
}
pub fn parse_paths(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(tag("\n"), path)(input)
}

fn path(input: &str) -> IResult<&str, Path> {
    let path_parser = separated_list1(tag(" -> "), pair);
    verify(path_parser, |path: &Path| path.len() > 1)(input)
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
        let expected = vec![
            vec![Point { x: 123, y: 456 }, Point { x: 789, y: 123 }],
            vec![Point { x: 424, y: 456 }, Point { x: 152, y: 123 }],
        ];
        let (rest, res) = parse_paths(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(expected, res);
    }

    #[test]
    fn it_computes_points_on_segments() {
        let p1 = Point { x: 0, y: 10 };
        let p2 = Point { x: 0, y: 5 };

        let points = get_points_on_segment(p1, p2);
        assert_eq!(points.len(), 6);

        assert_eq!(p2, points[0]);
        assert_eq!(p1, points[5]);
        assert_eq!(Point{x: 0, y: 6}, points[1]);
    }
}
