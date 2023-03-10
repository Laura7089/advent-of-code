use std::collections::HashMap;

// Used to initialise buffers
const EXPECTED_PASSPORTS: usize = 500;
const EXPECTED_PASSPORT_LENGTH: usize = 150;

const BIRTH_YEAR_LIMITS: (u32, u32) = (1920, 2002);
const ISSUE_YEAR_LIMITS: (u32, u32) = (2010, 2020);
const EXPR_YEAR_LIMITS: (u32, u32) = (2020, 2030);

#[derive(Debug, PartialEq)]
pub struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: Height,
    hair_colour: u32,
    eye_colour: String,
    passport_id: u32,
    country_id: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Height {
    Inches(u32),
    Centimetres(u32),
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    MissingField(String),
    BadFieldFormatting(String),
    BadLineFormatting,
}

use ValidationError::*;

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissingField(_) => write!(f, "Missing field"),
            BadFieldFormatting(_) => write!(f, "Bad formatting"),
            BadLineFormatting => write!(f, "Bad line formatting"),
        }
    }
}

impl Passport {
    pub fn try_from_line(line: &str) -> Result<Self, ValidationError> {
        // Collect all values into a hashmap
        let mut values = HashMap::new();
        for pair in line.split(" ") {
            if pair == "" {
                continue;
            }
            values.insert(&pair[0..3], pair[4..].to_string());
        }

        // Here comes the enormous, horrible, no-good input validation block :(
        Ok(Self {
            birth_year: year_helper(&mut values, "byr", BIRTH_YEAR_LIMITS)?,
            issue_year: year_helper(&mut values, "iyr", ISSUE_YEAR_LIMITS)?,
            expiration_year: year_helper(&mut values, "eyr", EXPR_YEAR_LIMITS)?,
            height: {
                let raw = values.remove("hgt").ok_or(MissingField("hgt".into()))?;
                // Parse as a number, avoiding the units
                let num = &raw[..raw.len() - 2]
                    .parse::<u32>()
                    .or(Err(BadFieldFormatting("hgt".into())))?;
                // Figure out the units and ensure it's within acceptable bounds
                match &raw[raw.len() - 2..raw.len()] {
                    "cm" if num > &149 || num < &194 => Height::Centimetres(*num),
                    "in" if num > &58 || num < &77 => Height::Inches(*num),
                    _ => return Err(BadFieldFormatting("hgt".into())),
                }
            },
            hair_colour: {
                let raw = values.remove("hcl").ok_or(MissingField("hcl".into()))?;
                let raw_bytes = raw.as_bytes();
                // Make sure it's the right length and has a "#"
                if raw_bytes[0] != '#' as u8 && raw_bytes.len() != 7 {
                    return Err(BadFieldFormatting("hcl".into()));
                }
                // Parse as a hex number
                u32::from_str_radix(&raw[1..], 16).or(Err(BadFieldFormatting("hcl".into())))?
            },
            eye_colour: {
                let raw = values.remove("ecl").ok_or(MissingField("ecl".into()))?;
                // Just check if it's in the acceptable list
                match raw.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => raw,
                    _ => return Err(BadFieldFormatting("hcl".into())),
                }
            },
            passport_id: {
                let raw = values.remove("pid").ok_or(MissingField("pid".into()))?;
                // Check length then parse and return
                if raw.len() != 9 {
                    return Err(BadFieldFormatting("pid".into()));
                }
                raw.parse().or(Err(BadFieldFormatting("pid".into())))?
            },
            country_id: values.remove("cid"),
        })
    }
}

fn year_helper(
    map: &mut HashMap<&str, String>,
    key: &str,
    (lower, upper): (u32, u32),
) -> Result<u32, ValidationError> {
    // Pop the value out and parse it as an int
    let num = map
        .remove(key)
        .ok_or(MissingField(key.into()))?
        .parse()
        .or(Err(BadFieldFormatting(key.into())))?;
    // Check our bounds
    if num > upper || num < lower {
        return Err(BadFieldFormatting(key.into()));
    }
    Ok(num)
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Result<Passport, ValidationError>> {
    let mut to_return = Vec::with_capacity(EXPECTED_PASSPORTS);
    let mut total_line = String::with_capacity(EXPECTED_PASSPORT_LENGTH);

    for line in input.lines() {
        if line == "" {
            to_return.push(Passport::try_from_line(&total_line));
            total_line.clear();
        }

        total_line.push_str(line);
        total_line.push_str(" ");
    }
    to_return.push(Passport::try_from_line(&total_line));

    to_return
}

#[aoc(day4, part1)]
pub fn solve_input_part1(input: &[Result<Passport, ValidationError>]) -> usize {
    input
        .iter()
        .filter(|p| match p {
            Ok(_) => true,
            Err(BadFieldFormatting(_)) => true,
            _ => false,
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_input_part2(input: &[Result<Passport, ValidationError>]) -> usize {
    input.iter().filter(|p| p.is_ok()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn parser_example_part1() {
        assert_eq!(
            parse_input(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            ),
            vec![
                Ok(Passport {
                    birth_year: 1937,
                    issue_year: 2017,
                    expiration_year: 2020,
                    height: Height::Centimetres(183),
                    hair_colour: 16777213,
                    eye_colour: "gry".into(),
                    passport_id: 860033327,
                    country_id: Some("147".into()),
                }),
                Err(ValidationError::MissingField("hgt".into())),
                Ok(Passport {
                    birth_year: 1931,
                    issue_year: 2013,
                    expiration_year: 2024,
                    height: Height::Centimetres(179),
                    hair_colour: 11409377,
                    eye_colour: "brn".into(),
                    passport_id: 760753108,
                    country_id: None,
                }),
                Err(ValidationError::MissingField("byr".into())),
            ]
        );
    }

    #[test_case("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f", 1980, 2012, 2030, Height::Inches(74), 6437423, "grn", 087499704, None; "1")]
    fn parse_valid_passport(
        input: &str,
        byr: u32,
        iyr: u32,
        eyr: u32,
        hgt: Height,
        hcl: u32,
        ecl: &str,
        pid: u32,
        cid: Option<String>,
    ) {
        assert_eq!(
            parse_input(input),
            vec![Ok(Passport {
                birth_year: byr,
                issue_year: iyr,
                expiration_year: eyr,
                height: hgt,
                hair_colour: hcl,
                eye_colour: ecl.into(),
                passport_id: pid,
                country_id: cid
            })]
        );
    }

    #[test_case("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926", ValidationError::BadFieldFormatting("eyr".into()); "1")]
    #[test_case("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946", ValidationError::BadFieldFormatting("eyr".into()); "2")]
    #[test_case("hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:27", ValidationError::BadFieldFormatting("hcl".into()); "3")]
    #[test_case("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007", ValidationError::BadFieldFormatting("byr".into()); "4")]
    fn parse_invalid_passport(input: &str, err: ValidationError) {
        assert_eq!(parse_input(input), vec![Err(err)]);
    }

    #[test]
    fn my_data_part1() {
        assert_eq!(
            solve_input_part1(&parse_input(
                &std::fs::read_to_string("./input/2020/day4.txt").unwrap()
            )),
            260
        );
    }
}
