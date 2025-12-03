use core::fmt;
use std::cmp::Ordering;

use aoc25::error::AocError;
use aoc25::result::AocResult;
use log::{debug, info};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Two,
    Twelve,
}

impl From<&str> for Mode {
    fn from(s: &str) -> Self {
        match s {
            "two" => Mode::Two,
            "twelve" => Mode::Twelve,
            _ => Mode::Two,
        }
    }
}

#[derive(clap::Parser, Debug, Clone)]
pub struct Config {
    #[clap(
        short,
        long,
        default_value = "data/day03/input.txt",
        help = "Path to input file"
    )]
    pub input: String,

    #[clap(short, long, default_value = "two", help = "Mode: 'two' or 'twelve'")]
    pub mode: Mode,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn max_char(s: &str) -> AocResult<(usize, char)> {
    s.chars()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            if a >= b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .ok_or(AocError::ParseError(format!("max_char: {}", s)))
}

#[derive(Debug, PartialEq, Clone)]
pub struct BatteryLine {
    pub line: String,
}

impl BatteryLine {
    fn largest_digit(s: &str, offset: usize, max_offset: usize) -> AocResult<(usize, u32)> {
        let mut max = max_char(&s[offset..max_offset])?;
        max.0 += offset;
        let num = char::to_digit(max.1, 10)
            .ok_or_else(|| AocError::ParseError(format!("largest_digit: {}", max.1)))?;
        Ok((max.0, num))
    }

    pub fn largest_number(&self, digits: u32) -> AocResult<u64> {
        let mut num: u64 = 0;
        let mut offset = 0;
        let mut max_offset = self.line.len() - (digits as usize - 1);

        for i in 0..digits {
            debug!("Finding digit {}", i);

            let digit = Self::largest_digit(&self.line, offset, max_offset)?;

            num = num * 10 + digit.1 as u64;
            offset = digit.0 + 1;
            max_offset += 1;
        }

        Ok(num)
    }
}

impl fmt::Display for BatteryLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

fn read_input_file(path: &str) -> AocResult<Vec<BatteryLine>> {
    std::fs::read_to_string(path)
        .map_err(|e| AocError::IoError(format!("Failed to read input file {}: {}", path, e)))?
        .lines()
        .map(|line| parse_battery_line(line))
        .collect()
}

fn parse_battery_line(line: &str) -> AocResult<BatteryLine> {
    Ok(BatteryLine {
        line: line.to_string(),
    })
}

fn calc_total_jolt(lines: &Vec<BatteryLine>, mode: Mode) -> u64 {
    let mut total_jolt = 0;
    let digits = match mode {
        Mode::Two => 2,
        Mode::Twelve => 12,
    };
    for line in lines {
        let jolt = line
            .largest_number(digits)
            .expect("Failed to compute largest jolt");
        total_jolt += jolt;
        info!(
            "- In {} you can make the largest jolt possible, {}",
            line, jolt
        );
    }
    total_jolt
}

fn main() {
    use clap::Parser;
    let config = Config::parse();
    env_logger::Builder::new()
        .filter_level(config.verbosity.into())
        .init();
    let lines = read_input_file(&config.input).expect("Failed to read input file");
    let total_jolt = calc_total_jolt(&lines, config.mode);
    println!("Total jolt from all battery lines: {}", total_jolt);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> AocResult<Vec<BatteryLine>> {
        read_input_file("data/day03/test_input.txt")
    }

    fn read_test_input2() -> AocResult<Vec<BatteryLine>> {
        read_input_file("data/day03/test_input2.txt")
    }

    #[test]
    fn test_example() {
        let line = BatteryLine {
            line: "123456".to_string(),
        };
        let jolt = line.largest_number(2).expect("largest number");
        assert_eq!(jolt, 56);
    }

    #[test]
    fn test_test_input() {
        let batteries = read_test_input().expect("read test input");
        let total_jolt = calc_total_jolt(&batteries, Mode::Two);
        assert_eq!(total_jolt, 357);
    }

    #[test]
    fn test_test_input2() {
        let batteries = read_test_input2().expect("read test input 2");
        let total_jolt = calc_total_jolt(&batteries, Mode::Two);
        assert_eq!(total_jolt, 77 + 98 + 66 + 66);
    }

    #[test]
    fn test_example_12() {
        let batteries = read_test_input().expect("read test input");
        let total_jolt = calc_total_jolt(&batteries, Mode::Twelve);
        assert_eq!(total_jolt, 3121910778619);
    }

    #[test]
    fn test_example_12_2() {
        let batteries = read_test_input2().expect("read test input 2");
        let total_jolt = calc_total_jolt(&batteries, Mode::Twelve);
        assert_eq!(total_jolt, 3084441169181);
    }
}
