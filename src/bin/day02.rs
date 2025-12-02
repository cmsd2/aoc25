use std::fmt;

use nom::sequence::terminated;
use nom::{
    IResult, Parser, character::complete::digit1, combinator::map_res, multi::separated_list1,
};
use aoc25::error::AocError;
use aoc25::result::AocResult;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IdRange {
    start: u64,
    end: u64,
}

impl fmt::Display for IdRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[derive(clap::Parser, Debug, Clone)]
struct Config {
    #[clap(
        short,
        long,
        default_value = "data/day02/input.txt",
        help = "Path to input file"
    )]
    pub input: String,
}

fn parse_id_range(s: &str) -> IResult<&str, IdRange> {
    let (s, start) = map_res(digit1, str::parse).parse(s)?;
    let (s, _) = nom::character::complete::char('-')(s)?;
    let (s, end) = map_res(digit1, str::parse).parse(s)?;
    Ok((s, IdRange { start, end }))
}

fn parse_id_range_sequence(input: &str) -> IResult<&str, Vec<IdRange>> {
    let separator = terminated(nom::character::complete::char(','), nom::character::complete::multispace0);
    separated_list1(separator, parse_id_range).parse(input)
}

fn read_input_file(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

fn parse_input_file(path: &str) -> AocResult<Vec<IdRange>> {
    let content = read_input_file(path).expect("Failed to read input file");
    let (_remainder, ranges) = parse_id_range_sequence(&content).map_err(|e| {
        AocError::ParseError(format!("Failed to parse input file {}: {}", path, e))
    })?;
    Ok(ranges)
}

pub fn id_is_valid(id: u64) -> bool {
    let digits = id.ilog10() + 1;
    if digits % 2 != 0 {
        return true;
    }
    let half = digits / 2;
    let pivot = 10u64.pow(half);
    let left = id / pivot;
    let right = id % pivot;
    if left == right {
        return false;
    }

    return true;
}

pub fn invalid_ids_in_range(range: &IdRange) -> impl Iterator<Item = u64> {
    (range.start..=range.end)
        .filter(|&id| !id_is_valid(id))
}

pub fn count_sum_invalid_ids_in_range(range: &IdRange) -> (u64, u64) {
    let acc = (0u64, 0u64);
    invalid_ids_in_range(range).fold(acc, |(count, sum), id| (count + 1, sum + id))
}

fn main() {
    use clap::Parser;
    println!("Hello, day02!");
    let config = Config::parse();
    println!("Input file: {}", config.input);
    let ranges = parse_input_file(&config.input).expect("Failed to parse input file");
    println!("Parsed {} ID ranges from input file.", ranges.len());
    let (mut total_count, mut total_sum) = (0u64, 0u64);
    for range in ranges {
        let (count, sum) = count_sum_invalid_ids_in_range(&range);
        println!("- {} has {} invalid IDs", range, count);
        total_count += count;
        total_sum += sum;
    }
    println!("Total invalid IDs: {}", total_count);
    println!("Sum of invalid IDs: {}", total_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test_input_file() -> Vec<IdRange> {
        parse_input_file("data/day02/test_input.txt").expect("Failed to parse test input file")
    }

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_parse_id_range() {
        let input = "123-456";
        let (_remainder, range) = parse_id_range(input).expect("parser");
        assert_eq!(range.start, 123);
        assert_eq!(range.end, 456);
    }

    #[test]
    fn test_parse_id_range_sequence() {
        let input = "11-22,95-115,998-1012";
        let (_remainder, ranges) = parse_id_range_sequence(input).expect("parser");
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], IdRange { start: 11, end: 22 });
        assert_eq!(ranges[1], IdRange { start: 95, end: 115 });
        assert_eq!(ranges[2], IdRange { start: 998, end: 1012 });
    }

    #[test]
    fn test_parse_test_input() {
        let ranges = parse_test_input_file();
        assert_eq!(ranges.len(), 8);
    }

    #[test]
    fn test_id_is_valid() {
        let fixtures = vec![
            (55, false),
            (6464, false),
            (123123, false),
            (101, true),
        ];
        for (id, expected) in fixtures {
            let result = id_is_valid(id);
            assert_eq!(result, expected, "id_is_valid({}) returned {}, expected {}", id, result, expected);
        }
    }

    #[test]
    fn test_count_sum_invalid_ids_in_range() {
        let range = IdRange { start: 11, end: 22 };
        let (count, sum) = count_sum_invalid_ids_in_range(&range);
        assert_eq!(count, 2);
        assert_eq!(sum, 11 + 22);

        let range = IdRange { start: 95, end: 115 };
        let (count, sum) = count_sum_invalid_ids_in_range(&range);
        assert_eq!(count, 1);
        assert_eq!(sum, 99);
    }

    #[test]
    fn test_count_sum_invalid_ids_in_test_input() {
        let ranges = parse_test_input_file();
        let expected = (8, 1227775554);
        let (mut total_count, mut total_sum) = (0u64, 0u64);
        for range in ranges {
            let (count, sum) = count_sum_invalid_ids_in_range(&range);
            println!("- {} has {} invalid IDs", range, count);
            total_count += count;
            total_sum += sum;
        }
        assert_eq!((total_count, total_sum), expected);
    }
}
