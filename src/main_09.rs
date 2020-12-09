use std::str::FromStr;
use std::collections::VecDeque;

static ENCRYPTED: &'static str = include_str!("../data/data_09.txt");

fn main() {
    // Part 1.

    let mut enc_data = ENCRYPTED.lines()
        .map(|line| u32::from_str(line).unwrap());

    let mut window = enc_data.by_ref()
        .take(25)
        .collect::<VecDeque<_>>();

    let mut invalid = None;

    for n in enc_data {
        if !is_valid(n, &window) {
            invalid = Some(n);
            println!("Part 1: Invalid value: {}", n);
            break;
        }

        window.pop_front();
        window.push_back(n);
    }

    // `n` is valid if two distinct values in `window` sum to `n`.
    fn is_valid(n: u32, window: &VecDeque<u32>) -> bool {
        for i in 0..window.len() {
            for j in i..window.len() {
                if window[i] + window[j] == n { return true; }
            }
        }
        false
    }

    // Part 2.

    let mut enc_data = ENCRYPTED.lines()
        .map(|line| u32::from_str(line).unwrap());

    let mut buffer = vec![];

    let invalid = invalid.expect("No invalid number found.");

    for start in 0.. {
        let mut sum = 0;

        for i in start.. {
            // Lazy buffer fill
            if i >= buffer.len() {
                if let Some(n) = enc_data.next() {
                    buffer.push(n);
                } else {
                    println!("Sequence summing to invalid item not found.");
                    return;
                }
            }

            sum += buffer[i];

            if sum > invalid {
                // Break to outer loop and retry starting from next index.
                break;
            } else if sum == invalid {
                // Found a sequence.
                let mut seq = buffer[start..=i].to_vec();
                seq.sort();

                let (low, high) = (seq.first().unwrap(), seq.last().unwrap());

                println!("Part 2: {} + {} == {}", low, high, low + high);
                return;
            }
        }
    }
}