#[macro_use]
extern crate simple_error;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

use aocf::Aoc;
use day01::run_day_01;
use day02::run_day_02;
use day03::run_day_03;
use day04::run_day_04;

use std::env;
use std::error::Error;

fn main() {
    let session_cookie: Option<String> = env::var("aocd_session_id").ok();

    let year = 2022;

    if let Ok(input) = get_aoc_input(year, 1, &session_cookie) {
        run_day_01(input);
        print_separation();
    }

    if let Ok(input) = get_aoc_input(year, 2, &session_cookie) {
        run_day_02(input);
        print_separation()
    }

    if let Ok(input) = get_aoc_input(year, 3, &session_cookie) {
        run_day_03(input);
        print_separation()
    }

    if let Ok(input) = get_aoc_input(year, 4, &session_cookie) {
        run_day_04(input);
        print_separation();
    }
}

fn print_separation() {
    print!("\n\n\n");
}

fn get_aoc_input(
    year: i32,
    day: u32,
    session_cookie: &Option<String>,
) -> Result<String, Box<dyn Error>> {
    let mut aoc = Aoc::new().year(Some(year)).day(Some(day));
    if let Some(session_str) = session_cookie {
        aoc = aoc.cookie(session_str);
    }

    aoc = aoc.init().unwrap();
    let input = aoc.get_input(false)?;
    Ok(input)
}
