use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

const NEIGHBOR_COORDS: &[(usize, usize); 2] = &[
    (1, 0), // RIGHT
    (0, 1), // TOP
];

#[derive(Debug, Clone)]
struct HeightMap(Vec<Vec<u8>>);

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
                    .collect()
            })
            .collect();
        Ok(Self(map))
    }
}

impl HeightMap {
    fn get_neighbors_at(&self, (x, y): (usize, usize)) -> Vec<u8> {
        [usize::checked_add, usize::checked_sub]
            .iter()
            .flat_map(|op| {
                NEIGHBOR_COORDS
                    .iter()
                    .filter_map(|(dx, dy)| {
                        if let (Some(y), Some(x)) = (op(y, *dy), op(x, *dx)) {
                            self.0.get(y).and_then(|l| l.get(x).copied())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect()
    }

    fn risk_level(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(x, digit)| self.get_neighbors_at((*x, y)).iter().all(|n| n > digit))
                    .map(|(_x, digit)| u32::from(*digit) + 1)
                    .sum::<u32>()
            })
            .sum()
    }
}

fn main() {
    let height_map = HeightMap::from_str(&std::fs::read_to_string(FILE_PATH).unwrap()).unwrap();
    println!("Part1: risk level = {}", height_map.risk_level());
}
