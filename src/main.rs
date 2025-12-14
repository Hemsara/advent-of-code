mod day_one;
fn main() {
    if let Err(e) = day_one::day_one() {
        eprintln!("Error: {}", e);
    }
}
