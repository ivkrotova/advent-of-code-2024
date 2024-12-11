mod helpers;
mod day01;
mod day02;
mod day03;

use std::io;

fn main() -> io::Result<()> {
    println!("Advent of Code 2024");
    day01::solve()?;
    day02::solve()?;
    day03::solve()?;
    Ok(())
}