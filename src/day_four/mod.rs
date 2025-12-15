use std::fs;
use std::io;

pub fn day_four() -> io::Result<()> {
    let contents = fs::read_to_string("input_four.txt")?;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut possible_indexes: Vec<(usize, usize)> = Vec::new();

    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            check_around(&grid, row, col, &mut possible_indexes);
        }
    }

    let total_accessible = possible_indexes.len();
    println!("Total accessible rolls: {}", total_accessible);

    Ok(())
}

fn check_around(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    possible_indexes: &mut Vec<(usize, usize)>,
) -> bool {
    if grid[row][col] != '@' {
        return false;
    }

    let mut count = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in directions {
        let nr = row as isize + dr;
        let nc = col as isize + dc;

        if nr >= 0 && nc >= 0 && (nr as usize) < grid.len() && (nc as usize) < grid[row].len() {
            if grid[nr as usize][nc as usize] == '@' {
                count += 1;
            }
        }
    }

    if count < 4 {
        possible_indexes.push((row, col));
        return true;
    }

    false
}
