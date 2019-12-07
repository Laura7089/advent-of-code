use std::io::Stdin;

type DataPoint = i32;

/// Get (0-based) digit from number
pub fn get_digit(val: DataPoint, digit: u32) -> u8 {
    (val as usize / (10_usize.pow(digit)) % 10) as u8
}

pub enum OpCode {
    Add,
    Mult,
    Input,
    Output,
    Halt,
}

impl OpCode {
    pub fn from_raw(code: DataPoint) -> Result<OpCode, String> {
        match code % 100 {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mult),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            99 => Ok(Self::Halt),
            other => Err(format!("Invalid instruction value: {}", other)),
        }
    }

    pub fn data_len(&self) -> usize {
        match self {
            Self::Add => 4,
            Self::Mult => 4,
            Self::Input => 2,
            Self::Output => 2,
            Self::Halt => 1,
        }
    }
}

pub enum ComputerState {
    Continue,
    Halt,
    Error(String),
}

pub struct IntcodeComputer {
    pub mem: Vec<DataPoint>,
    input: Stdin,
    i_pointer: usize,
    pub state: ComputerState,
    last_instruction: OpCode,
}

impl IntcodeComputer {
    /// Get a new IntcodeComputer with the given memory state
    pub fn new(mem: Vec<DataPoint>, input: Stdin) -> IntcodeComputer {
        IntcodeComputer {
            mem: mem,
            input: input,
            i_pointer: 0,
            state: ComputerState::Continue,
            last_instruction: OpCode::Halt,
        }
    }

    /// Set state to error and return Err(err_msg)
    fn error(&mut self, err_msg: String) -> Result<(), String> {
        self.state = ComputerState::Error(err_msg.clone());
        Err(err_msg.clone())
    }

    /// Get argument of an instruction at pointer, 1-based
    fn get_arg(&mut self, arg_num: usize) -> Result<DataPoint, String> {
        let ins = self.mem[self.i_pointer];
        // Catch stuff trying to call args outside our mem
        if self.i_pointer + arg_num >= self.mem.len() {
            self.error(format!(
                "Instruction {} at {} requested arg out of memory",
                ins, self.i_pointer
            ))?;
        }
        match get_digit(ins, (arg_num + 1) as u32) {
            0 => Ok(self.mem[self.mem[self.i_pointer + arg_num] as usize]),
            1 => Ok(self.mem[self.i_pointer + arg_num]),
            _ => match self
                .error(format!(
                    "Non-0/1 digit in parameter section of instruction: {}",
                    ins
                ))
                .err()
            {
                Some(ref err_msg) => Err(err_msg.clone()),
                None => Err(String::new()),
            },
        }
    }

    /// Retrieve the next opcode on the computer
    fn get_opcode(&mut self) -> Result<&OpCode, String> {
        // Make sure we haven't iterated out of mem
        let mem_len = self.mem.len();
        if self.i_pointer >= mem_len {
            self.error(String::from(
                "Program iterated out of memory without halting",
            ))?;
        }
        // Set the next instruction
        let ins = self.mem[self.i_pointer];
        self.last_instruction = OpCode::from_raw(ins)?;
        Ok(&self.last_instruction)
    }

    /// Step forward one instruction
    pub fn step(&mut self) -> Result<&ComputerState, String> {
        match self.state {
            ComputerState::Continue => {
                // Increment pointer
                self.i_pointer += self.last_instruction.data_len();
                self.get_opcode()?;
                // Execute instruction
                match self.last_instruction {
                    OpCode::Add => {
                        self.mem[self.mem[self.i_pointer + 3] as usize] =
                            self.get_arg(1)? + self.get_arg(2)?;
                    }
                    OpCode::Mult => {
                        self.mem[self.mem[self.i_pointer + 3] as usize] =
                            self.get_arg(1)? * self.get_arg(2)?;
                    }
                    OpCode::Input => {
                        let mut result: String = String::new();
                        self.input.read_line(&mut result).unwrap();
                        match result.parse() {
                            Ok(int) => self.mem[self.i_pointer + 1] = int,
                            Err(_) => {
                                self.error(format!("Malformatted input given: {}", result))?
                            }
                        }
                    }
                    OpCode::Output => println!("{}", self.get_arg(1)?),
                    OpCode::Halt => self.state = ComputerState::Halt,
                }
                Ok(&self.state)
            }
            // Can't step a halted or error'd computer
            ComputerState::Halt => Err(String::from("Computer is halted")),
            ComputerState::Error(ref msg) => Err(msg.clone()),
        }
    }
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
