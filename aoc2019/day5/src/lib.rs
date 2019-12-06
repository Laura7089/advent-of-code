/// Get (0-based) digit from number
pub fn get_digit(val: isize, digit: u8) -> u8 {
    (val as usize / (10_usize.pow(digit as u32)) % 10) as u8
}

pub enum DataValue {
    PositionMode(usize),
    ImmediateMode(isize),
}

impl DataValue {
    pub fn from(type_int: u8, value: isize) -> Result<DataValue, String> {
        match type_int {
            0 => {
                if value < 0 {
                    return Err(format!(
                        "{} is negative and cannot be used as a positional parameter",
                        value
                    ));
                }
                Ok(DataValue::PositionMode(value as usize))
            }
            1 => Ok(DataValue::ImmediateMode(value)),
            _ => Err(format!(
                "Parameter type int must be 0 or 1, {} received",
                type_int
            )),
        }
    }

    pub fn eval(&self, data: &[isize]) -> Result<isize, String> {
        match self {
            Self::PositionMode(pos) => {
                let length = data.len();
                if *pos as usize >= data.len() {
                    return Err(format!(
                        "The position {} is out of range of this input of length {}",
                        pos, length
                    ));
                }
                Ok(data[*pos as usize])
            }
            Self::ImmediateMode(val) => Ok(*val),
        }
    }
}

pub enum OpCode {
    Add(DataValue, DataValue, DataValue),
    Mult(DataValue, DataValue, DataValue),
    Input(DataValue),
    Output(DataValue),
    Halt,
}

impl OpCode {
    pub fn from_raw(code: &[isize]) -> Result<OpCode, String> {
        match code[0] % 100 {
            1 => Ok(Self::Add(
                DataValue::from(get_digit(code[0], 3), code[1])?,
                DataValue::from(get_digit(code[0], 2), code[2])?,
                DataValue::from(get_digit(code[0], 1), code[3])?,
            )),
            2 => Ok(Self::Mult(
                DataValue::from(get_digit(code[0], 3), code[1])?,
                DataValue::from(get_digit(code[0], 2), code[2])?,
                DataValue::from(get_digit(code[0], 1), code[3])?,
            )),
            3 => Ok(Self::Input(DataValue::from(
                get_digit(code[0], 1),
                code[1],
            )?)),
            4 => Ok(Self::Output(DataValue::from(
                get_digit(code[0], 1),
                code[1],
            )?)),
            99 => Ok(Self::Halt),
            other => Err(format!("Invalid instruction value '{}'", other)),
        }
    }

    pub fn data_len(&self) -> isize {
        match self {
            Self::Add(_, _, _) => 3,
            Self::Mult(_, _, _) => 3,
            Self::Input(_) => 1,
            Self::Output(_) => 1,
            Self::Halt => 0,
        }
    }

    pub fn exec(&self, seq: &mut [isize]) -> Result<Option<()>, String> {
        match self {
            Self::Add(src1, src2, dest) => {
                seq[dest.eval(seq)? as usize] = src1.eval(seq)? + src2.eval(seq)?;
                Ok(Some(()))
            }
            Self::Mult(src1, src2, dest) => {
                seq[dest.eval(seq)? as usize] = src1.eval(seq)? * src2.eval(seq)?;
                Ok(Some(()))
            }
            Self::Input(DataValue::PositionMode(src)) => {
                seq[*src as usize] = 0;
                Ok(Some(()))
            }
            Self::Input(DataValue::ImmediateMode(_)) => {
                Err(String::from("Immediate address type given to input opcode"))
            }
            Self::Output(dest) => {
                println!("{}", dest.eval(seq)?);
                Ok(Some(()))
            }
            Self::Halt => Ok(None),
        }
    }
}

pub fn run_code(code: &mut [isize]) -> Result<(), String> {
    let mut current_pos = 0_usize;
    let mut current_opcode = OpCode::from_raw(code)?;
    while current_opcode.exec(code)? == Some(()) {
        current_pos += current_opcode.data_len() as usize;
        if current_pos >= code.len() {
            return Err(String::from("Iterated outside of code without halting"));
        } else {
            current_opcode = OpCode::from_raw(&code[current_pos..])?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit() {
        assert_eq!(get_digit(12345, 2), 3);
        assert_eq!(get_digit(100, 1), 0);
        assert_eq!(get_digit(123421, 8), 0);
    }
}
