use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug)]
struct SquidGrid {
    energy_levels: [[u8; 10]; 10],
    flashes_count: usize,
    flashed: Vec<(usize, usize)>,
}

impl FromStr for SquidGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let energy_levels: Result<Vec<[u8; 10]>, Self::Err> =
            s.lines().try_fold(vec![], |mut acc, l| {
                let vec: Vec<u8> = l
                    .chars()
                    .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
                    .collect();
                acc.push(
                    vec.try_into()
                        .map_err(|e| format!("Expected 10 elements: {:?}", e))?,
                );
                Ok(acc)
            });
        let energy_levels = energy_levels?
            .try_into()
            .map_err(|e| format!("Expected 10 elements: {:?}", e))?;
        Ok(Self {
            energy_levels,
            flashes_count: 0,
            flashed: vec![],
        })
    }
}

impl SquidGrid {
    const NEIGHBOR_COORDS: &'static [(i8, i8); 8] = &[
        (-1, 0),  // LEFT
        (-1, 1),  // TOP LEFT
        (0, 1),   // TOP
        (1, 1),   // TOP RIGHT
        (1, 0),   // RIGHT
        (1, -1),  // BOTTOM RIGHT
        (0, -1),  // BOTTOM
        (-1, -1), // BOTTOM LEFT
    ];

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn neighbor_coords((x, y): (usize, usize)) -> Vec<(usize, usize)> {
        Self::NEIGHBOR_COORDS
            .iter()
            .filter_map(|(dx, dy)| {
                let x = (x as isize) + *dx as isize;
                let y = (y as isize) + *dy as isize;
                (x >= 0 && y >= 0).then(|| (x as usize, y as usize))
            })
            .collect()
    }

    fn handle_step_on_coord(&mut self, (x, y): (usize, usize)) {
        if self.flashed.contains(&(x, y)) {
            return;
        }
        if let Some(level) = self.energy_levels.get_mut(y).and_then(|l| l.get_mut(x)) {
            *level += 1;
            if *level > 9 {
                *level = 0;
                self.flashed.push((x, y));
                Self::neighbor_coords((x, y))
                    .iter()
                    .for_each(|c| self.handle_step_on_coord(*c));
                self.flashes_count += 1;
            }
        }
    }

    fn handle_step(&mut self) {
        self.flashed.clear();
        for y in 0..10 {
            for x in 0..10 {
                self.handle_step_on_coord((x, y));
            }
        }
    }

    fn is_synced(&self) -> bool {
        self.energy_levels
            .iter()
            .all(|l| l.iter().all(|level| *level == 0))
    }
}

fn main() {
    let mut grid = SquidGrid::from_str(&std::fs::read_to_string(FILE_PATH).unwrap()).unwrap();
    (0..100).for_each(|_| {
        grid.handle_step();
    });
    println!("Part 1. Total = {}", grid.flashes_count);
    let mut step = 100;
    while !grid.is_synced() {
        grid.handle_step();
        step += 1;
    }
    println!("Part 2. Sync at {}", step);
}
