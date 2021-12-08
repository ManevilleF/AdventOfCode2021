use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug, Clone, Default)]
struct DigitPattern {
    pattern_for_1: [char; 2],
    pattern_for_4: [char; 4],
    pattern_for_7: [char; 3],
}

impl DigitPattern {
    fn char_arr<const SIZE: usize>(patterns: &[String]) -> Result<[char; SIZE], String> {
        let chars: Vec<char> = patterns
            .iter()
            .find(|p| p.len() == SIZE)
            .ok_or_else(|| format!("Could not find pattern with {} characters", SIZE))?
            .chars()
            .collect();
        chars
            .try_into()
            .map_err(|e| format!("Failed to convert {:?} to an array", e))
    }

    fn contained_in_str(pattern: &[char], str: &str) -> bool {
        pattern.iter().fold(true, |res, c| str.contains(*c) && res)
    }

    fn identify_pattern_of_5(&self, pattern: &str) -> usize {
        if Self::contained_in_str(&self.pattern_for_1, pattern) {
            3
        } else if self
            .pattern_for_4
            .iter()
            .copied()
            .filter(|c| pattern.contains(*c))
            .count()
            == 2
        {
            2
        } else {
            5
        }
    }

    fn identify_pattern_of_6(&self, pattern: &str) -> usize {
        if !Self::contained_in_str(&self.pattern_for_7, pattern) {
            6
        } else if Self::contained_in_str(&self.pattern_for_4, pattern) {
            9
        } else {
            0
        }
    }
}

impl FromStr for DigitPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let patterns: Vec<String> = s
            .split_ascii_whitespace()
            .map(ToString::to_string)
            .collect();
        Ok(Self {
            pattern_for_1: Self::char_arr::<2>(&patterns)?,
            pattern_for_4: Self::char_arr::<4>(&patterns)?,
            pattern_for_7: Self::char_arr::<3>(&patterns)?,
        })
    }
}

#[derive(Debug, Clone)]
struct Entry {
    patterns: DigitPattern,
    output_values: Vec<String>,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, output) = s
            .split_once('|')
            .ok_or_else(|| format!("{} needs 2 elements", s))?;
        let patterns = DigitPattern::from_str(patterns)?;
        let output_values: Vec<String> = output
            .split_ascii_whitespace()
            .map(ToString::to_string)
            .collect();
        Ok(Self {
            patterns,
            output_values,
        })
    }
}

impl Entry {
    fn outputs_sum_str(&self) -> String {
        self.output_values
            .iter()
            .map(|output_value| {
                match output_value.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 => self.patterns.identify_pattern_of_5(output_value),
                    6 => self.patterns.identify_pattern_of_6(output_value),
                    7 => 8,
                    _ => panic!("{} is not a valid output (wrong len)", output_value),
                }
                .to_string()
            })
            .collect()
    }
}

fn part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            entry
                .output_values
                .iter()
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
                .count()
        })
        .sum()
}

fn part2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| entry.outputs_sum_str().parse::<usize>().unwrap())
        .sum()
}

fn main() {
    let entries: Vec<Entry> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|s| Entry::from_str(s).unwrap())
        .collect();
    println!("Part1: {} outputs use 1, 4, 7, or 8 digit", part1(&entries));
    println!("Part2: Output sum is {}", part2(&entries));
}
