use std::fs;
use std::io;

pub fn day_one() -> io::Result<()> {
    let mut starting_position = 50;
    let mut zero_count = 0;

    let contents = fs::read_to_string("input.txt"). unwrap();

    for line in contents.lines() {
        let dir = match line.chars().next().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => continue,
        };
        let steps: i32 = line[1..].parse().unwrap();

        if dir == 1 {
            let zeros_during = ((starting_position + steps) / 100) - (starting_position / 100);
            zero_count += zeros_during;
            starting_position = (starting_position + steps) % 100;
            println!(
                "The dial is rotated R{} to point at {} (passed {} zeros).",
                steps, starting_position, zeros_during
            );
        } else {
            let zeros_during = (starting_position - 1).div_euclid(100)
                - (starting_position - steps - 1).div_euclid(100);
            zero_count += zeros_during;
            starting_position = (starting_position - steps).rem_euclid(100);
            println!(
                "The dial is rotated L{} to point at {} (passed {} zeros).",
                steps, starting_position, zeros_during
            );
        }
    }

    println!("Zero position count: {}", zero_count);
    Ok(())
}
