use std::collections::HashMap;
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

const REGULAR_PATTERNS: &[&str; 10] = &[
    "abcefg",  // 0
    "cf",      // 1
    "acdeg",   // 2
    "acdfg",   // 3
    "bcdf",    // 4
    "abdfg",   // 5
    "abdefg",  // 6
    "acf",     // 7
    "abcdefg", // 8
    "abcdfg",  // 9
];

#[derive(Debug, Clone)]
struct Entry {
    patterns: [String; 10],
    output_values: [String; 4],
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: [String; 2] = s
            .trim()
            .split('|')
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .try_into()
            .map_err(|e| format!("{} Is not valid: {:?} needs 2 elements", s, e))?;
        let patterns = split[0]
            .trim()
            .split(' ')
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .try_into()
            .map_err(|e| format!("{} Is not valid: {:?} needs 10 elements", s, e))?;
        let output_values = split[1]
            .trim()
            .split(' ')
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .try_into()
            .map_err(|e| format!("{} Is not valid: {:?} needs 4 elements", s, e))?;
        Ok(Self {
            patterns,
            output_values,
        })
    }
}

impl Entry {
    fn pattern_matching(&self) -> HashMap<char, Vec<char>> {
        let mut patterns = self.patterns.clone();
        patterns.sort_by_key(|p| p.len());
        let mut done_values: Vec<char> = vec![];
        patterns
            .iter()
            .filter(|p| [2, 3, 4, 7].contains(&p.len()))
            .fold(HashMap::<char, Vec<char>>::new(), |mut map, pattern| {
                for matched in REGULAR_PATTERNS.iter().filter(|p| p.len() == pattern.len()) {
                    let from = pattern.chars();
                    let to = matched.chars().collect::<Vec<char>>();
                    for from_char in from.into_iter() {
                        let entry = map.entry(from_char).or_insert_with(|| {
                            to.iter()
                                .copied()
                                .filter(|c| !done_values.contains(c))
                                .collect()
                        });
                        *entry = entry.iter().filter(|c| to.contains(c)).copied().collect();
                    }
                }
                for (_k, v) in map.iter() {
                    done_values.extend(v.clone());
                    done_values.sort_unstable();
                    done_values.dedup();
                }
                map
            })
    }

    fn possible_matches(output: &str, mapper: &HashMap<char, Vec<char>>) -> Vec<String> {
        output.chars().fold(vec![], |acc, c| {
            mapper
                .get(&c)
                .map(|options| {
                    options
                        .iter()
                        .map(|option| {
                            if acc.is_empty() {
                                vec![option.to_string()]
                            } else {
                                acc.iter()
                                    .map(|s| format!("{}{}", s, option))
                                    .collect::<Vec<String>>()
                            }
                        })
                        .flatten()
                        .collect()
                })
                .unwrap_or(acc)
        })
    }

    fn match_output(output: &str, mapper: &HashMap<char, Vec<char>>) -> usize {
        let possible_matches = Self::possible_matches(output, mapper);
        let matches = possible_matches.iter().fold(vec![], |mut res, candidate| {
            let mut chars: Vec<char> = candidate.chars().collect();
            chars.sort_unstable();
            let str = String::from_iter(chars);
            if let Some(pos) = REGULAR_PATTERNS.iter().position(|s| s == &str.as_str()) {
                res.push(pos);
                res.dedup();
            }
            res
        });
        matches
            .first()
            .copied()
            .unwrap_or_else(|| panic!("No matches for {}", output))
    }

    fn outputs_sum_str(&self) -> String {
        let mapper = self.pattern_matching();
        let res: Vec<String> = self
            .output_values
            .iter()
            .map(|output_value| Self::match_output(output_value, &mapper).to_string())
            .collect();
        res.join("")
    }
}

fn part1(entries: &[Entry]) -> usize {
    entries.iter().fold(0, |count, entry| {
        entry
            .output_values
            .iter()
            .fold(0, |c, digit| match digit.len() {
                1 | 2 | 3 | 4 | 7 => c + 1,
                _ => c,
            })
            + count
    })
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
        .split('\n')
        .map(|s| Entry::from_str(s).unwrap())
        .collect();
    println!("Part1: {} outputs use 1, 4, 7, or 8 digit", part1(&entries));
    println!("Part2: Output sum is {}", part2(&entries));
}
