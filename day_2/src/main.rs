use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug, Copy, Clone)]
pub enum MoveDirection {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for MoveDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        if split.len() != 2 {
            return Err(format!("Wrong format: `{}` cannot be parsed", s));
        }
        let value = split[1].parse::<u32>().map_err(|e| e.to_string())?;
        match split[0] {
            "down" => Ok(Self::Down(value)),
            "forward" => Ok(Self::Forward(value)),
            "up" => Ok(Self::Up(value)),
            _ => Err(format!("Wrong direction: {}", s)),
        }
    }
}

fn part_1(directions: impl Iterator<Item = MoveDirection>) {
    let (x, y) = directions.fold((0, 0), |(x, y), dir| match dir {
        MoveDirection::Forward(v) => (x + v, y),
        MoveDirection::Down(v) => (x, y + v),
        MoveDirection::Up(v) => (x, y.saturating_sub(v)),
    });
    println!("Part 1. Final pos = ({}, {}), Result = {}", x, y, x * y);
}

fn part_2(directions: impl Iterator<Item = MoveDirection>) {
    let (x, y, _aim) = directions.fold((0, 0, 0), |(x, y, aim), dir| match dir {
        MoveDirection::Forward(v) => (x + v, y + aim * v, aim),
        MoveDirection::Down(v) => (x, y, aim + v),
        MoveDirection::Up(v) => (x, y, aim.saturating_sub(v)),
    });
    println!("Part 2. Final pos = ({}, {}), Result = {}", x, y, x * y);
}

fn main() {
    let file_content: Vec<MoveDirection> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .filter_map(|str| MoveDirection::from_str(str).ok())
        .collect();
    part_1(file_content.iter().copied());
    part_2(file_content.into_iter());
}
