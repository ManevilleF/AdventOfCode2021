const FILE_PATH: &str = "input.txt";

type Coords = (usize, usize);

#[derive(Debug, Clone)]
struct HeightMap(Vec<Vec<u8>>);

impl From<String> for HeightMap {
    fn from(s: String) -> Self {
        Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
                        .collect()
                })
                .collect(),
        )
    }
}

impl HeightMap {
    fn neighbors_at(&self, (x, y): Coords) -> Vec<(Coords, u8)> {
        [usize::checked_add, usize::checked_sub]
            .iter()
            .flat_map(|op| {
                [(1, 0), (0, 1)]
                    .iter()
                    .filter_map(|(dx, dy)| {
                        let (x, y) = (op(x, *dx)?, op(y, *dy)?);
                        let digit = self.0.get(y)?.get(x)?;
                        Some(((x, y), *digit))
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
                        self.neighbors_at((*x, y))
                            .iter()
                            .all(|(_coords, n)| n > digit)
                    })
                    .map(|(x, digit)| ((x, y), *digit))
                    .collect::<Vec<(Coords, u8)>>()
            })
            .collect()
    }

    fn basin_at(&self, coords: Coords, basin: &mut Vec<Coords>) {
        if basin.contains(&coords) {
            return;
        }
        basin.push(coords);
        self.neighbors_at(coords)
            .into_iter()
            .filter(|(_coords, d)| *d < 9)
            .for_each(|(coords, _d)| self.basin_at(coords, basin));
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
    let map = HeightMap::from(std::fs::read_to_string(FILE_PATH).unwrap());
    println!(
        "Part1: risk level = {}",
        map.low_points()
            .iter()
            .map(|(_, digit)| u32::from(*digit) + 1)
            .sum::<u32>()
    );
    println!("Part2: basin sizes = {}", map.basin_sizes());
}
