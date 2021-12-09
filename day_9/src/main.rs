use std::str::FromStr;

const FILE_PATH: &str = "input.txt";
const NEIGHBOR_COORDS: &[Coords; 2] = &[
    (1, 0), // RIGHT
    (0, 1), // TOP
];

type Coords = (usize, usize);

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
    fn get_neighbors_at(&self, (x, y): Coords) -> Vec<(Coords, u8)> {
        [usize::checked_add, usize::checked_sub]
            .iter()
            .flat_map(|op| {
                NEIGHBOR_COORDS
                    .iter()
                    .filter_map(|(dx, dy)| {
                        if let (Some(y), Some(x)) = (op(y, *dy), op(x, *dx)) {
                            self.0
                                .get(y)
                                .and_then(|l| l.get(x).copied())
                                .map(|d| ((x, y), d))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Coords, u8)>>()
            })
            .collect()
    }

    fn low_points(&self) -> Vec<(Coords, u8)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(x, digit)| {
                        self.get_neighbors_at((*x, y))
                            .iter()
                            .all(|(_coords, n)| n > digit)
                    })
                    .map(|(x, digit)| ((x, y), *digit))
                    .collect::<Vec<(Coords, u8)>>()
            })
            .collect()
    }

    fn risk_level(&self) -> u32 {
        self.low_points()
            .iter()
            .map(|(_coords, digit)| u32::from(*digit) + 1)
            .sum()
    }

    fn basin_at(&self, coords: Coords, basin: &mut Vec<Coords>) {
        if basin.contains(&coords) {
            return;
        }
        basin.push(coords);
        for (coords, _d) in self
            .get_neighbors_at(coords)
            .into_iter()
            .filter(|(_coords, d)| *d < 9)
        {
            self.basin_at(coords, basin);
        }
    }

    fn basin_sizes(&self) -> usize {
        let mut sizes: Vec<usize> = self
            .low_points()
            .iter()
            .map(|(coord, _d)| {
                let mut basin = vec![];
                self.basin_at(*coord, &mut basin);
                basin.len()
            })
            .collect();
        sizes.sort_unstable();
        (0..3).filter_map(|_| sizes.pop()).product()
    }
}

fn main() {
    let height_map = HeightMap::from_str(&std::fs::read_to_string(FILE_PATH).unwrap()).unwrap();
    println!("Part1: risk level = {}", height_map.risk_level());
    println!("Part2: basin sizes = {}", height_map.basin_sizes());
}
