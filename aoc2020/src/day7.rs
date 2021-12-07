#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Bag {
    colour: String,
    sub_bags: Option<Vec<(Bag, usize)>>,
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<Bag> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(" bags contain ");
            let root_bag_colour = line_iter.next().expect("Bad line formatting").to_string();
            let sub_bags_raw = line_iter.next().expect("Bad line formatting");
            if &sub_bags_raw[..2] == "no" {
                Bag {
                    colour: root_bag_colour,
                    sub_bags: None,
                }
            } else {
                Bag {
                    colour: root_bag_colour,
                    sub_bags: Some(
                        sub_bags_raw[0..sub_bags_raw.len() - 1]
                            .split(", ")
                            .map(|sub_bag| {
                                let mut sub_bag_split = sub_bag.split(" ");
                                let num = sub_bag_split
                                    .next()
                                    .expect("Bad line formatting")
                                    .parse()
                                    .expect("Bad line formatting");
                                let bag = Bag {
                                    colour: (sub_bag_split
                                        .next()
                                        .expect("Bad line formatting")
                                        .to_string()
                                        + " "
                                        + sub_bag_split.next().expect("Bad line formatting"))
                                    .to_string(),
                                    sub_bags: None,
                                };
                                (bag, num)
                            })
                            .collect(),
                    ),
                }
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn solve_input_part1(input: &[Bag]) -> usize {
    println!("{:?}", input);
    0
}
