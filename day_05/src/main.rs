use std::cmp::{max, min};
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug, Copy, Clone, Default)]
struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl FromStr for IVec2 {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (x, y) = str
            .split_once(',')
            .and_then(|(x, y)| x.parse().ok().zip(y.parse().ok()))
            .ok_or(format!("{} doesn't have 2 valid elements", str))?;
        Ok(Self { x, y })
    }
}

impl From<(i32, i32)> for IVec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Line {
    pub start: IVec2,
    pub end: IVec2,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (start, end) = str
            .split_once(" -> ")
            .and_then(|(x, y)| x.parse().ok().zip(y.parse().ok()))
            .ok_or(format!("{} doesn't have 2 valid elements", str))?;
        Ok(Self { start, end })
    }
}

impl Line {
    /// In vertical or horizontal line
    fn in_straight_line(&self, point: IVec2) -> bool {
        if self.start.x == self.end.x {
            point.x == self.start.x
                && (min(self.start.y, self.end.y)..=max(self.start.y, self.end.y))
                    .contains(&point.y)
        } else if self.start.y == self.end.y {
            point.y == self.start.y
                && (min(self.start.x, self.end.x)..=max(self.start.x, self.end.x))
                    .contains(&point.x)
        } else {
            false
        }
    }

    /// In vertical or horizontal line
    fn in_line(&self, point: IVec2) -> bool {
        point.y >= min(self.start.y, self.end.y)
            && point.y <= max(self.start.y, self.end.y)
            && point.x >= min(self.start.x, self.end.x)
            && point.x <= max(self.start.x, self.end.x)
            && ((self.end.x - self.start.x) * (point.y - self.start.y)
                == (point.x - self.start.x) * (self.end.y - self.start.y))
    }
}

fn get_result(max_point: IVec2, lines: &[Line], filter: impl Fn(&Line, IVec2) -> bool) -> usize {
    (0..=max_point.y).fold(0, |count, y| {
        let new_count = (0..=max_point.x)
            .filter(|x| {
                let point = (*x, y).into();
                lines.iter().filter(|l| filter(l, point)).count() >= 2
            })
            .count();
        count + new_count
    })
}

fn main() {
    let lines: Vec<Line> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect();
    let max_point = lines.iter().fold(IVec2::default(), |acc, line| {
        let x_max = max(line.start.x, line.end.x);
        let y_max = max(line.start.y, line.end.y);
        (max(acc.x, x_max), max(acc.y, y_max)).into()
    });
    println!(
        "Part1. Result = {}",
        get_result(max_point, &lines, Line::in_straight_line)
    );
    println!(
        "Part2. Result = {}",
        get_result(max_point, &lines, Line::in_line)
    );
}
