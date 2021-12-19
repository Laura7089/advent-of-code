#[derive(Debug, Clone, PartialEq)]
pub struct BITSPacketVersioned {
    pub version: u8,
    pub packet: BITSPacket,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BITSPacket {
    Literal(usize),
    Operator(BITSOperator, Vec<BITSPacketVersioned>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BITSOperator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

pub fn get_bits(hex_raw: &str) -> Vec<bool> {
    hex_raw
        .bytes()
        .map(|c| match c {
            // TODO: this is kinda awful, but actual bit manipulation
            // is difficult because the input isn't byte-aligned
            b'0' => [false, false, false, false],
            b'1' => [false, false, false, true],
            b'2' => [false, false, true, false],
            b'3' => [false, false, true, true],
            b'4' => [false, true, false, false],
            b'5' => [false, true, false, true],
            b'6' => [false, true, true, false],
            b'7' => [false, true, true, true],
            b'8' => [true, false, false, false],
            b'9' => [true, false, false, true],
            b'A' => [true, false, true, false],
            b'B' => [true, false, true, true],
            b'C' => [true, true, false, false],
            b'D' => [true, true, false, true],
            b'E' => [true, true, true, false],
            b'F' => [true, true, true, true],
            _ => panic!("Non-hex character in input"),
        })
        .flatten()
        .collect()
}

pub fn bits_to_num(raw: &[bool]) -> usize {
    raw.iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (*b as usize) << i)
        .sum()
}

pub fn format_bits(bits: &[bool]) -> String {
    let bits_str: Vec<String> = bits
        .iter()
        .map(|b| if *b { "1" } else { "0" }.into())
        .collect();

    bits_str.join("")
}

pub fn parse(bits: &[bool]) -> (BITSPacketVersioned, usize) {
    let version: u8 = bits_to_num(&bits[0..3]) as u8;
    let type_id: u8 = bits_to_num(&bits[3..6]) as u8;

    #[cfg(test)]
    println!(
        "{}: version {}, type {}",
        format_bits(bits),
        version,
        type_id
    );

    // Keeps track of what bit in the sequence we're looking at
    let mut ctr = 6;

    let packet_type = match type_id {
        // Literal value
        4 => BITSPacket::Literal({
            // TODO: change me to a fixed-size buffer?
            let mut lit_groups = Vec::with_capacity(24);

            while let [true, vals @ ..] = &bits[ctr..(ctr + 5)] {
                lit_groups.extend_from_slice(vals);
                ctr += 5;
            }

            // NOTE: skips the first bit because we know it's 0
            lit_groups.extend_from_slice(&bits[(ctr + 1)..(ctr + 5)]);
            ctr += 5;

            bits_to_num(&lit_groups)
        }),
        // Operator
        op => {
            let op_type = match op {
                0 => BITSOperator::Sum,
                1 => BITSOperator::Product,
                2 => BITSOperator::Minimum,
                3 => BITSOperator::Maximum,
                5 => BITSOperator::Greater,
                6 => BITSOperator::Less,
                7 => BITSOperator::Equal,
                _ => panic!("Unknown opcode: {}", op),
            };

            let subpackets = if bits[6] {
                // Next 11 bits is the number of subpackets
                let num_subpackets = bits_to_num(&bits[7..18]);
                ctr += 12;

                #[cfg(test)]
                println!(
                    "Detected operator {} with {} subpackets",
                    op, num_subpackets
                );

                let mut subpackets = Vec::with_capacity(num_subpackets);

                while subpackets.len() < num_subpackets {
                    let (subp, subplen) = parse(&bits[ctr..]);

                    ctr += subplen;
                    subpackets.push(subp);
                }

                subpackets
            } else {
                // Next 15 bits is length of subpackets
                let len_subpackets = bits_to_num(&bits[7..22]);
                ctr += 16;
                let sub_start = ctr;

                #[cfg(test)]
                println!(
                    "Detected operator {} with {} BITS of subpackets",
                    op, len_subpackets
                );

                let mut subpackets = Vec::with_capacity(10);
                while ctr - sub_start < len_subpackets {
                    let (subp, subplen) = parse(&bits[ctr..]);

                    ctr += subplen;
                    subpackets.push(subp);
                }

                subpackets
            };

            BITSPacket::Operator(op_type, subpackets)
        }
    };

    (
        BITSPacketVersioned {
            version,
            packet: packet_type,
        },
        // Number of bits we've consumed
        ctr,
    )
}

impl BITSPacketVersioned {
    pub fn eval(&self) -> usize {
        use BITSOperator::*;
        use BITSPacket::*;

        match &self.packet {
            Literal(l) => *l,
            Operator(Sum, subs) => subs.iter().map(Self::eval).sum(),
            Operator(Product, subs) => subs.iter().map(Self::eval).product(),
            Operator(Minimum, subs) => subs.iter().map(Self::eval).min().unwrap(),
            Operator(Maximum, subs) => subs.iter().map(Self::eval).max().unwrap(),
            Operator(Greater, subs) => (subs[0].eval() > subs[1].eval()) as usize,
            Operator(Less, subs) => (subs[0].eval() < subs[1].eval()) as usize,
            Operator(Equal, subs) => (subs[0].eval() == subs[1].eval()) as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_literal() {
        let example_input = "D2FE28";
        let bits = get_bits(example_input);

        let expected = BITSPacketVersioned {
            version: 6,
            packet: BITSPacket::Literal(2021),
        };

        assert_eq!(parse(&bits).0, expected);
    }

    #[test]
    fn example_operator() {
        let example_input = "38006F45291200";
        let bits = get_bits(example_input);

        let expected = BITSPacketVersioned {
            version: 1,
            packet: BITSPacket::Operator(
                BITSOperator::Less,
                vec![
                    BITSPacketVersioned {
                        version: 6,
                        packet: BITSPacket::Literal(10),
                    },
                    BITSPacketVersioned {
                        version: 2,
                        packet: BITSPacket::Literal(20),
                    },
                ],
            ),
        };

        assert_eq!(parse(&bits).0, expected);
    }
}
