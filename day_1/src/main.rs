const FILE_PATH: &str = "input.txt";

fn get_increasing_count(values: &[u32], window_size: usize) -> usize {
    let values: Vec<u32> = values
        .windows(window_size)
        .map(|w| w.iter().sum())
        .collect();
    values
        .iter()
        .enumerate()
        .filter(|(i, v)| {
            v > &i
                .checked_sub(1)
                .and_then(|prev| values.get(prev))
                .unwrap_or(v)
        })
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
