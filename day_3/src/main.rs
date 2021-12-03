const FILE_PATH: &str = "input.txt";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Bit {
    Bit0,
    Bit1,
}

impl Bit {
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::Bit0),
            '1' => Some(Self::Bit1),
            _ => None,
        }
    }
}

impl From<Bit> for char {
    fn from(b: Bit) -> Self {
        match b {
            Bit::Bit0 => '0',
            Bit::Bit1 => '1',
        }
    }
}

#[derive(Debug, Clone, Default)]
struct BitDistribution {
    count_0: usize,
    count_1: usize,
}

impl BitDistribution {
    pub const fn max_bit(&self) -> Bit {
        if self.count_1 >= self.count_0 {
            Bit::Bit1
        } else {
            Bit::Bit0
        }
    }

    pub const fn min_bit(&self) -> Bit {
        if self.count_1 < self.count_0 {
            Bit::Bit1
        } else {
            Bit::Bit0
        }
    }

    pub fn at(index: usize, bits: &[Vec<Bit>]) -> Self {
        bits.iter()
            .filter_map(|arr| arr.get(index))
            .fold(Self::default(), |mut distrib, bit| {
                match bit {
                    Bit::Bit0 => distrib.count_0 += 1,
                    Bit::Bit1 => distrib.count_1 += 1,
                }
                distrib
            })
    }

    pub fn bit_vec(distributions: &[Self], func: impl Fn(&Self) -> Bit) -> Vec<Bit> {
        distributions.iter().map(func).collect()
    }
}

fn bit_vec_values(bits: &[Bit]) -> (String, u32) {
    let str: String = bits.iter().copied().map(char::from).collect();
    let value = u32::from_str_radix(&str, 2).unwrap();
    (str, value)
}

fn day_1(bits: &[Vec<Bit>], expected_len: usize) {
    let distributions: Vec<BitDistribution> = (0..expected_len)
        .map(|i| BitDistribution::at(i, bits))
        .collect();
    let (gamma_str, gamma) = bit_vec_values(&BitDistribution::bit_vec(
        &distributions,
        BitDistribution::max_bit,
    ));
    let (epsilon_str, epsilon) = bit_vec_values(&BitDistribution::bit_vec(
        &distributions,
        BitDistribution::min_bit,
    ));
    println!(
        "Part1. Gamma str: {} - val = {}, Epsilon str: {} - val = {}. Result = {}",
        gamma_str,
        gamma,
        epsilon_str,
        epsilon,
        gamma * epsilon
    );
}

fn day2_candidate(
    previous_candidate: &[Vec<Bit>],
    func: impl Fn(&BitDistribution) -> Bit,
    index: usize,
) -> Option<Vec<Vec<Bit>>> {
    if previous_candidate.len() <= 1 {
        return None;
    }
    let target_bit = func(&BitDistribution::at(index, previous_candidate));
    let new_candidate: Vec<Vec<Bit>> = previous_candidate
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

fn day_2(bits: &[Vec<Bit>], max_len: usize) {
    let (oxygen, co2) = (0..max_len).fold(
        (bits.to_vec(), bits.to_vec()),
        |(mut oxy_candidates, mut co2_candidate), i| {
            if let Some(candidate) = day2_candidate(&oxy_candidates, BitDistribution::max_bit, i) {
                oxy_candidates = candidate;
            }
            if let Some(candidate) = day2_candidate(&co2_candidate, BitDistribution::min_bit, i) {
                co2_candidate = candidate;
            }
            (oxy_candidates, co2_candidate)
        },
    );
    let (oxygen_str, oxygen) = bit_vec_values(oxygen.first().unwrap());
    let (co2_str, co2) = bit_vec_values(co2.first().unwrap());
    println!(
        "Part2. Oxygen str: {} - val = {}, CO2 str: {} - val = {}. Result = {}",
        oxygen_str,
        oxygen,
        co2_str,
        co2,
        oxygen * co2
    );
}

fn main() {
    let bits: Vec<Vec<Bit>> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .map(|str| str.chars().filter_map(Bit::from_char).collect())
        .collect();
    let expected_len = bits.first().expect("File is empty").len();
    day_1(&bits, expected_len);
    day_2(&bits, expected_len);
}
