use std::collections::HashMap;
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

type Coords = [usize; 2];

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u8>>,
    max_coords: Coords,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .and_then(|d| d.try_into().ok())
                            .ok_or(format!("Invalid line: {}", line))
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<u8>>, Self::Err>>()?;
        let y = map
            .len()
            .checked_sub(1)
            .ok_or_else(|| String::from("Map is empty"))?;
        let x = map
            .last()
            .and_then(|l| l.len().checked_sub(1))
            .ok_or_else(|| String::from("Map is empty"))?;
        Ok(Self {
            max_coords: [x, y],
            map,
        })
    }
}

impl Map {
    fn neighbor_coordinates(&self, [x, y]: Coords) -> Vec<(Coords, u8)> {
        [usize::checked_add, usize::checked_sub]
            .iter()
            .flat_map(|op| {
                [(1, 0), (0, 1)]
                    .iter()
                    .filter_map(|(dx, dy)| {
                        let [x, y] = [op(x, *dx)?, op(y, *dy)?];
                        let cost = self.map.get(y)?.get(x)?;
                        Some(([x, y], *cost))
                    })
                    .collect::<Vec<(Coords, u8)>>()
            })
            .collect()
    }

    fn pop_from_stack(stack: &mut HashMap<Coords, usize>) -> Option<(Coords, usize)> {
        let (coord, cost) = stack
            .iter()
            .min_by_key(|(_c, v)| *v)
            .map(|(c, v)| (*c, *v))?;
        stack.remove(&coord);
        Some((coord, cost))
    }

    fn find_cheapest_path(&self) -> Option<usize> {
        let mut stack = HashMap::new();
        stack.insert([0, 0], 0);
        let mut handled = vec![];
        let mut results = Vec::new();
        while let Some((coord, cost)) = Self::pop_from_stack(&mut stack) {
            if coord == self.max_coords {
                results.push(cost);
                continue;
            }
            for (neighbor, new_cost) in self
                .neighbor_coordinates(coord)
                .into_iter()
                .filter(|(c, _)| !handled.contains(c))
            {
                let new_cost = cost + new_cost as usize;
                let entry = stack.entry(neighbor).or_insert(new_cost);
                if *entry > new_cost {
                    *entry = new_cost;
                }
            }
            handled.push(coord);
        }
        results.into_iter().min()
    }
}

fn incremented_map(map: &[Vec<u8>], delta: u8) -> Vec<Vec<u8>> {
    map.iter().map(|vec| incremented_vec(vec, delta)).collect()
}

fn incremented_vec(vec: &[u8], delta: u8) -> Vec<u8> {
    vec.iter()
        .map(|v| {
            let value = v + delta;
            if value > 9 {
                1
            } else {
                value
            }
        })
        .collect()
}

fn main() {
    let map = Map::from_str(std::fs::read_to_string(FILE_PATH).unwrap().as_str()).unwrap();
    println!("Part 1: {}", map.find_cheapest_path().unwrap());

    let mut prev = map.map;
    let new_map: Vec<Vec<u8>> = (0..5)
        .map(|y| {
            if y > 0 {
                prev = incremented_map(&prev, 1);
            }
            prev.clone()
        })
        .flat_map(|map| {
            map.iter()
                .map(|vec| {
                    let mut prev_vec = vec.clone();
                    (1..5).fold(prev_vec.clone(), |mut acc, _x| {
                        let new_vec: Vec<u8> = incremented_vec(&prev_vec, 1);
                        acc.extend(new_vec.clone());
                        prev_vec = new_vec;
                        acc
                    })
                })
                .collect::<Vec<Vec<u8>>>()
        })
        .collect();
    let map = Map {
        max_coords: [new_map[0].len() - 1, new_map.len() - 1],
        map: new_map,
    };
    println!("Part 2: {}", map.find_cheapest_path().unwrap());
}
