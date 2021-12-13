use std::collections::HashSet;
use std::str::FromStr;

const FILE_PATH: &str = "test.txt";

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

fn main() {
    let (positions, fold_instructions) = std::fs::read_to_string(FILE_PATH)
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
    println!("{:#?}", positions);
    println!("{:#?}", fold_instructions);
}
