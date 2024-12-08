// Import necessary modules from the standard library
// File is used for reading files from the disk
use std::fs::File;
// io is used input/output functionality
use std::io::{self, Read};
use crate::helpers::readers;

// This function solves the puzzle and returns a Result type
// The Result can either be Ok (success) or an io::Error
pub fn solve() -> io::Result<()> {
    // First, define the path to the input file
    let file_path = "src/day01/input.txt";
    let contents = readers::read_file_contents(file_path);

    // Create two empty vectors to store our numbers
    // Vec<i32> means a vector (growable array) of 32-bit integers
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    // Fifth, read the file line by line    
    for line in contents.lines() {
        // Each input line is split into two numbers by whitespace
        let mut parts = line.split_whitespace();
        // Try to get two numbers from each line
        // parts contains an iterator over the parts of the line
        if let (Some(col1), Some(col2)) = (parts.next(), parts.next()) {
            // Now we try to parse the string numbers into integers
            if let (Ok(num1), Ok(num2)) = (col1.parse::<i32>(), col2.parse::<i32>()) {
                // If successful, add the numbers to the vectors created earlier
                column1.push(num1);
                column2.push(num2);
            }
        }
    }  

    // Sort both columns in ascending order
    column1.sort();
    column2.sort();

    // Calculate the total difference between corresponding numbers:
    // 1. iter() creates an iterator over the vectors
    // 2. zip() pairs up elements from both vectors, e.g. (1, 2), (3, 4), (5, 6)
    // 3. map() transforms each pair into their absolute difference, e.g. (1, 2) -> 1
    // 4. sum() adds up all the differences, e.g. 1 + 1 + 1 = 3
    let total_difference: i32 = column1.iter()
        .zip(column2.iter())
        .map(|(c1, c2)| (c1 - c2).abs())
        .sum();

    println!("The Day 1 solution is: {}", total_difference);
    
    // Return Ok to indicate success
    Ok(())
}

