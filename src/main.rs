mod day_one;
mod day_two;
mod day_three;

fn main() {
    // if let Err(e) = day_one::day_one() {
    //     eprintln!("Error: {}", e);
    // }
    // if let Err(e) = day_two::day_two() {
    //     eprintln!("Error: {}", e);
    // }
    if let Err(e) = day_three::day_three() {
        eprintln!("Error: {}", e);
    }
}
