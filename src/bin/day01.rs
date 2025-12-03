use aoc25::error::AocError;
use aoc25::result::AocResult;
use std::fmt::{self};
use std::io::{self};

use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1,
    combinator::map_res, sequence::pair,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    CountZerosAfterRotation,
    CountZerosDuringRotation,
}

impl From<&str> for Mode {
    fn from(s: &str) -> Self {
        match s {
            "after" => Mode::CountZerosAfterRotation,
            "during" => Mode::CountZerosDuringRotation,
            _ => Mode::CountZerosAfterRotation,
        }
    }
}

#[derive(clap::Parser, Debug, Clone)]
pub struct Config {
    #[clap(
        short,
        long,
        default_value = "data/day01/input.txt",
        help = "Path to input file"
    )]
    pub input: String,

    #[clap(
        short,
        long,
        default_value = "after",
        help = "Mode: 'after' or 'during'"
    )]
    pub mode: Mode,

    #[clap(short, long, help = "Enable verbose output")]
    pub verbose: bool,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub num: u32,
}

impl State {
    pub fn new() -> Self {
        State { num: 50 }
    }

    pub fn apply(&mut self, instruction: Instruction, mode: Mode, verbose: bool) -> u32 {
        let mut zeros = 0;
        match instruction {
            Instruction {
                operation: Operation::Left,
                argument: count,
            } => {
                while count > self.num {
                    if self.num != 0 {
                        zeros += 1;
                    }
                    self.num += 100;
                }
                self.num -= count;
            }
            Instruction {
                operation: Operation::Right,
                argument: count,
            } => {
                self.num += count;
                zeros += self.num / 100;
                self.num %= 100;
                if self.num == 0 {
                    zeros -= 1;
                }
            }
        }
        if verbose {
            print!(
                "- The dial is rotated {} to point at {}",
                instruction, self.num
            );
            if mode == Mode::CountZerosDuringRotation && zeros > 0 {
                print!("; during this rotation, it points at 0 {} times", zeros);
            }
            println!(".");
        }
        zeros
    }

    pub fn apply_multiple(
        &mut self,
        instructions: Vec<Instruction>,
        mode: Mode,
        verbose: bool,
    ) -> u32 {
        let mut zeros_after = 0;
        let mut zeros_during = 0;
        for instruction in instructions {
            zeros_during += self.apply(instruction, mode, verbose);
            if self.num == 0 {
                zeros_after += 1;
            }
        }
        if mode == Mode::CountZerosDuringRotation {
            zeros_during + zeros_after
        } else {
            zeros_after
        }
    }
}

impl Operation {
    pub fn from_str(op: &str) -> Option<Self> {
        match op {
            "L" => Some(Operation::Left),
            "R" => Some(Operation::Right),
            _ => None,
        }
    }
}

pub struct Instruction {
    pub operation: Operation,
    pub argument: u32,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self.operation {
            Operation::Left => "L",
            Operation::Right => "R",
        };
        write!(f, "{}{}", op_str, self.argument)
    }
}

impl Instruction {
    pub fn new(op: Operation, count: u32) -> Self {
        Instruction {
            operation: op,
            argument: count as u32,
        }
    }
}

fn read_file(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

fn read_instructions_file(path: &str) -> AocResult<Vec<Instruction>> {
    let content = read_file(path).map_err(|e| AocError::ParseError(e.to_string()))?;
    let instructions = content
        .lines()
        .map(parse)
        .collect::<std::result::Result<Vec<Instruction>, AocError>>()?;
    Ok(instructions)
}

fn main() {
    use clap::Parser;
    let args = Config::parse();
    let instructions = read_instructions_file(&args.input).expect("Failed to read input file");
    let mut state = State::new();
    let zero_count = state.apply_multiple(instructions, args.mode, args.verbose);
    println!("Zero count: {}", zero_count);
}

fn parse_op(input: &str) -> IResult<&str, Operation> {
    alt((
        tag("L").map_opt(|_| Some(Operation::Left)),
        tag("R").map_opt(|_| Some(Operation::Right)),
    ))
    .parse(input)
}

fn parse_count(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse).parse(input)
}
fn parse_instruction(input: &str) -> IResult<&str, (Operation, u32)> {
    pair(parse_op, parse_count).parse(input)
}

