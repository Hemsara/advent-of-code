use std::fs;
use std::io;

pub fn day_four() -> io::Result<()> {
    v2();
    Ok(())
}

fn v2() {
    let contents = fs::read_to_string("input_four.txt").unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    let mut total_removed = 0;

    loop {
        let mut possible_indexes: Vec<(usize, usize)> = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                check_around(&grid, row, col, &mut possible_indexes);
            }
        }

        if possible_indexes.is_empty() {
            break; // no more accessible rolls
        }

        for (row, col) in &possible_indexes {
            grid[*row][*col] = '.';
        }

        total_removed += possible_indexes.len();
    }

    println!("Total rolls removed: {}", total_removed);

    for row in grid.iter() {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}

fn v1() {
    let contents = fs::read_to_string("input_four.txt").unwrap();

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
