const FILE_PATH: &str = "input.txt";

fn get_increasing_count(values: &[u32]) -> usize {
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
        .split('\n')
        .filter_map(|str| str.parse().ok())
        .collect();

    println!("Part 1: {}", get_increasing_count(&file_content));

    let triple_sums: Vec<u32> = (0..file_content.len().saturating_sub(2))
        .map(|i| {
            (0..=2)
                .filter_map(|delta| file_content.get(i + delta))
                .sum::<u32>()
        })
        .collect();
    println!("Part 2: {}", get_increasing_count(&triple_sums));
}
