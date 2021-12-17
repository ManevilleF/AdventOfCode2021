use glam::IVec2;
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";
const GRAVITY: IVec2 = IVec2::Y;

#[derive(Debug)]
struct Bounds {
    min: IVec2,
    max: IVec2,
}

impl FromStr for Bounds {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str = s
            .strip_prefix("target area: ")
            .ok_or(format!("Invalid str {}", s))?;
        let (min, max) = str
            .split_once(',')
            .and_then(|(x, y)| {
                x.strip_prefix("x=").and_then(|v| {
                    v.split_once("..")
                        .and_then(|(x_min, x_max)| x_min.parse().ok().zip(x_max.parse().ok()))
                        .zip(y.trim().strip_prefix("y=").and_then(|v| {
                            v.split_once("..").and_then(|(y_min, y_max)| {
                                y_min.parse().ok().zip(y_max.parse().ok())
                            })
                        }))
                })
            })
            .map(|((x_min, x_max), (y_min, y_max))| {
                (IVec2::new(x_min, y_min), IVec2::new(x_max, y_max))
            })
            .ok_or(format!("Invalid str: {}", str))?;
        Ok(Self { min, max })
    }
}

impl Bounds {
    fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= self.min.x && pos.y >= self.min.y && pos.x <= self.max.x && pos.y <= self.max.y
    }

    fn out_of_bounds(&self, pos: IVec2) -> bool {
        pos.x > self.max.x || pos.y < self.min.y
    }
}

fn max_y_with_velocity(mut velocity: IVec2, bounds: &Bounds) -> Option<i32> {
    let mut pos = IVec2::ZERO;
    let mut max_y = None;
    while !bounds.in_bounds(pos) {
        if bounds.out_of_bounds(pos) {
            return None;
        }
        pos += velocity;
        if pos.y > max_y.unwrap_or(i32::MIN) {
            max_y = Some(pos.y);
        }
        velocity -= GRAVITY; // Gravity
        velocity.x -= velocity.x.signum(); // Drag
    }
    max_y
}

fn compute_velocities(bounds: &Bounds) -> (Vec<IVec2>, Option<i32>) {
    let mut max_y_pos = None;
    let velocities = (bounds.min.y..=bounds.max.x)
        .rev()
        .flat_map(|y| {
            (1..=bounds.max.x)
                .filter_map(|x| {
                    let velocity = IVec2::new(x, y);
                    let pos = max_y_with_velocity(velocity, bounds)?;
                    if max_y_pos.is_none() {
                        max_y_pos = Some(pos);
                    }
                    Some(velocity)
                })
                .collect::<Vec<IVec2>>()
        })
        .collect();
    (velocities, max_y_pos)
}

fn main() {
    let bounds = Bounds::from_str(std::fs::read_to_string(FILE_PATH).unwrap().as_str()).unwrap();
    let (velocities, max_y) = compute_velocities(&bounds);
    println!("Part 1: Max Y pos is {}", max_y.unwrap());
    println!("Part 2: There are {} valid velocities", velocities.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocities() {
        let bounds = Bounds {
            min: IVec2::new(20, -10),
            max: IVec2::new(30, -5),
        };
        assert!(max_y_with_velocity(IVec2::new(7, 2), &bounds).is_some());
        assert!(max_y_with_velocity(IVec2::new(6, 3), &bounds).is_some());
        assert!(max_y_with_velocity(IVec2::new(9, 0), &bounds).is_some());
        assert!(max_y_with_velocity(IVec2::new(17, -4), &bounds).is_none());
        assert_eq!(max_y_with_velocity(IVec2::new(6, 9), &bounds), Some(45));
    }

    #[test]
    fn test_valid_velocities() {
        let bounds = Bounds {
            min: IVec2::new(20, -10),
            max: IVec2::new(30, -5),
        };
        let (res, max) = compute_velocities(&bounds);
        assert_eq!(res.len(), 112);
        assert_eq!(max, Some(45));
    }
}
