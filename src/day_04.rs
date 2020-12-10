use advent_2020::*;

use std::str::FromStr;

static PASSPORT_DATA: &'static str = include_str!("../data/data_04.txt");

pub fn part_1() -> usize {
    get_passports()
        .filter(Passport::is_valid_pt1)
        .count()
}

pub fn part_2() -> usize {
    get_passports()
        .filter(Passport::is_valid_pt2)
        .count()
}

fn get_passports<'a>() -> impl Iterator<Item=Passport<'a>> {
    PASSPORT_DATA.paragraphs(true)
        .map(|para| Passport::parse(para))

}

#[derive(Clone, Debug)]
struct Passport<'s> {
    byr: Option<&'s str>,  // (Birth Year)
    iyr: Option<&'s str>,  // (Issue Year)
    eyr: Option<&'s str>,  // (Expiration Year)
    hgt: Option<&'s str>,  // (Height)
    hcl: Option<&'s str>,  // (Hair Color)
    ecl: Option<&'s str>,  // (Eye Color)
    pid: Option<&'s str>,  // (Passport ID)
    cid: Option<&'s str>,  // (Country ID)

    parse_error: bool,
}

impl<'s> Passport<'s> {

    fn parse(input: &'s str) -> Passport {
        let mut pp = Passport {
            byr: None, iyr: None, eyr: None, hgt: None,
            hcl: None, ecl: None, pid: None, cid: None,
            parse_error: false,
        };

        input.split_whitespace()
            .for_each(|entry| pp.parse_field(entry));

        pp

    }

    fn is_valid_pt1(&self) -> bool {
        !self.parse_error
            && self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    /// Validation for part 2:
    ///
    ///     - byr (Birth Year) - four digits; at least 1920 and at most 2002.
    ///     - iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    ///     - eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    ///     - hgt (Height) - a number followed by either cm or in:
    ///     -     If cm, the number must be at least 150 and at most 193.
    ///     -     If in, the number must be at least 59 and at most 76.
    ///     - hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ///     - ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    ///     - pid (Passport ID) - a nine-digit number, including leading zeroes.
    ///     - cid (Country ID) - ignored, missing or not.
    ///
    fn is_valid_pt2(&self) -> bool {
        let mut valid = true;

        valid &= self.byr.is_some_and_valid(|ref year| (1920..=2002).contains(year));

        valid &= self.iyr.is_some_and_valid(|ref year| (2010..=2020).contains(year));

        valid &= self.eyr.is_some_and_valid(|ref year| (2020..=2030).contains(year));

        valid &= self.hgt.as_ref().map(|hgt| {
            hgt.split_when(char::is_alphabetic)
                .map(|(qty, units)| {
                    let qty = u32::from_str(qty);
                    match (qty, units) {
                        (Ok(ref num), "in") => (59..=76).contains(num),
                        (Ok(ref num), "cm") => (150..=193).contains(num),
                        _ => false,
                    }
                }).unwrap_or(false)
        }).unwrap_or(false);

        valid &= self.hcl.as_ref().map(|hcl| {
            let mut chars = hcl.chars();
            chars.next() == Some('#')
                && chars.fold(
                (0, true), |(len, is_hex), c| (len + 1, is_hex && c.is_ascii_hexdigit())) == (6, true)
        }).unwrap_or(false);

        valid &= self.ecl.map(|ecl|
            match ecl {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            }
        ).unwrap_or(false);

        valid &= self.pid.as_ref().map(|pid|
            pid.chars().fold(
                (0, true), |(len, numeric), c| (len + 1, numeric && c.is_ascii_digit())
            ) == (9, true)
        ).unwrap_or(false);

        valid
    }

    fn parse_field(&mut self, entry: &'s str) {
        if let Some((key, val)) = entry.split_once_(":") {
            let field = match key {
                "byr" => &mut self.byr,
                "iyr" => &mut self.iyr,
                "eyr" => &mut self.eyr,
                "hgt" => &mut self.hgt,
                "hcl" => &mut self.hcl,
                "ecl" => &mut self.ecl,
                "pid" => &mut self.pid,
                "cid" => &mut self.cid,
                _ => {
                    self.parse_error = true;
                    return
                },
            };

            if field.is_some() { self.parse_error = true; }

            *field = Some(val);
        } else {
            // Field format not recognised.
            self.parse_error = true;
        }
    }
}
