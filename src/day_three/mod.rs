use std::io;

pub fn day_three() -> io::Result<()> {
    let contents = std::fs::read_to_string("input_three.txt")?;
    let mut highest_banks: Vec<usize> = Vec::new();

    for line in contents.lines() {
        let bytes = line.as_bytes();
        let mut current_highest_bank: usize = 0;

        for j in 0..bytes.len() {
            let a = (bytes[j] - b'0') as usize;

            for k in (j + 1)..bytes.len() {
                let b = (bytes[k] - b'0') as usize;
                let v = a * 10 + b;

                if v > current_highest_bank {
                    current_highest_bank = v;
                }
            }
        }

        highest_banks.push(current_highest_bank);
    }

    println!("Highest banks: {:?}", highest_banks);
    let sum: usize = highest_banks.iter().sum();
    println!("Sum of highest banks: {}", sum);

    Ok(())
}
