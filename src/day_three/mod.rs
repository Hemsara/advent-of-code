use std::io;

pub fn day_three() -> io::Result<()> {
    v2()?;
    Ok(())
}

fn v2() -> io::Result<()> {
    let contents = std::fs::read_to_string("input_three.txt")?;
    let mut total_sum: usize = 0;

    for line in contents.lines() {
        if line.len() < 12 {
            continue;
        }

        let mut deletes_allowed = line.len() - 12;
        let mut banks: Vec<u8> = Vec::new();

        for &digit in line.as_bytes() {
            let d = digit - b'0';

            while let Some(&last) = banks.last() {
                if deletes_allowed > 0 && last < d {
                    banks.pop();
                    deletes_allowed -= 1;
                } else {
                    break;
                }
            }

            banks.push(d);
        }

        let mut value: usize = 0;
        for &d in banks.iter().take(12) {
            value = value * 10 + d as usize;
        }

        total_sum += value;
    }

    println!("Total sum of banks: {}", total_sum);
    Ok(())
}

fn v1() {
    let contents = std::fs::read_to_string("input_three.txt").unwrap();
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
}
