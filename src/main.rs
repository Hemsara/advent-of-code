use std::fs;

fn main() -> std::io::Result<()> {
    let mut starting_position = 50;
    let mut zero_position_count = 0;

    let contents = fs::read_to_string("input.txt")?;
    println!("The dial starts by pointing at 50.");

    for line in contents.lines() {
        let line = line.trim();

        if line.len() < 2 {
            continue;
        }

        let dir = match line.chars().next().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => continue,
        };

        let steps: i32 = match line[1..].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if dir == -1 {
            starting_position = (starting_position - steps).rem_euclid(100);
            if starting_position == 0 {
                zero_position_count += 1;
            }
            println!(
                "The dial is rotated L{} to point at {}.",
                steps, starting_position
            );
        } else {
            starting_position = (starting_position + steps).rem_euclid(100);
            if starting_position == 0 {
                zero_position_count += 1;
                
            }
            println!(
                "The dial is rotated R{} to point at {}.",
                steps, starting_position
            );
        }
    }

    println!("Zero position count: {}", zero_position_count);

    //
    Ok(())
}
