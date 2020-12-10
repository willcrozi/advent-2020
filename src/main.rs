mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

fn main() {
    println!("----------------------------------------");
    println!("Day 1, part 1: {}", day_01::part_1().unwrap());
    println!("Day 1, part 2: {}", day_01::part_2().unwrap());

    println!("----------------------------------------");
    println!("Day 2, part 1: {}", day_02::part_1());
    println!("Day 2, part 2: {}", day_02::part_2());

    println!("----------------------------------------");
    println!("Day 3, part 1: {}", day_03::part_1());
    println!("Day 3, part 2: {}", day_03::part_2());

    println!("----------------------------------------");
    println!("Day 4, part 1: {}", day_04::part_1());
    println!("Day 4, part 2: {}", day_04::part_2());

    println!("----------------------------------------");
    println!("Day 5, part 1: {}", day_05::part_1());
    println!("Day 5, part 2: {}", day_05::part_2().unwrap());

    println!("----------------------------------------");
    println!("Day 6, part 1: {}", day_06::part_1());
    println!("Day 6, part 2: {}", day_06::part_2());

    println!("----------------------------------------");
    println!("Day 7, part 1: {}", day_07::part_1());
    println!("Day 7, part 2: {}", day_07::part_2());

    println!("----------------------------------------");
    println!("Day 8, part 1: {}", day_08::part_1());
    println!("Day 8, part 2: {}", day_08::part_2().unwrap());

    println!("----------------------------------------");
    println!("Day 9, part 1: {}", day_09::part_1().unwrap());
    println!("Day 9, part 2: {}", day_09::part_2().unwrap());

    println!("----------------------------------------");
    println!("Day 10, part 1: {}", day_10::part_1());
    println!("Day 10, part 2: {}", day_10::part_2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_01() {
        assert_eq!(day_01::part_1(), Some(1013211));
        assert_eq!(day_01::part_2(), Some(13891280));
    }

    #[test]
    fn day_02() {
        assert_eq!(day_02::part_1(), 622);
        assert_eq!(day_02::part_2(), 263);
    }

    #[test]
    fn day_03() {
        assert_eq!(day_03::part_1(), 278);
        assert_eq!(day_03::part_2(), 9709761600);
    }

    #[test]
    fn day_04() {
        assert_eq!(day_04::part_1(), 206);
        assert_eq!(day_04::part_2(), 123);
    }

    #[test]
    fn day_05() {
        assert_eq!(day_05::part_1(), 980);
        assert_eq!(day_05::part_2(), Some(607));
    }

    #[test]
    fn day_06() {
        assert_eq!(day_06::part_1(), 6506);
        assert_eq!(day_06::part_2(), 3243);
    }

    #[test]
    fn day_07() {
        assert_eq!(day_07::part_1(), 316);
        assert_eq!(day_07::part_2(), 11310);
    }

    #[test]
    fn day_08() {
        assert_eq!(day_08::part_1(), 1675);
        assert_eq!(day_08::part_2(), Some(1532));
    }

    #[test]
    fn day_09() {
        assert_eq!(day_09::part_1(), Some(41682220));
        assert_eq!(day_09::part_2(), Some(5388976));
    }

    #[test]
    fn day_10() {
        assert_eq!(day_10::part_1(), 2592);
        assert_eq!(day_10::part_2(), 198428693313536);
    }
}
