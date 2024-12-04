mod day01;
mod day02;

use std::io;

fn main() -> io::Result<()> {
    println!("Advent of Code 2024");
    day01::solve()?;
    Ok(())
}
