use std::str::FromStr;
use std::collections::BTreeMap;

static ADAPTERS: &'static str = include_str!("../data/data_10.txt");

fn get_joltages() -> Vec<i32> {
    let mut joltages = ADAPTERS.lines()
        .map(|jolts| i32::from_str(jolts).unwrap())
        .collect::<Vec<_>>();

    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);

    joltages
}

pub fn part_1() -> u32 {
    let joltages = get_joltages();

    let result = joltages.into_iter()
        .fold((0, [0; 4]), |mut acc, next| {
            acc.1[(next - acc.0) as usize] += 1;
            (next, acc.1)
        });

    return result.1[1] * result.1[3];
}

pub fn part_2() -> usize {
    let joltages = get_joltages();
    let mut map: BTreeMap<(i32, usize), usize> = BTreeMap::new();

    count_subcombos((0, 0), &joltages[..], &mut map)
}

fn count_subcombos(combo: (i32, usize), joltages: &[i32], map: &mut BTreeMap<(i32, usize), usize>) -> usize {
    let (end, next) = combo;

    if next == joltages.len() {
        return 1;
    }

    if let Some(count) = map.get(&(end, next)) {
        *count
    } else {
        let mut count = 0;
        for i in next..joltages.len() {
            if joltages[i] - end <= 3 {
                count += count_subcombos((joltages[i], i + 1), joltages, map);
            }
        }

        map.insert((end, next), count);

        count
    }
}
