use std::fmt;

use log::{debug, info};
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Two,
    Multiple,
}

impl From<&str> for Mode {
    fn from(s: &str) -> Self {
        match s {
            "two" => Mode::Two,
            "multiple" => Mode::Multiple,
            _ => Mode::Two,
        }
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

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    #[clap(short, long, default_value = "two", help = "Mode: 'two' or 'multiple'")]
    pub mode: Mode,

    #[clap(short, long, help = "Run benchmark")]
    pub bench: bool,

    #[clap(long, help = "Benchmark iterations", default_value = "1000")]
    pub iterations: usize,
}

pub struct BenchmarkResult {
    start_time: std::time::Instant,
    end_time: std::time::Instant,
    iterations: u32,
}

impl BenchmarkResult {
    pub fn run<F>(iterations: u32, f: F) -> Self where F: Fn() {
        let start_time = std::time::Instant::now();
        for _ in 0..iterations {
            f();
        }
        let end_time = std::time::Instant::now();
        BenchmarkResult {
            start_time,
            end_time,
            iterations
        }
    }

    pub fn duration(&self) -> std::time::Duration {
        self.end_time.duration_since(self.start_time)
    }
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration = self.duration();
        write!(f, "Duration: {:?}", duration)?;
        write!(f, "Average:  {:?}", duration / self.iterations)?;
        Ok(())
    }
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

pub fn id_is_valid(id: u64, mode: Mode) -> bool {
    let digits = id.ilog10() + 1;
    let max_freq = match mode {
        Mode::Two => 2,
        Mode::Multiple => digits,
    };
    let mut valid = true;
    debug!("Validating id {} with {} digits in mode {:?}", id, digits, mode);
    for freq in 2..=max_freq {
        debug!("Checking id {} for freq {}", id, freq);
        if digits % freq != 0 {
            debug!("Skipping id {} for freq {}: not divisible", id, freq);
            continue;
        }

        let mut valid_at_freq = false;
        let period = digits / freq;
        let pivot = 10u64.pow(period);
        let right = id % pivot;
        let mut id_pivoted = id;
        debug!("  period {}, pivot {}, right {}", period, pivot, right);
        for i in 1..freq {
            debug!("    iteration {}, id {}", i, id_pivoted);
            id_pivoted /= pivot;
            if id_pivoted % pivot != right {
                debug!("      id {} valid at iteration {}", id_pivoted, i);
                valid_at_freq = true;
                break;
            }
        }

        valid = valid && valid_at_freq;

        if !valid {
            break;
        }
    }
    
    return valid;
}

pub fn invalid_ids_in_range(range: &IdRange, mode: Mode) -> impl Iterator<Item = u64> {
    (range.start..=range.end)
        .filter(move |&id| !id_is_valid(id, mode))
}

pub fn count_sum_invalid_ids_in_range(range: &IdRange, mode: Mode) -> (u64, u64) {
    let acc = (0u64, 0u64);
    invalid_ids_in_range(range, mode).fold(acc, |(count, sum), id| (count + 1, sum + id))
}

pub fn calc_count_sum(ranges: &[IdRange], mode: Mode) -> (u64, u64) {
    let (mut total_count, mut total_sum) = (0u64, 0u64);
    for range in ranges {
        let (count, sum) = count_sum_invalid_ids_in_range(&range, mode);
        info!("- {} has {} invalid IDs", range, count);
        total_count += count;
        total_sum += sum;
    }    
    (total_count, total_sum)
}

fn main() {
    use clap::Parser;
    let config = Config::parse();
    
    env_logger::Builder::new()
        .filter_level(config.verbosity.into())
        .init();
    
    let ranges = parse_input_file(&config.input).expect("Failed to parse input file");
    info!("Parsed {} ID ranges from input file {}", ranges.len(), config.input);

    if config.bench {
        let bench_result = BenchmarkResult::run(config.iterations as u32, || {
            let _ = calc_count_sum(&ranges[..], config.mode);
        });
        println!("Benchmark result over {} iterations:\n{}", config.iterations, bench_result);
    } else {
        let (total_count, total_sum) = calc_count_sum(&ranges[..], config.mode);   
        println!("Total invalid IDs: {}", total_count);
        println!("Sum of invalid IDs: {}", total_sum);
    }
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
        assert_eq!(ranges.len(), 11);
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
            let result = id_is_valid(id, Mode::Two);
            assert_eq!(result, expected, "id_is_valid({}) returned {}, expected {}", id, result, expected);
        }
    }

    #[test]
    fn test_id_is_valid_multiple_mode() {
        let fixtures = vec![
            (55, false),
            (6464, false),
            (123123, false),
            (123123123, false),
            (1212121212, false),
            (1111111, false),
            (101, true),
        ];
        for (id, expected) in fixtures {
            let result = id_is_valid(id, Mode::Multiple);
            assert_eq!(result, expected, "id_is_valid({}) returned {}, expected {}", id, result, expected);
        }
    }

    #[test]
    fn test_count_sum_invalid_ids_in_range() {
        let range = IdRange { start: 11, end: 22 };
        let (count, sum) = count_sum_invalid_ids_in_range(&range, Mode::Two);
        assert_eq!(count, 2);
        assert_eq!(sum, 11 + 22);

        let range = IdRange { start: 95, end: 115 };
        let (count, sum) = count_sum_invalid_ids_in_range(&range, Mode::Two);
        assert_eq!(count, 1);
        assert_eq!(sum, 99);
    }

    #[test]
    fn test_count_sum_invalid_ids_in_test_input() {
        let ranges = parse_test_input_file();
        let expected = (8, 1227775554);
        let (total_count, total_sum) = calc_count_sum(&ranges[..], Mode::Two);
        assert_eq!((total_count, total_sum), expected);
    }

    #[test]
    fn test_coun_sum_invalid_ids_multiple_mode_in_test_input() {
        let ranges = parse_test_input_file();
        let expected = (13, 4174379265);
        let (total_count, total_sum) = calc_count_sum(&ranges[..], Mode::Multiple);
        assert_eq!((total_count, total_sum), expected);
    }
}
