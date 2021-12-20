use glam::IVec3;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const FILE_PATH: &str = "test.txt";

#[derive(Debug, Clone)]
struct ScannerData {
    id: usize,
    beacons: Vec<IVec3>,
}

#[derive(Debug, Clone)]
struct ScannerMatch {
    match_id: usize,
    delta: IVec3,
    new_scanner: ScannerData,
}

impl FromStr for ScannerData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let title = lines
            .next()
            .ok_or_else(|| "Scanner data is too short".to_string())?
            .replace('-', "")
            .replace("scanner", "");
        let id = title
            .trim()
            .parse()
            .map_err(|e| format!("Failed to parse id in {}: {}", title, e))?;
        let beacons: Result<Vec<IVec3>, Self::Err> = lines
            .map(|l| {
                let coord: Result<Vec<i32>, Self::Err> = l
                    .split(',')
                    .map(|s| {
                        s.parse()
                            .map_err(|e| format!("Failed to parse coordinate {}", e))
                    })
                    .collect();
                let coords: [i32; 3] = coord?
                    .try_into()
                    .map_err(|e| format!("Failed to retrieve coords fro {:?}", e))?;
                Ok(IVec3::from(coords))
            })
            .collect();
        Ok(Self {
            id,
            beacons: beacons?,
        })
    }
}

impl Display for ScannerData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.beacons.len())
    }
}

impl ScannerData {
    fn rotate_x(mut coord: IVec3, times: usize) -> IVec3 {
        for _ in 0..times {
            coord = IVec3::new(coord.x, -coord.z, coord.y)
        }
        coord
    }

    fn rotate_y(mut coord: IVec3, times: usize) -> IVec3 {
        for _ in 0..times {
            coord = IVec3::new(-coord.z, coord.y, coord.x)
        }
        coord
    }

    fn rotate_z(mut coord: IVec3, times: usize) -> IVec3 {
        for _ in 0..times {
            coord = IVec3::new(coord.y, -coord.x, coord.z)
        }
        coord
    }

    fn rotated_candidates(&self) -> Vec<Self> {
        (0..=1)
            .flat_map(|x| {
                (0..=3)
                    .flat_map(|y| {
                        (0..=3)
                            .map(|z| Self {
                                id: self.id,
                                beacons: self
                                    .beacons
                                    .iter()
                                    .map(|c| {
                                        Self::rotate_z(Self::rotate_y(Self::rotate_x(*c, x), y), z)
                                    })
                                    .collect(),
                            })
                            .collect::<Vec<Self>>()
                    })
                    .collect::<Vec<Self>>()
            })
            .collect()
    }

    fn find_delta(&self, other: &Self) -> Option<IVec3> {
        let mut res_map = HashMap::new();
        for c1 in &self.beacons {
            for c2 in &other.beacons {
                let delta = *c2 - *c1;
                *res_map.entry(delta).or_insert(0) += 1;
            }
        }
        let (delta, max) = res_map.into_iter().max_by_key(|(_k, v)| *v)?;
        if max >= 12 {
            return Some(delta);
        }
        None
    }

    fn translate(&mut self, delta: IVec3) {
        for coord in self.beacons.iter_mut() {
            *coord += delta;
        }
    }

    fn find_match(&self, other: &Self) -> Option<ScannerMatch> {
        for (i, mut candidate) in self.rotated_candidates().into_iter().enumerate() {
            if let Some(delta) = candidate.find_delta(other) {
                println!("Found match with {} rotation", i);
                candidate.translate(delta);
                return Some(ScannerMatch {
                    match_id: other.id,
                    delta,
                    new_scanner: candidate,
                });
            }
        }
        None
    }
}

fn main() {
    let mut scanners: Vec<ScannerData> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split("\n\n")
        .map(|s| ScannerData::from_str(s).unwrap())
        .collect();
    let mut map = HashMap::new();
    for scanner in &scanners {
        for other in &scanners {
            if scanner.id == other.id {
                continue;
            }
            if let Some(matched) = scanner.find_match(other) {
                println!(
                    "-- Matched {} with {}: {:?}",
                    scanner.id, matched.match_id, matched.delta
                );
                map.insert(scanner.id, matched);
            }
        }
    }
    println!("{}", map.len());
}
