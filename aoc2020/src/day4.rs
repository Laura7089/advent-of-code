use std::collections::HashMap;

const EXPECTED_PASSPORTS: usize = 500;
const EXPECTED_PASSPORT_LENGTH: usize = 150;

pub struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_colour: String,
    eye_colour: String,
    passpord_id: String,
    country_id: Option<String>,
}

pub enum ValidationError {
    MissingField,
    BadFieldFormatting,
    BadLineFormatting,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingField => write!(f, "Missing field"),
            ValidationError::BadFieldFormatting => write!(f, "Bad formatting"),
            ValidationError::BadLineFormatting => write!(f, "Bad line formatting"),
        }
    }
}

impl Passport {
    pub fn try_from_line(line: &String) -> Result<Self, ValidationError> {
        let mut values = HashMap::new();
        for pair in line.split(" ") {
            if pair == "" {
                continue;
            }

            let mut pair_iter = pair.split(":");
            values.insert(
                pair_iter.next().ok_or(ValidationError::BadLineFormatting)?,
                pair_iter
                    .next()
                    .ok_or(ValidationError::BadLineFormatting)?
                    .to_string(),
            );
        }

        Ok(Self {
            birth_year: values.remove("byr").ok_or(ValidationError::MissingField)?,
            issue_year: values.remove("iyr").ok_or(ValidationError::MissingField)?,
            expiration_year: values.remove("eyr").ok_or(ValidationError::MissingField)?,
            height: values.remove("hgt").ok_or(ValidationError::MissingField)?,
            hair_colour: values.remove("hcl").ok_or(ValidationError::MissingField)?,
            eye_colour: values.remove("ecl").ok_or(ValidationError::MissingField)?,
            passpord_id: values.remove("pid").ok_or(ValidationError::MissingField)?,
            country_id: values.remove("cid"),
        })
    }
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

    to_return
}

#[aoc(day4, part1)]
pub fn solve_input_part1(input: &[Result<Passport, ValidationError>]) -> usize {
    input.iter().filter(|p| p.is_ok()).count()
}
