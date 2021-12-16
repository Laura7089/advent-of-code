#[derive(Debug, Clone, PartialEq)]
pub struct BITSPacketVersioned {
    version: u8,
    packet: BITSPacket,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BITSPacket {
    Literal(usize),
    Operator(BITSOperator, Vec<BITSPacketVersioned>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BITSOperator {
    Unknown,
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
    raw.into_iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (*b as usize) << i)
        .sum()
}

impl BITSPacketVersioned {
    pub fn parse(bits: &[bool]) -> (Self, usize) {
        let version: u8 = bits_to_num(&bits[0..3]) as u8;
        let type_id: u8 = bits_to_num(&bits[3..6]) as u8;

        #[cfg(test)]
        println!(
            "Parsing packet {:?}, got version {} and type {}",
            bits.iter().map(|b| *b as usize).collect::<Vec<_>>(),
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
            _op => BITSPacket::Operator(BITSOperator::Unknown, {
                if bits[6] {
                    // Next 11 bits is the number of subpackets
                    let num_subpackets = bits_to_num(&bits[7..18]);
                    ctr = 18;

                    #[cfg(test)]
                    println!(
                        "Detected operator packet with {} subpackets",
                        num_subpackets
                    );

                    let mut subpackets = Vec::with_capacity(num_subpackets);

                    subpackets
                } else {
                    // Next 15 bits is length of subpackets
                    let len_subpackets = bits_to_num(&bits[7..22]);
                    ctr = 22;

                    #[cfg(test)]
                    println!(
                        "Detected operator packet with {} BITS of subpackets",
                        len_subpackets
                    );

                    let mut subpackets = Vec::with_capacity(10);
                    while ctr < len_subpackets {
                        let (subp, subplen) = BITSPacketVersioned::parse(&bits[ctr..]);

                        ctr += subplen;
                        subpackets.push(subp);
                    }

                    subpackets
                }
            }),
        };

        (
            Self {
                version,
                packet: packet_type,
            },
            // Number of bits we've consumed
            ctr - 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_literal() {
        let example_input = "D2FE28";
        let bits = get_bits(example_input);

        let expected = BITSPacket {
            version: 6,
            packet: BITSPacketType::Literal(2021),
        };

        assert_eq!(BITSPacket::parse(&bits).0, expected);
    }

    #[test]
    fn example_operator() {
        let example_input = "38006F45291200";
        let bits = get_bits(example_input);

        let expected = BITSPacket {
            version: 1,
            packet: BITSPacket::Operator(
                BITSOperator::Unknown,
                vec![
                    BITSPacket {
                        version: 1,
                        packet: BITSPacketType::Literal(10),
                    },
                    BITSPacket {
                        version: 1,
                        packet: BITSPacketType::Literal(20),
                    },
                ],
            ),
        };
    }
}
