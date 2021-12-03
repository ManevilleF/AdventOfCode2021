const FILE_PATH: &str = "input.txt";

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

    pub fn max_char(&self) -> char {
        self.max_bit().into()
    }

    pub const fn min_bit(&self) -> Bit {
        if self.count_1 < self.count_0 {
            Bit::Bit1
        } else {
            Bit::Bit0
        }
    }

    pub fn min_char(&self) -> char {
        self.min_bit().into()
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

    pub fn get_string(distributions: &[Self], func: impl Fn(&Self) -> char) -> String {
        distributions.iter().map(func).collect()
    }
}

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

fn day_1(bits: &[Vec<Bit>], expected_len: usize) {
    let distributions: Vec<BitDistribution> = (0..expected_len)
        .map(|i| BitDistribution::at(i, bits))
        .collect();
    let gamma_str = BitDistribution::get_string(&distributions, BitDistribution::max_char);
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon_str = BitDistribution::get_string(&distributions, BitDistribution::min_char);
    let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();
    println!(
        "Part1. Gamma str: {} - val = {}, Epsilon str: {} - val = {}. Result = {}",
        gamma_str,
        gamma,
        epsilon_str,
        epsilon,
        gamma * epsilon
    );
}

fn day_2(bits: &[Vec<Bit>], max_len: usize) {
    let (oxygen, co2) = (0..max_len).fold(
        (bits.to_vec(), bits.to_vec()),
        |(mut oxy_candidates, mut co2_candidate), i| {
            if oxy_candidates.len() > 1 {
                let oxy_bit = BitDistribution::at(i, &oxy_candidates).max_bit();
                oxy_candidates = oxy_candidates
                    .into_iter()
                    .filter(|arr| *arr.get(i).unwrap() == oxy_bit)
                    .collect();
            }
            if co2_candidate.len() > 1 {
                let co2_bit = BitDistribution::at(i, &co2_candidate).min_bit();
                co2_candidate = co2_candidate
                    .into_iter()
                    .filter(|arr| *arr.get(i).unwrap() == co2_bit)
                    .collect();
            }
            (oxy_candidates, co2_candidate)
        },
    );
    let oxygen_str: String = oxygen[0].iter().copied().map(char::from).collect();
    let oxygen = u32::from_str_radix(&oxygen_str, 2).unwrap();
    let co2_str: String = co2[0].iter().copied().map(char::from).collect();
    let co2 = u32::from_str_radix(&co2_str, 2).unwrap();
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
