const FILE_PATH: &str = "input.txt";

fn get_increasing_count(values: &[u32], delta: usize) -> usize {
    values
        .iter()
        .enumerate()
        .filter(|(i, v)| *v < values.get(i + delta).unwrap_or(v))
        .count()
}

fn main() {
    let file_content: Vec<u32> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|str| str.parse().unwrap())
        .collect();
    println!("Part 1: {}", get_increasing_count(&file_content, 1));
    println!("Part 2: {}", get_increasing_count(&file_content, 3));
}
