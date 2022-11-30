pub mod day01;
use aocf::Aoc;
use std::env;

fn main() {
    day01::run_day_01();

    let session_cookie = env::var("aocd_session_id").unwrap();

    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(2))
        .cookie(&session_cookie)
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        println!("{}", i);
    }
}
