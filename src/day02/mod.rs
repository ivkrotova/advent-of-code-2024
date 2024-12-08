use crate::helpers::readers;
use std::io;

fn parse_levels(report: &str) -> Vec<i32> {
    report.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn all_increasing(report: &str) -> bool {
    let levels = parse_levels(report);
    for i in 0..levels.len() - 1 {
        if levels[i] >= levels[i + 1] {
            return false;
        }
    }
    return true;
}

pub fn all_decreasing(report: &str) -> bool {
    let levels = parse_levels(report);
    for i in 0..levels.len() - 1 {
        if levels[i] <= levels[i + 1] {
            return false;
        }
    }
    return true;
}

pub fn any_adjacent_diff_gt_3(report: &str) -> bool {
    let levels = parse_levels(report);
    for i in 0..levels.len() - 1 {
        if (levels[i] - levels[i + 1]).abs() > 3 {
            return true;
        }
    }
    return false;
}

pub fn is_safe(report: &str) -> bool {
    if report.trim().is_empty() {
        return false;
    }
    
    return (all_increasing(report) || all_decreasing(report)) && !any_adjacent_diff_gt_3(report);
}

pub fn solve() -> io::Result<()> {
    // First, define the path to the input file
    let file_path = "src/day02/input";
    let contents = readers::read_file_contents(file_path);
    let mut safe_reports = 0;
    // The data (contents) consists of many reports, one report per line
    for report in contents.split("\n") {
        // Check if the report is not empty and is safe
        if !report.trim().is_empty() && is_safe(report) {
            safe_reports += 1;
        }
    }
    // Print the number of safe reports
    println!("Safe reports: {}", safe_reports);
    return Ok(());
}