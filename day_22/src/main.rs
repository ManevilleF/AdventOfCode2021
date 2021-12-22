use glam::IVec3;
use regex::{Captures, Regex};
use std::collections::HashSet;
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

#[derive(Debug)]
struct BoundsOperation {
    operation: bool,
    min: IVec3,
    max: IVec3,
}

impl FromStr for BoundsOperation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = REGEX.captures(s).ok_or(format!("Invalid str {}", s))?;
        let operation = if let Some("on") = captures.get(1).map(|m| m.as_str()) {
            true
        } else {
            false
        };
        let x_min = get_capture(&captures, 2)?;
        let x_max = get_capture(&captures, 3)?;
        let y_min = get_capture(&captures, 4)?;
        let y_max = get_capture(&captures, 5)?;
        let z_min = get_capture(&captures, 6)?;
        let z_max = get_capture(&captures, 7)?;
        Ok(Self {
            operation,
            min: IVec3::new(x_min, y_min, z_min),
            max: IVec3::new(x_max, y_max, z_max),
        })
    }
}

impl BoundsOperation {
    fn all_coords(&self) -> Vec<IVec3> {
        (self.min.x..=self.max.x)
            .flat_map(|x| {
                (self.min.y..=self.max.y)
                    .flat_map(|y| {
                        (self.min.z..=self.max.z)
                            .map(|z| IVec3::new(x, y, z))
                            .collect::<Vec<IVec3>>()
                    })
                    .collect::<Vec<IVec3>>()
            })
            .collect()
    }
}

fn fill_set(operations: &[BoundsOperation], stop_at_50: bool) -> HashSet<IVec3> {
    let mut map = HashSet::new();
    for operation in operations {
        if stop_at_50
            && (operation.min.x < -50
                || operation.min.y < -50
                || operation.min.z < -50
                || operation.max.x > 50
                || operation.max.y > 50
                || operation.max.z > 50)
        {
            continue;
        }
        let coords = operation.all_coords();
        if operation.operation {
            map.extend(coords);
        } else {
            for coord in coords {
                map.remove(&coord);
            }
        }
    }
    map
}

fn main() {
    let operations: Vec<BoundsOperation> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|l| BoundsOperation::from_str(l).unwrap())
        .collect();
    println!("Part 1: count = {}", fill_set(&operations, true).len());
}
