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
        let arr: Vec<i32> = str.split(',').filter_map(|s| s.parse().ok()).collect();
        let arr: [i32; 2] = arr
            .try_into()
            .map_err(|_| format!("{} doesn't have 2 valid elements", str))?;
        Ok(Self {
            x: arr[0],
            y: arr[1],
        })
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
        let arr: Vec<IVec2> = str.split(" -> ").filter_map(|s| s.parse().ok()).collect();
        let arr: [IVec2; 2] = arr
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
        .filter_map(|s| s.parse().ok())
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
