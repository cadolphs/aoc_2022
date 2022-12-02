pub mod day01;
pub mod day02;
use aocf::Aoc;
use day01::run_day_01;
use std::env;
use std::error::Error;

fn main() {
    let session_cookie: Option<String> = env::var("aocd_session_id").ok();

    let year = 2022;
    
    if let Ok(input) = get_aoc_input(year, 1, &session_cookie) {
        run_day_01(input);
    }
}

fn get_aoc_input(year: i32, day: u32, session_cookie: &Option<String>) -> Result<String, Box<dyn Error>> {
    let mut aoc = Aoc::new()
        .year(Some(year))
        .day(Some(day));
    if let Some(session_str) = session_cookie {
        aoc = aoc.cookie(session_str);
    }

    aoc = aoc
        .init()
        .unwrap();
    let input = aoc.get_input(false)?;
    Ok(input)
}
