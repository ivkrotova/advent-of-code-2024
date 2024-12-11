mod helpers;
mod day01;
mod day02;
mod day03;
mod day04;

use std::io;

fn main() -> io::Result<()> {
    println!("Advent of Code 2024");
    day01::solve()?;
    day02::solve()?;
    day03::solve()?;
    day04::solve()?;
    Ok(())
}
