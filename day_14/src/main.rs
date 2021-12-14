use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";
type Pair = [char; 2];

#[derive(Debug)]
struct Polymer {
    pub pairs: HashMap<Pair, usize>,
    pub counts: HashMap<char, usize>,
}

#[derive(Debug)]
struct PairInsertions(HashMap<Pair, char>);

impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let counts = chars.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        });
        let pairs = chars.windows(2).try_fold(HashMap::new(), |mut acc, win| {
            let key: [char; 2] = (*win)
                .try_into()
                .map_err(|e| format!("Invalid char pair {:?}, {}", win, e))?;
            *acc.entry(key).or_insert(0) += 1;
            Result::<_, Self::Err>::Ok(acc)
        })?;
        Ok(Self { pairs, counts })
    }
}

impl FromStr for PairInsertions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| {
                l.split_once(" -> ")
                    .map(|(left, right)| {
                        (
                            left.chars().collect::<Vec<char>>(),
                            right.chars().collect::<Vec<char>>(),
                        )
                    })
                    .and_then(|(l, r)| l.try_into().ok().zip(r.first().copied()))
                    .ok_or_else(|| format!("Wrong line {}", l))
            })
            .collect::<Result<HashMap<Pair, char>, Self::Err>>()?;
        Ok(Self(map))
    }
}

impl PairInsertions {
    pub fn apply_to_polymer(&self, polymer: Polymer) -> Polymer {
        polymer.pairs.into_iter().fold(
            Polymer {
                pairs: HashMap::default(),
                counts: polymer.counts,
            },
            |mut poly, (k, v)| {
                if let Some(insertion) = self.0.get(&k) {
                    *poly.counts.entry(*insertion).or_insert(0) += v;
                    for pair in [[k[0], *insertion], [*insertion, k[1]]] {
                        *poly.pairs.entry(pair).or_insert(0) += v;
                    }
                }
                poly
            },
        )
    }
}

impl Polymer {
    fn subtracted_repartition(&self) -> usize {
        match self.counts.values().minmax() {
            MinMaxResult::NoElements | MinMaxResult::OneElement(_) => 0,
            MinMaxResult::MinMax(min, max) => max.saturating_sub(*min),
        }
    }
}

fn main() {
    let (mut polymer, pairs): (Polymer, PairInsertions) = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split_once("\n\n")
        .map(|(poly, pairs)| {
            (
                Polymer::from_str(poly).unwrap(),
                PairInsertions::from_str(pairs).unwrap(),
            )
        })
        .unwrap();
    for i in 1..=40 {
        polymer = pairs.apply_to_polymer(polymer);
        if i == 10 {
            println!("Part 1: {}", polymer.subtracted_repartition());
        }
    }
    println!("Part 2: {}", polymer.subtracted_repartition());
}
