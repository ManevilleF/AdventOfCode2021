const FILE_PATH: &str = "input.txt";

#[derive(Debug, Clone, Default)]
struct BitDistribution {
    count_0: usize,
    count_1: usize,
}

impl BitDistribution {
    pub const fn max_char(&self) -> char {
        if self.count_1 > self.count_0 {
            '1'
        } else {
            '0'
        }
    }

    pub const fn min_char(&self) -> char {
        if self.count_1 < self.count_0 {
            '1'
        } else {
            '0'
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

fn day_1(bits: &[Vec<Bit>]) {
    let expected_len = bits.first().unwrap().len();
    let distributions: Vec<BitDistribution> = (0..expected_len)
        .map(|i| BitDistribution::at(i, &bits))
        .collect();
    let gamma_str = BitDistribution::get_string(&distributions, BitDistribution::max_char);
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon_str = BitDistribution::get_string(&distributions, BitDistribution::min_char);
    let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();
    println!(
        "Gamma str: {} - val {}, Epsilon str: {} - val {}. Result: {}",
        gamma_str,
        gamma,
        epsilon_str,
        epsilon,
        gamma * epsilon
    );
}

fn day_2(bits: &[Vec<Bit>]) {}

fn main() {
    let bits: Vec<Vec<Bit>> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .map(|str| str.chars().filter_map(Bit::from_char).collect())
        .collect();
    day_1(&bits);
    day_2(&bits);
}
