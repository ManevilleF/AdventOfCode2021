#![allow(clippy::cast_possible_wrap)]
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

type Pixel = [isize; 2];

const FILE_PATH: &str = "input.txt";
const NEIGHBOR_COORDS: [Pixel; 9] = [
    [-1, -1], //BOTTOM LEFT
    [0, -1],  // BOTTOM
    [1, -1],  // BOTTOM RIGHT
    [-1, 0],  // LEFT
    [0, 0],   // CENTER
    [1, 0],   // RIGHT
    [-1, 1],  // TOP LEFT
    [0, 1],   // TOP
    [1, 1],   // TOP RIGHT
];

#[derive(Debug, Clone)]
struct Image(HashSet<Pixel>);

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let set = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(x, c)| (c == '#').then(|| [x as isize, y as isize]))
                    .collect::<HashSet<Pixel>>()
            })
            .collect();
        Ok(Self(set))
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let [x_min, y_min] = [
            self.x_min().ok_or(fmt::Error)? - 4,
            self.y_min().ok_or(fmt::Error)? - 4,
        ];
        let [x_max, y_max] = [
            self.x_max().ok_or(fmt::Error)? + 4,
            self.y_max().ok_or(fmt::Error)? + 4,
        ];
        let buff: Vec<String> = (y_min..=y_max)
            .map(|y| {
                (x_min..=x_max)
                    .map(|x| {
                        if self.0.get(&[x, y]).is_some() {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect();
        write!(f, "{}", buff.join("\n"))
    }
}

impl Image {
    fn x_min(&self) -> Option<isize> {
        self.0.iter().min_by_key(|[x, _]| *x).map(|[x, _]| *x)
    }
    fn x_max(&self) -> Option<isize> {
        self.0.iter().max_by_key(|[x, _]| *x).map(|[x, _]| *x)
    }
    fn y_min(&self) -> Option<isize> {
        self.0.iter().min_by_key(|[_, y]| *y).map(|[_, y]| *y)
    }
    fn y_max(&self) -> Option<isize> {
        self.0.iter().max_by_key(|[_, y]| *y).map(|[_, y]| *y)
    }

    fn pixel_data(&self, [x, y]: Pixel) -> u16 {
        let bits: String = NEIGHBOR_COORDS
            .iter()
            .map(|[dx, dy]| {
                let coord = [x + dx, y + dy];
                if self.0.get(&coord).is_some() {
                    '1'
                } else {
                    '0'
                }
            })
            .collect();
        u16::from_str_radix(&bits, 2).unwrap()
    }

    fn compute_image(&self, algorithm: &[bool; 512]) -> Self {
        let [x_max, y_max] = [self.x_max().unwrap() + 1, self.y_max().unwrap() + 1];
        let [x_min, y_min] = [self.x_min().unwrap() - 1, self.y_min().unwrap() - 1];
        let set = (y_min..=y_max)
            .flat_map(|y| {
                (x_min..=x_max)
                    .filter_map(|x| {
                        let coord = [x, y];
                        let data = self.pixel_data(coord);
                        algorithm[data as usize].then(|| coord)
                    })
                    .collect::<HashSet<Pixel>>()
            })
            .collect();
        Self(set)
    }
}

fn main() {
    let (algo, input) = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split_once("\n\n")
        .map(|(algo, input)| {
            let algo: [bool; 512] = algo
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<bool>>()
                .try_into()
                .unwrap();
            let input = Image::from_str(input).unwrap();
            (algo, input)
        })
        .unwrap();
    println!("{}", input);
    let new_image = input.compute_image(&algo);
    println!("{}", new_image);
    let new_image = new_image.compute_image(&algo);
    println!("{}", new_image);
    println!("Part 1: {} lit pixels", new_image.0.len());
}
