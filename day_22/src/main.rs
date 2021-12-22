use glam::IVec3;
use regex::{Captures, Regex};
use std::str::FromStr;

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)"#).unwrap();
}
const FILE_PATH: &str = "input.txt";

fn get_capture(captures: &Captures, index: usize) -> Result<i32, String> {
    captures
        .get(index)
        .and_then(|v| v.as_str().parse().ok())
        .ok_or_else(|| String::from("Invalid value"))
}

#[derive(Debug, Copy, Clone)]
struct Bounds {
    min: IVec3,
    max: IVec3,
}

#[derive(Debug, Clone)]
struct Volume {
    bounds: Bounds,
    is_on: bool,
}

impl Bounds {
    fn volume(&self) -> i64 {
        (i64::from(self.max.x) - i64::from(self.min.x) + 1)
            * (i64::from(self.max.y) - i64::from(self.min.y) + 1)
            * (i64::from(self.max.z) - i64::from(self.min.z) + 1)
    }

    fn intersection_with(&self, other: &Self) -> Option<Self> {
        let (x_min, x_max) = (self.min.x.max(other.min.x), self.max.x.min(other.max.x));
        let (y_min, y_max) = (self.min.y.max(other.min.y), self.max.y.min(other.max.y));
        let (z_min, z_max) = (self.min.z.max(other.min.z), self.max.z.min(other.max.z));
        if x_min <= x_max && y_min <= y_max && z_min <= z_max {
            Some(Self {
                min: IVec3::new(x_min, y_min, z_min),
                max: IVec3::new(x_max, y_max, z_max),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct EngineOperation(Vec<(Bounds, bool)>);

impl FromStr for EngineOperation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Result<Vec<(Bounds, bool)>, String> = s
            .lines()
            .map(|line| {
                let captures = REGEX.captures(line).ok_or(format!("Invalid str {}", s))?;
                let operation = matches!(captures.get(1).map(|m| m.as_str()), Some("on"));
                let x_min = get_capture(&captures, 2)?;
                let x_max = get_capture(&captures, 3)?;
                let y_min = get_capture(&captures, 4)?;
                let y_max = get_capture(&captures, 5)?;
                let z_min = get_capture(&captures, 6)?;
                let z_max = get_capture(&captures, 7)?;
                Ok((
                    Bounds {
                        min: IVec3::new(x_min, y_min, z_min),
                        max: IVec3::new(x_max, y_max, z_max),
                    },
                    operation,
                ))
            })
            .collect();
        Ok(Self(map?))
    }
}

fn find_volume(operations: &[(Bounds, bool)], stop_at_50: bool) -> i64 {
    let mut volumes: Vec<Volume> = Vec::new();
    for (bounds, state) in operations {
        if stop_at_50
            && (bounds.min.x < -50
                || bounds.min.y < -50
                || bounds.min.z < -50
                || bounds.max.x > 50
                || bounds.max.y > 50
                || bounds.max.z > 50)
        {
            continue;
        }
        let mut add = Vec::new();
        if *state {
            add.push(Volume {
                bounds: *bounds,
                is_on: true,
            });
        }
        add.extend(volumes.iter().filter_map(|v| {
            v.bounds.intersection_with(bounds).map(|bounds| Volume {
                bounds,
                is_on: !v.is_on,
            })
        }));
        volumes.extend(add);
    }
    volumes
        .iter()
        .map(|v| {
            let sign = if v.is_on { 1 } else { -1 };
            sign * (v.bounds.volume() as i64)
        })
        .sum::<i64>()
}

fn main() {
    let operations: EngineOperation =
        EngineOperation::from_str(std::fs::read_to_string(FILE_PATH).unwrap().as_str()).unwrap();
    let volume = find_volume(&operations.0, true);
    println!("Part 1 = {}", volume);
    let volume = find_volume(&operations.0, false);
    println!("Part 2 = {}", volume);
}
