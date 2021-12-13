#![feature(hash_drain_filter)]
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

type Coords = (u32, u32);

#[derive(Debug, Copy, Clone)]
enum FoldAlong {
    X(u32),
    Y(u32),
}

#[derive(Debug, Clone)]
struct Positions(HashSet<Coords>);

impl FromStr for FoldAlong {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (text, value) = s
            .split_once('=')
            .ok_or_else(|| format!("Expected 2 elements in {}", s))?;
        let value = value
            .parse()
            .map_err(|e| format!("Wrong fold value {}", e))?;
        match text.to_ascii_lowercase().as_str() {
            "fold along y" => Ok(Self::Y(value)),
            "fold along x" => Ok(Self::X(value)),
            _ => return Err(format!("Wrong fold instruction `{}`", text)),
        }
    }
}

impl FromStr for Positions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .lines()
            .map(|l| {
                l.split_once(',')
                    .and_then(|(x, y)| x.parse::<u32>().ok().zip(y.parse::<u32>().ok()))
                    .ok_or(format!("Expected two valid elements in {}", l))
            })
            .collect::<Result<HashSet<Coords>, Self::Err>>()?;
        Ok(Self(coords))
    }
}

impl Positions {
    pub fn fold(&mut self, instruction: FoldAlong) {
        let folded_values: HashSet<Coords> = self
            .0
            // WARNING: unstable, use nightly toolchain
            .drain_filter(|(x, y)| match &instruction {
                FoldAlong::X(x_treshold) => x_treshold < x,
                FoldAlong::Y(y_treshold) => y_treshold < y,
            })
            .filter_map(|(x, y)| match &instruction {
                FoldAlong::X(x_treshold) => x
                    .checked_sub(*x_treshold)
                    .and_then(|delta| x_treshold.checked_sub(delta))
                    .map(|x| (x, y)),
                FoldAlong::Y(y_treshold) => y
                    .checked_sub(*y_treshold)
                    .and_then(|delta| y_treshold.checked_sub(delta))
                    .map(|y| (x, y)),
            })
            .collect();
        for v in folded_values {
            self.0.insert(v);
        }
    }

    pub fn max_coords(&self) -> Coords {
        self.0
            .iter()
            .fold((0, 0), |(mut x_acc, mut y_acc), (x, y)| {
                if *x > x_acc {
                    x_acc = *x;
                }
                if *y > y_acc {
                    y_acc = *y;
                }
                (x_acc, y_acc)
            })
    }
}

impl Display for Positions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x_max, y_max) = self.max_coords();
        let buff = (0..=y_max)
            .map(|y| {
                (0..=x_max)
                    .map(|x| self.0.get(&(x, y)).map(|_| '#').unwrap_or('.'))
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        write!(f, "{}", buff.join("\n"))
    }
}

fn main() {
    let (mut positions, fold_instructions) = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split_once("\n\n")
        .map(|(p, f)| {
            (
                Positions::from_str(p).unwrap(),
                f.lines()
                    .map(|i| FoldAlong::from_str(i).unwrap())
                    .collect::<Vec<FoldAlong>>(),
            )
        })
        .unwrap();
    for (i, fold_instruction) in fold_instructions.into_iter().enumerate() {
        positions.fold(fold_instruction);
        if i == 0 {
            println!("Part 1: {} Dots visible", positions.0.len());
        }
    }
    println!("Part2: \n{}", positions);
}
