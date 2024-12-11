use std::io;
use crate::helpers::readers;
use regex::Regex;

pub fn find_pattern(contents: &str) -> Vec<(i32, i32)> {
    // Find the pattern in the contents
    // The valid pattern must be exactly "mul(X,Y)", where X and Y are 1-3 digit numbers
    // Extra characters do nothing
    // The pattern must be found in the contents
    // Return all the patterns found in the contents
    // contents is one string
    let pattern = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let mut patterns = Vec::new();
    for capture in pattern.captures_iter(contents) {
        let x = &capture[1];
        let y = &capture[2];
        patterns.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    }
    return patterns;
}

pub fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

pub fn solve() -> io::Result<()> {
    let file_path = "src/data/input03";
    let contents = readers::read_file_contents(file_path);
    let patterns = find_pattern(&contents);
    // iter() is a lazy iterator
    // sum() is a function that takes an iterator and returns the sum of the elements
    // map() is a function that takes an iterator and returns a new iterator
    // |(x, y)| multiply(*x, *y) is a closure that takes a tuple and returns the product of the two elements
    let result = patterns.iter().map(|(x, y)| multiply(*x, *y)).sum::<i32>();
    println!("The Day 3 solution is: {}", result);
    return Ok(());
}