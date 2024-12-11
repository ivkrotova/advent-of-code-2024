use std::io;
use crate::helpers::readers;

// Define the matrix type
type Grid = Vec<Vec<char>>;

fn read_grid(contents: &str) -> Grid {
    contents
        .lines() // lines() returns an iterator
        .map(|line| { // map() applies a function to each element of the iterator
            line.chars().collect::<Vec<char>>() // collect() converts the iterator to a vector
        })
        .collect() // collect() converts the iterator to a vector
}

fn validate_grid(grid: &Grid) -> bool {
    if grid.is_empty() {
        return false;
    }

    let width = grid[0].len();
    grid.iter().all(|row| row.len() == width) // check if all rows have the same length
}

// Find the X in all positions in the grid
fn find_all_x(grid: &Grid) -> Vec<(usize, usize)> {
    let mut x_positions = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'X' {
                x_positions.push((i, j));
            }
        }
    }
    return x_positions;
}

// Check if the position is out of bounds
fn is_out_of_bounds(row: i32, col: i32, grid: &Grid) -> bool {
    row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32
}

// Check if the XMAS is found in the grid from the X position
fn check_from_x(grid: &Grid, x_row: usize, x_col: usize, d_row: i32, d_col: i32) -> bool {
    // Convert starting position to i32 for safe arithmetic
    let row = x_row as i32;
    let col = x_col as i32;
    
    // Check if any position would be out of bounds
    for i in 0..4 {
        if is_out_of_bounds(row + i * d_row, col + i * d_col, grid) {
            return false;
        }
    }
    
    // Check for 'M', 'A', 'S' in sequence
    grid[(row + d_row) as usize][(col + d_col) as usize] == 'M' &&
    grid[(row + 2 * d_row) as usize][(col + 2 * d_col) as usize] == 'A' &&
    grid[(row + 3 * d_row) as usize][(col + 3 * d_col) as usize] == 'S'
}

// Find all the XMAS in the grid from the X positions
fn find_xmas(grid: &Grid, x_pos: &[(usize, usize)]) -> Vec<((usize, usize), (i32, i32))> {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];
    
    let mut found = Vec::new();
    for &(row, col) in x_pos {
        for &(d_row, d_col) in &directions {
            if check_from_x(grid, row, col, d_row, d_col) {
                found.push(((row, col), (d_row, d_col)));
            }
        }
    }
    found
}

pub fn solve() -> io::Result<()> {
    let file_path = "src/data/input04";
    let contents = readers::read_file_contents(file_path);
    let grid = read_grid(&contents);
    if !validate_grid(&grid) {
        println!("The grid is invalid");
        return Ok(());
    }
    
    let x_positions = find_all_x(&grid);
    let xmas_positions = find_xmas(&grid, &x_positions);
    println!("The number of XMAS found is: {}", xmas_positions.len());
    Ok(())
}