fn parse(line: &str) -> std::result::Result<Instruction, AocError> {
    let (_remainder, (op, count)) = parse_instruction(line)
        .map_err(|e| AocError::NomError(format!("error parsing '{}', {}", line, e)))?;

    Ok(Instruction::new(op, count))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_file() -> String {
        read_file("data/day01/test_input.txt").expect("Failed to read test input file")
    }

    fn read_test_instructions() -> Vec<Instruction> {
        read_instructions_file("data/day01/test_input.txt").expect("Failed to read test input file")
    }

    #[test]
    fn test_read_file() {
        let _ = read_test_file();
    }

    #[test]
    fn test_parse_op() {
        let (_remainder, o) = parse_op("L").expect("parser");
        assert_eq!(o, Operation::Left);
    }

    #[test]
    fn test_parse_count() {
        let (_remainder, count) = parse_count("123").expect("parser");
        assert_eq!(count, 123);
    }

    #[test]
    fn test_parse_instruction() {
        let (remainder, ell) = parse_instruction("L8").expect("parser");
        assert_eq!(remainder, "");
        assert_eq!(ell, (Operation::Left, 8));
    }

    #[test]
    fn test_parse_instructions() {
        let instructions = read_test_instructions();
        assert_eq!(instructions.len(), 10);
    }

    #[test]
    fn test_apply_instruction() {
        let mut state = State::new();
        state.apply(
            Instruction {
                operation: Operation::Left,
                argument: 68,
            },
            Mode::CountZerosAfterRotation,
            false,
        );
        assert_eq!(state, State { num: 82 });
    }

    #[test]
    fn test_apply_test_data() {
        let mut state = State::new();
        let instructions = read_test_instructions();
        let zero_count = state.apply_multiple(instructions, Mode::CountZerosAfterRotation, false);
        assert_eq!(zero_count, 3);
    }

    #[test]
    fn test_apply_instruction_count_during() {
        let mut state = State::new();
        let zero_count = state.apply(
            Instruction {
                operation: Operation::Left,
                argument: 68,
            },
            Mode::CountZerosAfterRotation,
            false,
        );
        assert_eq!(zero_count, 1);
    }

    #[test]
    fn test_apply_test_data_count_during() {
        let mut state = State::new();
        let instructions = read_test_instructions();
        let zero_count = state.apply_multiple(instructions, Mode::CountZerosDuringRotation, false);
        assert_eq!(zero_count, 6);
    }

    #[test]
    fn test_big_rotation() {
        let mut state = State::new();
        let zero_count = state.apply(
            Instruction {
                operation: Operation::Right,
                argument: 1000,
            },
            Mode::CountZerosAfterRotation,
            false,
        );
        assert_eq!(state.num, 50);
        assert_eq!(zero_count, 10);
    }

    #[test]
    fn test_fiddly_bits() {
        let cases = vec![
            (Operation::Left, 5, 5, 0, 0),
            (Operation::Right, 5, 95, 0, 0),
            (Operation::Left, 5, 0, 95, 0),
            (Operation::Right, 5, 95, 0, 0),
            (Operation::Right, 5, 0, 5, 0),
            (Operation::Left, 100, 5, 5, 1),
            (Operation::Right, 100, 5, 5, 1),
        ];
        let mut state = State::new();
        let mode = Mode::CountZerosAfterRotation;
        for (op, arg, num, expected_num, expected_zeros) in cases {
            state.num = num;
            let zero_count = state.apply(
                Instruction {
                    operation: op,
                    argument: arg,
                },
                mode,
                false,
            );
            assert_eq!(state.num, expected_num);
            assert_eq!(zero_count, expected_zeros);
        }
    }
}
