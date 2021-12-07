const FILE_PATH: &str = "input.txt";

fn find_best_cost(positions: &[i32], func: impl Fn(i32, i32) -> i32) -> Option<i32> {
    let min_pos = *positions.iter().min()?;
    let max_pos = *positions.iter().max()?;
    let costs =
        (min_pos..max_pos).map(|pos| positions.iter().fold(0, |cost, p| cost + func(pos, *p)));
    costs.min()
}

fn main() {
    let positions: Vec<i32> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!(
        "Part1. Best cost = {}",
        find_best_cost(&positions, |pos, p| (pos - p).abs()).unwrap()
    );
    println!(
        "Part2. Best cost = {}",
        find_best_cost(&positions, |pos, p| {
            let v = (pos - p).abs();
            (v * (v + 1)) / 2
        })
        .unwrap()
    );
}
