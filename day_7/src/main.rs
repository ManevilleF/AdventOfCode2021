const FILE_PATH: &str = "input.txt";

fn find_best_cost(positions: &[i32]) -> Option<i32> {
    let min_pos = *positions.iter().min()?;
    let max_pos = *positions.iter().max()?;
    let costs =
        (min_pos..max_pos).map(|pos| positions.iter().fold(0, |cost, p| cost + (pos - p).abs()));
    costs.min()
}

fn main() {
    let positions: Vec<i32> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Best cost = {}", find_best_cost(&positions).unwrap());
}
