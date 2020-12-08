use std::str::FromStr;

static EXPENSE_DATA: &'static str = include_str!("../data/data_01.txt");

fn main() {
    // Common.
    let nums = EXPENSE_DATA.lines()
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<_>>();

    // Part 1.
    println!("== Part 1 ==");
    'outer_1: for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 {
                println!(
                    "Found entries: {} + {} == 2020; tell the elves their product is: {}.",
                    nums[i], nums[j], nums[i] * nums[j]);
                    break 'outer_1;
            }
        }
    }

    // Part 2.
    println!("== Part 1 ==");
    'outer_2: for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!(
                        "Found entries: {} + {} + {} == 2020; tell the elves their product is: {}.",
                        nums[i], nums[j], nums[k], nums[i] * nums[j] * nums[k]);
                    break 'outer_2;
                }
            }
        }
    }




}
