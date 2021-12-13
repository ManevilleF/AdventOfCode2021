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

impl FoldAlong {
    fn coords_to_fold_closure(self) -> Box<dyn Fn(&Coords) -> Option<(Coords, Coords)>> {
        match self {
            Self::X(threshold) => Box::new(move |(x, y): &Coords| {
                x.checked_sub(threshold)
                    .and_then(|delta| threshold.checked_sub(delta))
                    .map(|v| ((*x, *y), (v, *y)))
            }),
            Self::Y(threshold) => Box::new(move |(x, y): &Coords| {
                y.checked_sub(threshold)
                    .and_then(|delta| threshold.checked_sub(delta))
                    .map(|v| ((*x, *y), (*x, v)))
            }),
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
                    .and_then(|(x, y)| x.parse().ok().zip(y.parse().ok()))
                    .ok_or(format!("Expected two valid elements in {}", l))
            })
            .collect::<Result<HashSet<Coords>, Self::Err>>()?;
        Ok(Self(coords))
    }
}

impl Positions {
    pub fn fold(&mut self, instruction: FoldAlong) {
        let func = instruction.coords_to_fold_closure();
        let folded_values: Vec<(Coords, Coords)> = self.0.iter().filter_map(func).collect();
        for (delete, insert) in folded_values {
            self.0.remove(&delete);
            self.0.insert(insert);
        }
    }

    pub fn max_coords(&self) -> Coords {
        (
            self.0.iter().max_by_key(|(x, _)| x).map_or(0, |(x, _)| *x),
            self.0.iter().max_by_key(|(_, y)| y).map_or(0, |(_, y)| *y),
        )
    }
}

impl Display for Positions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x_max, y_max) = self.max_coords();
        let buff = (0..=y_max)
            .map(|y| {
                (0..=x_max)
                    .map(|x| self.0.get(&(x, y)).map_or(' ', |_| '#'))
                    .collect()
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
                    .map(|l| FoldAlong::from_str(l).unwrap())
                    .collect::<Vec<FoldAlong>>(),
            )
        })
        .unwrap();
    for (i, fold_instruction) in fold_instructions.into_iter().enumerate() {
        positions.fold(fold_instruction);
        if i == 0 {
            println!("Part 1: {} dots visible", positions.0.len());
        }
    }
    println!("Part 2: \n{}", positions);
}
