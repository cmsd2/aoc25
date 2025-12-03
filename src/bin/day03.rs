use core::fmt;
use std::cmp::Ordering;

use aoc25::error::AocError;
use aoc25::result::AocResult;
use log::{debug, info};

#[derive(clap::Parser, Debug, Clone)]
pub struct Config {
    #[clap(
        short,
        long,
        default_value = "data/day03/input.txt",
        help = "Path to input file"
    )]
    pub input: String,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn max_char(s: &str) -> AocResult<(usize, char)> {
    s.chars().enumerate().max_by(|(_, a), (_, b)| {
            if a >= b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }).ok_or(AocError::ParseError(format!("max_char: {}", s)))
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

    pub fn largest_number(&self, digits: u32) -> AocResult<(u32, (usize, usize))> {
        assert_eq!(digits, 2);
        
        let max_1 = Self::largest_digit(&self.line, 0, self.line.len() - 1)?;
        debug!("digit 1: {} at {}", max_1.1, max_1.0);
        let max_2 = Self::largest_digit(&self.line, max_1.0 + 1, self.line.len())?;
        debug!("digit 2: {} at {}", max_2.1, max_2.0);

        Ok((max_1.1 * 10 + max_2.1, (max_1.0, max_2.0)))
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
    Ok(BatteryLine { line: line.to_string() })
}

fn calc_total_jolt(lines: &Vec<BatteryLine>) -> u32 {
    let mut total_jolt = 0;
    for line in lines {
        let (jolt, (idx1, idx2)) = line.largest_number(2).expect("Failed to compute largest jolt");
        total_jolt += jolt;
        info!("- In {} you can make the largest jolt possible, {} by turning on batteries {} and {}", line, jolt, idx1, idx2);
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
    let total_jolt = calc_total_jolt(&lines);
    println!("Total jolt from all battery lines: {}", total_jolt);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let line = BatteryLine { line: "123456".to_string() };
        let (jolt, (_idx1, _idx2)) = line.largest_number(2).expect("largest number");
        assert_eq!(jolt, 56);
    }

    #[test]
    fn test_test_input() {
        let batteries = read_input_file("data/day03/test_input.txt").expect("read test input");
        let total_jolt = calc_total_jolt(&batteries);
        assert_eq!(total_jolt, 357);
    }

    #[test]
    fn test_test_input2() {
        let batteries = read_input_file("data/day03/test_input2.txt").expect("read test input 2");
        let total_jolt = calc_total_jolt(&batteries);
        assert_eq!(total_jolt, 77 + 98 + 66 + 66);
    }
}