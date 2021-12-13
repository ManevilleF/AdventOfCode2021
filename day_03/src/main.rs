const FILE_PATH: &str = "input.txt";

struct BitDistribution {
    count_1: usize,
    count_0: usize,
}

impl BitDistribution {
    pub const fn max_bit(&self) -> char {
        if self.count_1 >= self.count_0 {
            '1'
        } else {
            '0'
        }
    }

    pub const fn min_bit(&self) -> char {
        if self.count_1 < self.count_0 {
            '1'
        } else {
            '0'
        }
    }

    pub fn at(index: usize, bits: &[Vec<char>]) -> Self {
        let len = bits.len();
        let count_1 = bits
            .iter()
            .filter_map(|arr| arr.get(index).copied())
            .filter(|c| *c == '1')
            .count();
        Self {
            count_1,
            count_0: len - count_1,
        }
    }

    pub fn bit_str(distributions: &[Self], func: impl Fn(&Self) -> char) -> String {
        distributions.iter().map(func).collect()
    }

    pub fn bit_vec_value(distributions: &[Self], func: impl Fn(&Self) -> char) -> u32 {
        let bit_str = Self::bit_str(distributions, func);
        bit_str_value(&bit_str)
    }
}

fn bit_str_value(bit_str: &str) -> u32 {
    u32::from_str_radix(bit_str, 2).unwrap()
}

fn day_1(bits: &[Vec<char>], expected_len: usize) {
    let distributions: Vec<BitDistribution> = (0..expected_len)
        .map(|i| BitDistribution::at(i, bits))
        .collect();
    let gamma = BitDistribution::bit_vec_value(&distributions, BitDistribution::max_bit);
    let epsilon = BitDistribution::bit_vec_value(&distributions, BitDistribution::min_bit);
    println!(
        "Part1. Gamma = {}, Epsilon = {}. Result = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn day2_candidate(
    previous_candidate: &[Vec<char>],
    func: impl Fn(&BitDistribution) -> char,
    index: usize,
) -> Option<Vec<Vec<char>>> {
    if previous_candidate.len() <= 1 {
        return None;
    }
    let target_bit = func(&BitDistribution::at(index, previous_candidate));
    let new_candidate: Vec<Vec<char>> = previous_candidate
        .iter()
        .cloned()
        .filter(|arr| arr.get(index).map_or(false, |bit| *bit == target_bit))
        .collect();
    if new_candidate.is_empty() {
        None
    } else {
        Some(new_candidate)
    }
}

fn day_2(bits: &[Vec<char>], max_len: usize) {
    let (oxygen, co2) = (0..max_len).fold(
        (bits.to_vec(), bits.to_vec()),
        |(mut oxy_candidates, mut co2_candidates), i| {
            if let Some(candidate) = day2_candidate(&oxy_candidates, BitDistribution::max_bit, i) {
                oxy_candidates = candidate;
            }
            if let Some(candidate) = day2_candidate(&co2_candidates, BitDistribution::min_bit, i) {
                co2_candidates = candidate;
            }
            (oxy_candidates, co2_candidates)
        },
    );
    let oxygen = bit_str_value(oxygen.first().unwrap().iter().collect::<String>().as_str());
    let co2 = bit_str_value(co2.first().unwrap().iter().collect::<String>().as_str());
    println!(
        "Part2. Oxygen = {}, CO2 = {}. Result = {}",
        oxygen,
        co2,
        oxygen * co2
    );
}

fn main() {
    let bits: Vec<Vec<char>> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .map(|str| str.chars().collect())
        .collect();
    let expected_len = bits.first().expect("File is empty").len();
    day_1(&bits, expected_len);
    day_2(&bits, expected_len);
}
