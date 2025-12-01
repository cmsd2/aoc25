use std::io::{self};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    sequence::pair,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Nom error: {0}")]
    NomError(String),
}

pub type AocResult<R> = std::result::Result<R, AocError>;

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
    pub fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction {
                operation: Operation::Left,
                argument: count,
            } => {
                while count > self.num {
                    self.num += 100;
                }
                self.num -= count
            }
            Instruction {
                operation: Operation::Right,
                argument: count,
            } => self.num = (self.num + count) % 100,
        }
    }

    pub fn apply_multiple(&mut self, instructions: Vec<Instruction>) -> u32 {
        let mut zero_count = 0;
        for instruction in instructions {
            self.apply(instruction);
            if self.num == 0 {
                zero_count += 1;
            }
        }
        zero_count
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

fn main() {
    println!("Hello, world!");
    let input = read_file("data/input.txt").expect("Failed to read input file");
    let instructions = input
        .lines()
        .map(parse)
        .collect::<std::result::Result<Vec<Instruction>, AocError>>()
        .expect("Error parsing input");
    let mut state = State { num: 50 };
    let zero_count = state.apply_multiple(instructions);
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
        read_file("data/test_input.txt").expect("Failed to read test input file")
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
        let input = read_test_file();
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 10);
        let _instructions = lines
            .into_iter()
            .map(parse)
            .collect::<std::result::Result<Vec<Instruction>, AocError>>()
            .expect("Error parsing test input");
    }

    #[test]
    fn test_apply_instruction() {
        let mut state = State { num: 50 };
        state.apply(Instruction {
            operation: Operation::Left,
            argument: 68,
        });
        assert_eq!(state, State { num: 82 });
    }

    #[test]
    fn test_apply_test_data() {
        let mut state = State { num: 50 };
        let input = read_test_file();
        let instructions = input
            .lines()
            .map(parse)
            .collect::<std::result::Result<Vec<Instruction>, AocError>>()
            .expect("Error parsing test input");
        let zero_count = state.apply_multiple(instructions);
        assert_eq!(zero_count, 3);
    }
}
