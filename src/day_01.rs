use std::str::FromStr;

static EXPENSE_DATA: &'static str = include_str!("../data/data_01.txt");

pub fn part_1() -> Option<u32> {
    let nums = parse_expenses();

    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 { return Some(nums[i] * nums[j]); }
        }
    }
    None
}

pub fn part_2() -> Option<u32> {
    let nums = parse_expenses();

    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 { return Some(nums[i] * nums[j] * nums[k]) }
            }
        }
    }
    None
}

pub fn parse_expenses() -> Vec<u32> {
    EXPENSE_DATA.lines()
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<_>>()
}
