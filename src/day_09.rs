use std::str::FromStr;
use std::collections::VecDeque;

static ENCRYPTED: &'static str = include_str!("../data/data_09.txt");

pub fn part_1() -> Option<u32> {
    let mut enc_data = get_encrypted_data();

    let mut window = enc_data.by_ref()
        .take(25)
        .collect::<VecDeque<_>>();

    for n in enc_data {
        if !is_valid(n, &window) {
            return Some(n);
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
    None
}

pub fn part_2() -> Option<u32> {
    let mut enc_data = get_encrypted_data();
    let invalid = part_1().expect("No invalid number found.");
    let mut buffer = vec![];

    for start in 0.. {
        let mut sum = 0;

        for i in start.. {
            // Lazy buffer fill
            if i >= buffer.len() {
                if let Some(n) = enc_data.next() {
                    buffer.push(n);
                } else {
                    // Sequence not found.
                    return None;
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
                return Some(low + high);
            }
        }
    }
    None
}

fn get_encrypted_data() -> impl Iterator<Item=u32> {
    ENCRYPTED.lines()
        .map(|line| u32::from_str(line).unwrap())
}