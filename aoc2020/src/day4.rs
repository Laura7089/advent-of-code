pub struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: String,
    hair_colour: String,
    eye_colour: String,
    passpord_id: u32,
    country_id: Option<u32>,
}

impl Passport {
    pub fn try_from_line(line: String) -> Result<Self, String> {
        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_colour = None;
        let mut eye_colour = None;
        let mut passpord_id = None;
        let mut country_id = None;

        for pair in line.split(" ") {
            if pair == "" {
                continue;
            }

            let mut pair_iter = pair.split(":");
            let key = pair_iter.next().expect("Bad pair formatting");
            let val = pair_iter.next().expect("Bad pair formatting");

            match key {
                "byr" => {
                    birth_year = Some(val.parse().or(Err("Bad digit formatting".to_string()))?)
                }
                "iyr" => {
                    issue_year = Some(val.parse().or(Err("Bad digit formatting".to_string()))?)
                }
                "eyr" => {
                    expiration_year = Some(val.parse().or(Err("Bad digit formatting".to_string()))?)
                }
                "hgt" => height = Some(val.to_string()),
                "hcl" => hair_colour = Some(val.to_string()),
                "ecl" => eye_colour = Some(val.to_string()),
                "pid" => {
                    passpord_id = Some(val.parse().or(Err("Bad digit formatting".to_string()))?)
                }
                "cid" => {
                    country_id = Some(val.parse().or(Err("Bad digit formatting".to_string()))?)
                }
                _ => return Err("Unknown key detected".into()),
            }
        }

        Ok(Self {
            birth_year: birth_year.ok_or("No birth year".to_string())?,
            issue_year: issue_year.ok_or("No issue year".to_string())?,
            expiration_year: expiration_year.ok_or("No expiration year".to_string())?,
            height: height.ok_or("No height".to_string())?,
            hair_colour: hair_colour.ok_or("No hair colour".to_string())?,
            eye_colour: eye_colour.ok_or("No eye colour".to_string())?,
            passpord_id: passpord_id.ok_or("No passport id".to_string())?,
            country_id,
        })
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Result<Passport, String>> {
    let mut to_return = Vec::new();
    let mut total_line = String::new();

    for line in input.replace(" ", "\n").lines() {
        if line == "" {
            to_return.push(Passport::try_from_line(dbg!(total_line)));
            total_line = String::new();
        }

        total_line.push_str(line);
        total_line.push_str(" ");
    }

    to_return
}

#[aoc(day4, part1)]
pub fn solve_input_part1(input: &[Result<Passport, String>]) -> usize {
    input.iter().filter(|p| p.is_ok()).count()
}
