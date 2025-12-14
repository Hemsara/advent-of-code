use std::io;

// extremely inefficient solution but it works for now
// correct ans is 20223751480 [1227775554 for test data]
pub fn day_two() -> io::Result<()> {
    // v1();
    v2();
    Ok(())
}
pub fn do_check(number: &str) -> bool {
    for size in 1..=number.len() / 2 {
        if iterate_digits(number, size) {
            return true;
        }
    }
    false
}

fn iterate_digits(number: &str, chunk_size: usize) -> bool {
    let len = number.len();

    if len % chunk_size != 0 {
        return false;
    }

    let first_chunk = &number[0..chunk_size];

    for i in (0..len).step_by(chunk_size) {
        let next_chunk = &number[i..i + chunk_size];
        if next_chunk != first_chunk {
            return false;
        }
    }

    true
}

fn v2() {
    let mut final_count: i64 = 0;
    let contents = std::fs::read_to_string("input_two.txt").unwrap();

    let ranges: Vec<&str> = contents.trim().split(',').collect();

    for range in ranges {
        let range_begin = range.split('-').next().unwrap().parse::<i64>().unwrap();
        let range_end = range.split('-').last().unwrap().parse::<i64>().unwrap();

        for i in range_begin..=range_end {
            let string_i = i.to_string();

            if do_check(&string_i) {
                final_count = string_i.parse::<i64>().unwrap() + final_count;
            }
        }
    }
    println!("Final count: {}", final_count);
}

fn v1() {
    let mut final_count: i64 = 0;
    let contents = std::fs::read_to_string("input_two.txt").unwrap();

    let ranges: Vec<&str> = contents.trim().split(',').collect();

    for range in ranges {
        let range_begin = range.split('-').next().unwrap().parse::<i64>().unwrap();
        let range_end = range.split('-').last().unwrap().parse::<i64>().unwrap();

        for i in range_begin..=range_end {
            let string_i = i.to_string();
            let is_even = string_i.len() % 2 == 0;

            if is_even {
                let (left, right) = string_i.split_at(string_i.len() / 2);

                if left == right {
                    final_count = string_i.parse::<i64>().unwrap() + final_count;
                }
            }
        }
    }
    println!("Final count: {}", final_count);
}
