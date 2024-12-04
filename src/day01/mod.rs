use std::fs::File;
use std::io::{self, Read};



pub fn solve() -> io::Result<()> {
    let file_path = "src/day01/input.txt";
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
            if let (Some(col1), Some(col2)) = (parts.next(), parts.next()) {
                if let (Ok(num1), Ok(num2)) = (col1.parse::<i32>(), col2.parse::<i32>()) {
                        column1.push(num1);
                        column2.push(num2);
                    }
                }
            }  

    column1.sort();
    column2.sort();

    let total_difference: i32 = column1.iter()
        .zip(column2.iter())
        .map(|(c1, c2)| (c1 - c2).abs())
        .sum();
    println!("Total difference: {}", total_difference);
    
    Ok(())
}

