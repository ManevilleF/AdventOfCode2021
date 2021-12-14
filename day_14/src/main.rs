use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";
type Pair = [char; 2];

#[derive(Debug)]
struct Polymer(HashMap<Pair, usize>);

#[derive(Debug)]
struct PairInsertion(HashMap<Pair, [Pair; 2]>);

impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let map = chars.windows(2).fold(HashMap::new(), |mut acc, win| {
            let key: [char; 2] = (*win).try_into().unwrap();
            *acc.entry(key).or_insert(0_usize) += 1;
            acc
        });
        Ok(Self(map))
    }
}

impl FromStr for PairInsertion {
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
                    .map(|(k, c): (Pair, char)| (k, [[k[0], c], [c, k[1]]]))
                    .ok_or_else(|| format!("Wrong line {}", l))
            })
            .collect::<Result<HashMap<Pair, [Pair; 2]>, Self::Err>>()?;
        Ok(Self(map))
    }
}

impl PairInsertion {
    pub fn apply_to_polymer(&self, polymer: Polymer) -> Polymer {
        let new_polymer = polymer
            .0
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                if let Some(pairs) = self.0.get(&k).copied() {
                    pairs
                        .into_iter()
                        .for_each(|pair| *acc.entry(pair).or_insert(0_usize) += v);
                }
                acc
            });
        Polymer(new_polymer)
    }
}

impl Polymer {
    fn repartition(&self) -> HashMap<char, usize> {
        self.0.iter().fold(HashMap::new(), |mut acc, (k, v)| {
            *acc.entry(k[0]).or_insert(0_usize) += *v;
            acc
        })
    }

    fn substracted_repartition(&self) -> usize {
        match self.repartition().values().minmax() {
            MinMaxResult::NoElements | MinMaxResult::OneElement(_) => 0,
            MinMaxResult::MinMax(min, max) => max.saturating_sub(min + 1),
        }
    }
}

fn main() {
    let (mut polymer, pairs): (Polymer, PairInsertion) = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split_once("\n\n")
        .map(|(poly, pairs)| {
            (
                Polymer::from_str(poly).unwrap(),
                PairInsertion::from_str(pairs).unwrap(),
            )
        })
        .unwrap();
    for i in 1..=40 {
        polymer = pairs.apply_to_polymer(polymer);
        if i == 10 {
            println!("Part 1: {}", polymer.substracted_repartition());
        }
    }
    println!("Part 2: {}", polymer.substracted_repartition());
}
