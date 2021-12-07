const FILE_PATH: &str = "input.txt";

fn find_best_cost(positions: &[i32], cost_fn: impl Fn(i32) -> i32) -> Option<i32> {
    (*positions.iter().min()?..*positions.iter().max()?)
        .map(|pos| {
            positions
                .iter()
                .fold(0, |cost, p| cost + cost_fn((pos - p).abs()))
        })
        .min()
}

fn main() {
    let positions: Vec<i32> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!(
        "Part1. Best cost = {}",
        find_best_cost(&positions, |len| len).unwrap()
    );
    println!(
        "Part2. Best cost = {}",
        find_best_cost(&positions, |len| (len * (len + 1)) / 2).unwrap()
    );
}
