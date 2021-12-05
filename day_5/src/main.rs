use std::cmp::{max, min};
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug, Copy, Clone, Default)]
struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl FromStr for UVec2 {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let arr: Vec<u32> = str.split(',').filter_map(|s| s.parse().ok()).collect();
        let arr: [u32; 2] = arr
            .try_into()
            .map_err(|_| format!("{} doesn't have 2 valid elements", str))?;
        Ok(Self {
            x: arr[0],
            y: arr[1],
        })
    }
}

impl From<(u32, u32)> for UVec2 {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Line {
    pub start: UVec2,
    pub end: UVec2,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let arr: Vec<UVec2> = str.split(" -> ").filter_map(|s| s.parse().ok()).collect();
        let arr: [UVec2; 2] = arr
            .try_into()
            .map_err(|_| format!("{} doesn't have 2 valid elements", str))?;
        Ok(Self {
            start: arr[0],
            end: arr[1],
        })
    }
}

impl Line {
    /// In vertical or horizontal line
    fn in_straight_line(&self, point: UVec2) -> bool {
        let res = if self.start.x == self.end.x {
            point.x == self.start.x
                && (min(self.start.y, self.end.y)..=max(self.start.y, self.end.y))
                    .contains(&point.y)
        } else if self.start.y == self.end.y {
            point.y == self.start.y
                && (min(self.start.x, self.end.x)..=max(self.start.x, self.end.x))
                    .contains(&point.x)
        } else {
            false
        };
        res
    }
}

fn main() {
    let lines: Vec<Line> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();
    let max_point = lines.iter().fold(UVec2::default(), |acc, line| {
        let x_max = max(line.start.x, line.end.x);
        let y_max = max(line.start.y, line.end.y);
        (max(acc.x, x_max), max(acc.y, y_max)).into()
    });
    let result = (0..=max_point.y).fold(0, |count, y| {
        let new_count = (0..=max_point.x)
            .filter(|x| {
                let point = (*x, y).into();
                lines.iter().filter(|l| l.in_straight_line(point)).count() >= 2
            })
            .count();
        count + new_count
    });
    println!("Part1. Result = {}", result);
}
