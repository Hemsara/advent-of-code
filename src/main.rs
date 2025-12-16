mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod day_five;
fn main() {
    // if let Err(e) = day_one::day_one() {
    //     eprintln!("Error: {}", e);
    // }
    // if let Err(e) = day_two::day_two() {
    //     eprintln!("Error: {}", e);
    // }
    // if let Err(e) = day_three::day_three() {
    //     eprintln!("Error: {}", e);
    // }
    // if let Err(e) = day_four::day_four() {
    //     eprintln!("Error: {}", e);
    // }
    if let Err(e) = day_five::day_five() {
        eprintln!("Error: {}", e);
    }
}
