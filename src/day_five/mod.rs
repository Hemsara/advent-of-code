use std::fs;
use std::io;

pub fn day_five() -> io::Result<()> {
    v1();
    Ok(())
}

fn v2() {}

fn v1() {
    let mut fresh_ingredient_id_ranges: Vec<(usize, usize)> = Vec::new();

    let mut available_ingredient_ids: Vec<usize> = Vec::new();

    let contents = fs::read_to_string("input_five.txt").unwrap();

    let sections: Vec<&str> = contents.split("\n\n").collect();

    for line in sections[0].lines() {
        let parts: Vec<&str> = line.split("-").collect();
        let start: usize = parts[0].trim().parse().unwrap();
        let end: usize = parts[1].trim().parse().unwrap();
        fresh_ingredient_id_ranges.push((start, end));
    }
    for line in sections[1].lines() {
        let id: usize = line.trim().parse().unwrap();
        available_ingredient_ids.push(id);
    }

    println!(
        "Fresh ingredient ID ranges: {:?}",
        fresh_ingredient_id_ranges
    );
    println!("Available ingredient IDs: {:?}", available_ingredient_ids);
    let mut count = 0;

    for ing in &available_ingredient_ids {
        for (start, end) in &fresh_ingredient_id_ranges {
            if ing >= start && ing <= end {
                count += 1;
                break;
            }
        }
    }
    println!("Total available fresh ingredients: {}", count);
}
